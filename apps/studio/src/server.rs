//! Local HTTP server (127.0.0.1 only): the embedded UI and its JSON API. Long work (crafts,
//! builds) runs on blocking threads; builds are jobs the UI polls for log lines. Agents building
//! over MCP (other processes) post what they do to /api/activity; the UI follows it over SSE.

use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Path as AxPath, Query, Request};
use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use capcut_core::crafts::{effect, vector};
use capcut_core::paths::{app_data_dir, capcut_root, clean_path_str, slug};
use capcut_core::settings::Settings;
use capcut_core::spec::GraphicSpec;
use capcut_core::{build, library, license, mcp, media, ProjectSpec};
use futures_util::{Stream, StreamExt};
use serde_json::{json, Value};
use std::collections::{HashMap, VecDeque};
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use tokio::sync::broadcast;
use tower::ServiceExt;
use tower_http::services::ServeFile;

#[derive(rust_embed::RustEmbed)]
#[folder = "ui"]
struct Ui;

type Q = Query<HashMap<String, String>>;

#[derive(Default)]
struct Job {
    state: &'static str,
    log: Vec<String>,
    result: Option<Value>,
    error: Option<String>,
}

fn jobs() -> &'static Mutex<HashMap<String, Arc<Mutex<Job>>>> {
    static J: OnceLock<Mutex<HashMap<String, Arc<Mutex<Job>>>>> = OnceLock::new();
    J.get_or_init(Default::default)
}

/// Blocking work off the async runtime; errors become 400 + message.
async fn run<F>(f: F) -> Response
where
    F: FnOnce() -> anyhow::Result<Value> + Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(Ok(v)) => Json(v).into_response(),
        Ok(Err(e)) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{e:#}")}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "lỗi nội bộ"}))).into_response(),
    }
}

pub async fn serve(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api/ping", get(|| async { Json(json!({"ok": true, "app": "draft-studio", "version": capcut_core::VERSION})) }))
        .route("/api/status", get(|| run(|| Ok(mcp::status()))))
        .route("/api/catalog", get(|| run(|| Ok(mcp::catalog()))))
        .route("/api/drafts", get(|| run(|| Ok(json!(library::list_drafts(None)?)))))
        .route("/api/draft", get(draft_detail))
        .route("/api/repair", post(repair))
        .route("/api/specs", get(list_specs))
        .route("/api/specs/:slug", get(get_spec).put(put_spec).delete(delete_spec))
        .route("/api/settings", get(|| run(|| Ok(json!(Settings::load())))).put(put_settings))
        .route("/api/pick", post(pick))
        .route("/api/probe", get(probe))
        .route("/api/file", get(file))
        .route("/api/graphic/preview", post(graphic_preview))
        .route("/api/motion/preview", post(motion_preview))
        .route("/api/build", post(start_build))
        .route("/api/jobs/:id", get(job))
        .route("/api/open", post(open))
        .route("/api/import", post(import_spec))
        .route("/api/activity", post(post_activity).layer(DefaultBodyLimit::max(16 << 20)))
        .route("/api/activity/stream", get(activity_stream))
        .route("/api/mcp-clients", get(get_mcp_clients).post(post_mcp_clients))
        .route("/api/license", get(|| async { Json(json!(license::status())) }).post(post_license).delete(delete_license))
        .fallback(static_ui)
        .layer(middleware::from_fn(require_key));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ---------- access key ----------

/// Every /api route but ping and the key itself needs an activated machine (license.rs); the UI
/// asks for the key first.
async fn require_key(req: Request, next: Next) -> Response {
    let p = req.uri().path();
    let open = !p.starts_with("/api/") || p == "/api/ping" || p == "/api/license";
    if open || license::status().activated {
        return next.run(req).await;
    }
    (StatusCode::FORBIDDEN, Json(json!({"error": license::NOT_ACTIVATED, "license": false}))).into_response()
}

async fn post_license(Json(b): Json<Value>) -> Response {
    run(move || Ok(json!(license::activate(b["key"].as_str().unwrap_or(""))?))).await
}

async fn delete_license() -> Response {
    run(|| {
        license::deactivate()?;
        Ok(json!(license::status()))
    })
    .await
}

async fn static_ui(req: Request) -> Response {
    let p = req.uri().path().trim_start_matches('/');
    let p = if p.is_empty() { "index.html" } else { p };
    match Ui::get(p) {
        Some(f) => {
            let mime = mime_guess::from_path(p).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref().to_string()), (header::CACHE_CONTROL, "no-cache".to_string())], f.data.into_owned()).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn draft_detail(q: Q) -> Response {
    let name = q.get("name").cloned().unwrap_or_default();
    run(move || Ok(json!(library::inspect_draft(&name)?))).await
}

async fn repair(Json(b): Json<Value>) -> Response {
    run(move || Ok(json!(library::repair_draft(b["name"].as_str().unwrap_or(""))?))).await
}

// ---------- saved studio projects ----------

fn specs_dir() -> PathBuf {
    app_data_dir().join("projects")
}

async fn list_specs() -> Response {
    run(|| {
        let mut out = vec![];
        for e in std::fs::read_dir(specs_dir()).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            let Ok(raw) = std::fs::read_to_string(&p) else { continue };
            let Ok(v) = serde_json::from_str::<Value>(&raw) else { continue };
            let modified = e.metadata().and_then(|m| m.modified()).ok().map(|t| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)).unwrap_or(0);
            out.push(json!({
                "slug": p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default(),
                "name": v["project_name"],
                "scenes": v["scenes"].as_array().map(|a| a.len()).unwrap_or(0)
                    + v["sections"].as_array().into_iter().flatten().map(|x| x["scenes"].as_array().map(|a| a.len()).unwrap_or(0)).sum::<usize>(),
                "overlays": v["overlays"].as_array().map(|a| a.len()).unwrap_or(0),
                "modified": modified,
            }));
        }
        out.sort_by(|a, b| b["modified"].as_u64().cmp(&a["modified"].as_u64()));
        Ok(json!(out))
    })
    .await
}

async fn get_spec(AxPath(s): AxPath<String>) -> Response {
    run(move || {
        let p = specs_dir().join(format!("{}.json", slug(&s)));
        Ok(serde_json::from_str(&std::fs::read_to_string(&p)?)?)
    })
    .await
}

async fn put_spec(AxPath(s): AxPath<String>, Json(b): Json<Value>) -> Response {
    run(move || {
        std::fs::create_dir_all(specs_dir())?;
        let sl = slug(&s);
        std::fs::write(specs_dir().join(format!("{sl}.json")), serde_json::to_string_pretty(&b)?)?;
        Ok(json!({"slug": sl}))
    })
    .await
}

async fn delete_spec(AxPath(s): AxPath<String>) -> Response {
    run(move || {
        let p = specs_dir().join(format!("{}.json", slug(&s)));
        if p.is_file() {
            std::fs::remove_file(p)?;
        }
        Ok(json!({"ok": true}))
    })
    .await
}

async fn put_settings(Json(b): Json<Value>) -> Response {
    run(move || {
        let s: Settings = serde_json::from_value(b)?;
        s.save()?;
        Ok(json!(s))
    })
    .await
}

// ---------- files ----------

async fn pick(Json(b): Json<Value>) -> Response {
    run(move || {
        let kind = b["kind"].as_str().unwrap_or("media").to_string();
        let multiple = b["multiple"].as_bool().unwrap_or(false);
        let mut d = rfd::FileDialog::new();
        if let Some(dir) = b["dir"].as_str().filter(|d| Path::new(d).is_dir()) {
            d = d.set_directory(dir);
        }
        let images = ["jpg", "jpeg", "png", "webp", "bmp", "gif"];
        let videos = ["mp4", "mov", "mkv", "webm", "avi", "m4v"];
        let audio = ["mp3", "wav", "m4a", "aac", "flac", "ogg", "opus"];
        d = match kind.as_str() {
            "images" => d.add_filter("Ảnh", &images),
            "visual" => d.add_filter("Ảnh & video", &[&images[..], &videos[..]].concat()),
            "video" => d.add_filter("Video", &videos),
            "audio" => d.add_filter("Âm thanh", &audio),
            "json" => d.add_filter("Spec JSON", &["json"]),
            _ => d.add_filter("Media", &[&images[..], &videos[..], &audio[..]].concat()),
        };
        let to_s = |p: PathBuf| clean_path_str(&p.to_string_lossy());
        let paths: Vec<String> = match kind.as_str() {
            "folder" => rfd::FileDialog::new().pick_folder().map(to_s).into_iter().collect(),
            _ if multiple => d.pick_files().unwrap_or_default().into_iter().map(to_s).collect(),
            _ => d.pick_file().map(to_s).into_iter().collect(),
        };
        // A folder of pictures expands to its images, sorted by name.
        if kind == "folder" {
            if let Some(dir) = paths.first() {
                let mut imgs: Vec<String> = std::fs::read_dir(dir)?
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| matches!(media::kind_of(p), media::MediaKind::Image | media::MediaKind::Video))
                    .map(to_s)
                    .collect();
                imgs.sort();
                return Ok(json!({"paths": imgs, "folder": dir}));
            }
        }
        Ok(json!({"paths": paths}))
    })
    .await
}

async fn probe(q: Q) -> Response {
    let p = q.get("path").cloned().unwrap_or_default();
    run(move || Ok(json!({"kind": media::kind_of(&p), "info": media::probe(&p), "exists": Path::new(&p).is_file()}))).await
}

/// Stream a local media file (with HTTP ranges, so video and audio can seek).
async fn file(q: Q, req: Request) -> Response {
    let p = PathBuf::from(q.get("path").cloned().unwrap_or_default());
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let allowed = matches!(media::kind_of(&p), media::MediaKind::Image | media::MediaKind::Video | media::MediaKind::Audio) || ext == "svg";
    if !allowed || !p.is_file() {
        return StatusCode::NOT_FOUND.into_response();
    }
    match ServeFile::new(&p).oneshot(req).await {
        Ok(r) => r.map(Body::new).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn open(Json(b): Json<Value>) -> Response {
    run(move || {
        match b["what"].as_str().unwrap_or("") {
            "capcut" => {
                let exe = capcut_root().ancestors().nth(3).map(|p| p.join("Apps").join("CapCut.exe")).filter(|p| p.is_file());
                let exe = exe.ok_or_else(|| anyhow::anyhow!("không tìm thấy CapCut.exe"))?;
                std::process::Command::new(exe).spawn()?;
            }
            "folder" => {
                let p = PathBuf::from(b["path"].as_str().unwrap_or(""));
                let target = if p.is_file() { p.parent().map(Path::to_path_buf).unwrap_or(p) } else { p };
                if !target.exists() {
                    anyhow::bail!("không có thư mục {}", target.display());
                }
                std::process::Command::new("explorer").arg(target.to_string_lossy().replace('/', "\\")).spawn()?;
            }
            other => anyhow::bail!("không mở được '{other}'"),
        }
        Ok(json!({"ok": true}))
    })
    .await
}

/// Read a spec JSON from disk (v1 or v2 fields) and hand it back normalised.
async fn import_spec(Json(b): Json<Value>) -> Response {
    run(move || {
        let p = b["path"].as_str().unwrap_or("");
        let raw = std::fs::read_to_string(p).map_err(|e| anyhow::anyhow!("không đọc được {p}: {e}"))?;
        let v: Value = serde_json::from_str(raw.trim_start_matches('\u{feff}'))?;
        let spec: ProjectSpec = serde_json::from_value(v.clone())?;
        let mut out = serde_json::to_value(&spec)?;
        if let Some(studio) = v.get("studio") {
            out["studio"] = studio.clone();
        }
        Ok(out)
    })
    .await
}

// ---------- crafts ----------

fn dims(b: &Value) -> (u32, u32, f64) {
    (
        b["width"].as_u64().unwrap_or(1920).clamp(16, 8192) as u32,
        b["height"].as_u64().unwrap_or(1080).clamp(16, 8192) as u32,
        b["fps"].as_f64().filter(|f| *f > 0.0).unwrap_or(30.0),
    )
}

async fn graphic_preview(Json(b): Json<Value>) -> Response {
    run(move || {
        let g: GraphicSpec = serde_json::from_value(b["graphic"].clone())?;
        let (w, h, _) = dims(&b);
        let dir = build::project_assets_dir(b["project_name"].as_str().unwrap_or("_preview")).join("graphics");
        Ok(json!(vector::render(&g, w, h, &dir)?))
    })
    .await
}

async fn motion_preview(Json(b): Json<Value>) -> Response {
    run(move || {
        let (w, h, fps) = dims(&b);
        let base = build::project_assets_dir(b["project_name"].as_str().unwrap_or("_preview"));
        let image = match (b["image"].as_str().filter(|s| !s.is_empty()), b.get("graphic").filter(|g| g.is_object())) {
            (Some(p), _) => PathBuf::from(p),
            (None, Some(g)) => {
                let g: GraphicSpec = serde_json::from_value(g.clone())?;
                PathBuf::from(vector::render(&g, w, h, &base.join("graphics"))?.png)
            }
            _ => anyhow::bail!("cần ảnh hoặc đồ họa để làm chuyển động"),
        };
        let dur = b["duration_sec"].as_f64().filter(|d| *d > 0.2).unwrap_or(4.0);
        // The browser plays WebM with alpha; the CapCut build renders the overlay's own format.
        let out = effect::render(&image, b["preset"].as_str().unwrap_or("fade"), "webm", w, h, fps, dur, &base.join("motion"))?;
        Ok(json!(out))
    })
    .await
}

// ---------- builds ----------

async fn start_build(Json(b): Json<Value>) -> Response {
    let spec: ProjectSpec = match serde_json::from_value(b["spec"].clone()) {
        Ok(s) => s,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("spec không hợp lệ: {e}")}))).into_response(),
    };
    let id = capcut_core::new_id();
    let job = Arc::new(Mutex::new(Job { state: "running", ..Default::default() }));
    if let Ok(mut j) = jobs().lock() {
        j.insert(id.clone(), job.clone());
    }
    std::thread::spawn(move || {
        let sink = job.clone();
        let log = move |s: &str| {
            if let Ok(mut j) = sink.lock() {
                j.log.push(s.to_string());
            }
        };
        let r = build::build_project(&spec, &log);
        if let Ok(mut j) = job.lock() {
            match r {
                Ok(rep) => {
                    j.result = Some(json!(rep));
                    j.state = "done";
                }
                Err(e) => {
                    j.error = Some(format!("{e:#}"));
                    j.state = "error";
                }
            }
        }
    });
    Json(json!({"job": id})).into_response()
}

async fn job(AxPath(id): AxPath<String>) -> Response {
    let j = jobs().lock().ok().and_then(|m| m.get(&id).cloned());
    match j {
        Some(j) => {
            let j = match j.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            Json(json!({"state": j.state, "log": j.log, "result": j.result, "error": j.error})).into_response()
        }
        None => (StatusCode::NOT_FOUND, Json(json!({"error": "không có job"}))).into_response(),
    }
}

// ---------- agent activity ----------
// MCP servers (capcut_core::activity, in the agent's process) post each tool call as it runs:
// start, spec, log lines, craft files, outcome. Every open UI gets them over SSE; the last
// ACTIVITY_KEEP are kept so a UI opened mid-build catches up.

const ACTIVITY_KEEP: usize = 2000;

#[derive(Default)]
struct Activity {
    next_id: u64,
    events: VecDeque<Value>,
    /// Highest `seq` taken per MCP session, so a retried batch is not shown twice.
    last_seq: HashMap<String, u64>,
    /// When each session was last heard from (ms): a running call that goes quiet is a dead agent.
    seen: HashMap<String, u64>,
}

struct Hub {
    state: Mutex<Activity>,
    tx: broadcast::Sender<Value>,
}

fn hub() -> &'static Hub {
    static H: OnceLock<Hub> = OnceLock::new();
    H.get_or_init(|| Hub { state: Mutex::new(Activity::default()), tx: broadcast::channel(4096).0 })
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

type Hook = Box<dyn Fn() + Send + Sync>;
static ON_AGENT_BUILD: OnceLock<Hook> = OnceLock::new();

/// Run when an agent starts a build while `open_studio_on_agent_build` is on (the desktop app
/// brings its window forward).
pub fn on_agent_build(f: impl Fn() + Send + Sync + 'static) {
    let _ = ON_AGENT_BUILD.set(Box::new(f));
}

/// One event or an array of them.
async fn post_activity(Json(b): Json<Value>) -> Response {
    let items = match b {
        Value::Array(a) => a,
        v => vec![v],
    };
    let (mut accepted, mut wake) = (0, false);
    {
        let h = hub();
        let mut a = h.state.lock().unwrap_or_else(|p| p.into_inner());
        let now = now_ms();
        for mut ev in items {
            let Some(session) = ev["session"].as_str().map(str::to_string) else { continue };
            a.seen.insert(session.clone(), now);
            if let Some(seq) = ev["seq"].as_u64() {
                let last = a.last_seq.entry(session).or_insert(0);
                if seq <= *last {
                    continue;
                }
                *last = seq;
            }
            a.next_id += 1;
            ev["id"] = json!(a.next_id);
            wake |= ev["type"] == "call_start" && ev["build"] == true;
            if ev["type"] == "spec" {
                if let Ok(sp) = serde_json::from_value::<ProjectSpec>(ev["spec"].clone()) {
                    let _ = build::save_studio_project(&sp);
                }
            }
            // heartbeats only refresh `seen`; the rest is kept for late UIs
            if ev["type"] != "alive" {
                a.events.push_back(ev.clone());
                if a.events.len() > ACTIVITY_KEEP {
                    a.events.pop_front();
                }
            }
            // sent under the lock, so a UI subscribing now gets each event once: kept or live
            let _ = h.tx.send(ev);
            accepted += 1;
        }
    }
    if wake && Settings::load().open_studio_on_agent_build {
        if let Some(f) = ON_AGENT_BUILD.get() {
            f();
        }
    }
    Json(json!({"ok": true, "app": "draft-studio", "accepted": accepted})).into_response()
}

fn sse_event(v: &Value) -> Event {
    Event::default().id(v["id"].to_string()).data(v.to_string())
}

/// Kept events (after `Last-Event-ID` / `?after=`), then `ready` with when each session was last
/// heard from, then live events. `lagged` tells a UI that fell behind to reconnect.
async fn activity_stream(headers: HeaderMap, q: Q) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let after = headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .or_else(|| q.get("after").and_then(|s| s.parse().ok()))
        .unwrap_or(0);
    let h = hub();
    let (backlog, ready, rx) = {
        let a = h.state.lock().unwrap_or_else(|p| p.into_inner());
        // ids restart with the studio: a UI that saw more than exists gets everything again
        let after = if after > a.next_id { 0 } else { after };
        let backlog: Vec<Event> = a.events.iter().filter(|e| e["id"].as_u64().unwrap_or(0) > after).map(sse_event).collect();
        let ready = Event::default().event("ready").data(json!({"now": now_ms(), "seen": a.seen}).to_string());
        (backlog, ready, h.tx.subscribe())
    };
    let live = futures_util::stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(v) => Some((sse_event(&v), rx)),
            Err(broadcast::error::RecvError::Lagged(_)) => Some((Event::default().event("lagged").data("{}"), rx)),
            Err(broadcast::error::RecvError::Closed) => None,
        }
    });
    let s = futures_util::stream::iter(backlog).chain(futures_util::stream::once(async move { ready })).chain(live).map(Ok);
    Sse::new(s).keep_alive(KeepAlive::default())
}

// ---------- 1-click MCP client connection ----------

fn mcp_exe_path() -> String {
    let p = capcut_core::paths::exe_dir().join("capcut_draft_app.exe");
    clean_path_str(&p.to_string_lossy()).replace('/', "\\")
}

fn mcp_client_targets() -> Vec<(&'static str, &'static str, PathBuf)> {
    let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("C:\\Users\\Default"));
    let appdata = std::env::var("APPDATA").map(PathBuf::from).unwrap_or_else(|_| home.join("AppData\\Roaming"));
    vec![
        ("antigravity", "Antigravity (Gemini Agent)", home.join(".gemini\\config\\mcp_config.json")),
        ("claude", "Claude Desktop", appdata.join("Claude\\claude_desktop_config.json")),
        ("cursor", "Cursor IDE", home.join(".cursor\\mcp.json")),
        ("windsurf", "Windsurf", home.join(".codeium\\windsurf\\mcp_config.json")),
    ]
}

fn inspect_mcp_clients() -> Value {
    let exe = mcp_exe_path();
    let mut clients = vec![];
    for (id, name, path) in mcp_client_targets() {
        let raw = std::fs::read_to_string(&path).ok();
        let parsed: Option<Value> = raw.as_deref().and_then(|s| serde_json::from_str(s).ok());
        let entry = parsed.as_ref().and_then(|v| v.get("mcpServers")).and_then(|m| m.get("draft-studio"));
        let cmd = entry.and_then(|e| e.get("command")).and_then(Value::as_str).map(str::to_string);
        let connected = entry.is_some();
        let installed = path.exists() || path.parent().is_some_and(|p| p.exists());
        clients.push(json!({
            "id": id,
            "name": name,
            "path": path.to_string_lossy(),
            "installed": installed,
            "connected": connected,
            "command": cmd,
        }));
    }
    json!({
        "mcp_exe": exe,
        "clients": clients,
    })
}

async fn get_mcp_clients() -> Response {
    run(|| Ok(inspect_mcp_clients())).await
}

async fn post_mcp_clients(Json(b): Json<Value>) -> Response {
    run(move || {
        let target = b["client"].as_str().unwrap_or("all");
        let action = b["action"].as_str().unwrap_or("connect");
        let exe = mcp_exe_path();
        for (id, _name, path) in mcp_client_targets() {
            if target != "all" && target != id {
                continue;
            }
            if target == "all" && action == "connect" && !path.exists() && !path.parent().is_some_and(|p| p.exists()) && id != "antigravity" && id != "cursor" {
                continue;
            }
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let mut root: Value = std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .filter(Value::is_object)
                .unwrap_or_else(|| json!({}));
            if !root.get("mcpServers").is_some_and(Value::is_object) {
                root["mcpServers"] = json!({});
            }
            if let Some(servers) = root["mcpServers"].as_object_mut() {
                if action == "disconnect" {
                    servers.remove("draft-studio");
                } else {
                    servers.insert("draft-studio".to_string(), json!({
                        "command": exe,
                        "args": ["mcp"]
                    }));
                }
            }
            std::fs::write(&path, serde_json::to_string_pretty(&root)?)?;
        }
        if action != "disconnect" {
            let mut se = Settings::load();
            if !se.open_studio_on_agent_build {
                se.open_studio_on_agent_build = true;
                let _ = se.save();
            }
        }
        Ok(inspect_mcp_clients())
    })
    .await
}
