//! Live feed of MCP tool calls to an open Draft Studio, so the user can watch an agent build:
//! the call, the spec, every progress line, each craft file as it lands, the result.
//!
//! Fire and forget. Events go through a bounded queue to one background thread that POSTs them
//! to `http://127.0.0.1:<studio port>/api/activity` (plain std TcpStream, short timeouts). A
//! closed, busy or slow studio never delays or fails a tool: a full queue drops events, a failed
//! POST is retried a little later. The events of the current call are kept, so a studio opened
//! mid-build is sent the whole call so far.

use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, RecvTimeoutError, SyncSender};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::build::Asset;
use crate::settings::Settings;
use crate::spec::ProjectSpec;

/// Tools that build or live-edit a draft from a spec; the studio puts their spec on its timeline.
pub const BUILD_TOOLS: &[&str] = &["capcut_build_project", "capcut_create_draft", "capcut_quick_slideshow", "capcut_live_edit"];

/// Queue between the tools and the sender thread; past this, events are dropped.
const QUEUE: usize = 4096;
/// Events of one call kept for a studio that opens mid-call (delivered log lines are pruned past it).
const KEEP: usize = 3000;
/// Events per POST.
const BATCH: usize = 200;
/// Wait after a failed POST (studio closed) before trying again.
const RETRY: Duration = Duration::from_millis(1500);
/// Undelivered events older than this are given up (nobody opened the studio).
const GIVE_UP: Duration = Duration::from_secs(120);
/// While a call runs, a heartbeat at least this often lets the studio tell a live agent from a dead one.
const HEARTBEAT: Duration = Duration::from_secs(5);

/// True when a Draft Studio answers on `port` (raw `GET /api/ping`).
pub fn studio_alive(port: u16) -> bool {
    let Ok(mut s) = TcpStream::connect_timeout(&([127, 0, 0, 1], port).into(), Duration::from_millis(500)) else {
        return false;
    };
    let _ = s.set_read_timeout(Some(Duration::from_secs(2)));
    if s.write_all(b"GET /api/ping HTTP/1.0\r\nHost: 127.0.0.1\r\n\r\n").is_err() {
        return false;
    }
    let mut buf = String::new();
    let _ = s.read_to_string(&mut buf);
    buf.contains("draft-studio")
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

enum Msg {
    Event(Value),
    Flush(SyncSender<()>),
}

struct Feed {
    tx: SyncSender<Msg>,
    session: String,
    calls: AtomicU64,
}

/// The MCP client's name, from `initialize`.
static CLIENT: Mutex<String> = Mutex::new(String::new());

fn feed() -> &'static Feed {
    static F: OnceLock<Feed> = OnceLock::new();
    F.get_or_init(|| {
        let (tx, rx) = sync_channel(QUEUE);
        let port = Settings::load().port();
        let _ = std::thread::Builder::new().name("studio-feed".into()).spawn(move || sender(rx, port));
        Feed { tx, session: crate::new_id()[..8].to_string(), calls: AtomicU64::new(0) }
    })
}

/// The MCP client's name (from `initialize`), shown in the studio as who is building.
pub fn set_client(name: &str) {
    if let Ok(mut c) = CLIENT.lock() {
        *c = name.chars().take(60).collect();
    }
}

/// Set once a call has used the feed; `flush` does not start the sender thread for nothing.
static FEED_STARTED: OnceLock<()> = OnceLock::new();

/// Wait (at most `max`) until queued events are delivered or given up. For a process about to
/// exit; tools never wait on the feed.
pub fn flush(max: Duration) {
    if FEED_STARTED.get().is_none() {
        return;
    }
    let (tx, rx) = sync_channel(1);
    if feed().tx.try_send(Msg::Flush(tx)).is_ok() {
        let _ = rx.recv_timeout(max);
    }
}

/// One tools/call as the studio sees it.
pub struct Call {
    id: u64,
    build: bool,
}

impl Call {
    /// Announce a tool call (tool name, short summary of its arguments).
    pub fn start(tool: &str, args: &Value) -> Call {
        let _ = FEED_STARTED.set(());
        let f = feed();
        let id = f.calls.fetch_add(1, Ordering::SeqCst) + 1;
        let build = BUILD_TOOLS.contains(&tool);
        let mut ev = json!({"type": "call_start", "tool": tool, "summary": summarize(tool, args), "build": build, "pid": std::process::id()});
        if !build {
            let raw = args.to_string();
            if raw.len() <= 4000 {
                ev["args"] = args.clone();
            }
        }
        let call = Call { id, build };
        call.send(ev);
        if build && Settings::load().open_studio_on_agent_build {
            open_studio();
        }
        call
    }

    fn send(&self, mut ev: Value) {
        let f = feed();
        ev["session"] = json!(f.session);
        ev["client"] = json!(CLIENT.lock().map(|c| c.clone()).unwrap_or_default());
        ev["call"] = json!(self.id);
        ev["ts"] = json!(now_ms());
        // full queue (studio far behind): drop rather than block the tool
        let _ = f.tx.try_send(Msg::Event(ev));
    }

    /// The spec a build tool is about to build.
    pub fn spec(&self, spec: &ProjectSpec) {
        if let Ok(v) = serde_json::to_value(spec) {
            self.send(json!({"type": "spec", "spec": v}));
        }
    }

    /// One progress line of the build.
    pub fn log(&self, line: &str) {
        self.send(json!({"type": "log", "line": line}));
    }

    /// A craft file is ready (VectorCraft PNG, EffectCraft video).
    pub fn asset(&self, a: &Asset) {
        self.send(json!({"type": "asset", "asset": a}));
    }

    /// The call's outcome: the build report for build tools, a short summary otherwise.
    pub fn finish(&self, r: &anyhow::Result<Value>) {
        match r {
            Ok(v) if self.build => self.send(json!({"type": "result", "report": v})),
            Ok(v) => {
                let text = v.to_string();
                let short: String = text.chars().take(600).collect();
                self.send(json!({"type": "result", "summary": if short.len() < text.len() { format!("{short}…") } else { short }}));
            }
            Err(e) => self.send(json!({"type": "error", "error": format!("{e:#}")})),
        }
    }
}

/// "Seneca" · 3 đoạn · 12 cảnh · 4 overlay for build tools; `key=value` pairs for the rest.
fn summarize(tool: &str, a: &Value) -> String {
    let n = |v: &Value, k: &str| v.get(k).and_then(Value::as_array).map(|x| x.len()).unwrap_or(0);
    if tool == "capcut_quick_slideshow" {
        return format!("\"{}\" · ảnh từ {}", a["name"].as_str().unwrap_or("?"), a["images_dir"].as_str().unwrap_or("?"));
    }
    if BUILD_TOOLS.contains(&tool) {
        let s = if matches!(tool, "capcut_build_project" | "capcut_live_edit") { &a["spec"] } else { a };
        let scenes = n(s, "scenes") + n(s, "video_clips") + s["sections"].as_array().into_iter().flatten().map(|x| n(x, "scenes")).sum::<usize>();
        let mut parts = vec![format!("\"{}\"", s["project_name"].as_str().unwrap_or("?"))];
        if let Some(note) = a["step_note"].as_str().filter(|x| !x.is_empty()) {
            parts.push(note.to_string());
        }
        for (count, label) in [(n(s, "sections"), "đoạn"), (scenes, "cảnh"), (n(s, "overlays"), "overlay"), (n(s, "bgm") + n(s, "bgm_tracks"), "nhạc")] {
            if count > 0 {
                parts.push(format!("{count} {label}"));
            }
        }
        return parts.join(" · ");
    }
    let mut out = vec![];
    for (k, v) in a.as_object().into_iter().flatten() {
        let v = match v {
            Value::String(s) if s.chars().count() > 60 => format!("{}…", s.chars().take(60).collect::<String>()),
            Value::String(s) => s.clone(),
            Value::Array(x) => format!("[{}]", x.len()),
            Value::Object(_) => "{…}".into(),
            other => other.to_string(),
        };
        out.push(format!("{k}={v}"));
    }
    let s = out.join(" ");
    if s.chars().count() > 200 {
        format!("{}…", s.chars().take(200).collect::<String>())
    } else {
        s
    }
}

/// Start the desktop studio if none answers (setting `open_studio_on_agent_build`). Off the
/// tool's thread: the check can take a moment. A running studio brings its own window forward.
fn open_studio() {
    static LAST: Mutex<Option<Instant>> = Mutex::new(None);
    if let Ok(mut l) = LAST.lock() {
        if l.is_some_and(|t| t.elapsed() < Duration::from_secs(60)) {
            return;
        }
        *l = Some(Instant::now());
    }
    let _ = std::thread::Builder::new().name("studio-open".into()).spawn(|| {
        if studio_alive(Settings::load().port()) {
            return;
        }
        let exe = crate::paths::exe_dir().join("DraftStudio.exe");
        let exe = if exe.is_file() { exe } else { std::env::current_exe().unwrap_or_default() };
        if exe.file_name().and_then(|n| n.to_str()).map(|n| n.eq_ignore_ascii_case("DraftStudio.exe")) != Some(true) {
            eprintln!("[draft-studio] không thấy DraftStudio.exe cạnh {} để mở", crate::paths::exe_dir().display());
            return;
        }
        if let Err(e) = spawn_detached(&exe) {
            eprintln!("[draft-studio] không mở được {}: {e}", exe.display());
        }
    });
}

/// The studio must outlive this MCP server and must not hold its stdio pipes (the client would
/// never see them close).
fn spawn_detached(exe: &Path) -> std::io::Result<()> {
    use std::process::{Command, Stdio};
    let cmd = || {
        let mut c = Command::new(exe);
        c.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        c
    };
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED: u32 = 0x0000_0008;
        const NEW_GROUP: u32 = 0x0000_0200;
        const BREAKAWAY: u32 = 0x0100_0000;
        // leave the client's job object if it allows that, so closing the agent keeps the studio
        if cmd().creation_flags(DETACHED | NEW_GROUP | BREAKAWAY).spawn().is_ok() {
            return Ok(());
        }
        cmd().creation_flags(DETACHED | NEW_GROUP).spawn().map(|_| ())
    }
    #[cfg(not(windows))]
    {
        cmd().spawn().map(|_| ())
    }
}

/// POST a JSON body to the studio; Ok only on a 2xx from a Draft Studio.
fn post(port: u16, body: &[u8]) -> std::io::Result<()> {
    let mut s = TcpStream::connect_timeout(&([127, 0, 0, 1], port).into(), Duration::from_millis(300))?;
    s.set_write_timeout(Some(Duration::from_secs(2)))?;
    s.set_read_timeout(Some(Duration::from_secs(2)))?;
    let head = format!("POST /api/activity HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
    s.write_all(head.as_bytes())?;
    s.write_all(body)?;
    let mut resp = Vec::new();
    let _ = s.take(4096).read_to_end(&mut resp);
    let resp = String::from_utf8_lossy(&resp);
    if resp.starts_with("HTTP/1.1 2") && resp.contains("draft-studio") {
        Ok(())
    } else {
        Err(std::io::Error::other(resp.lines().next().unwrap_or("không có phản hồi").to_string()))
    }
}

/// The background thread: numbers events in queue order, keeps the current call's events and
/// delivers whatever the studio has not got yet.
fn sender(rx: Receiver<Msg>, port: u16) {
    let mut events: Vec<Value> = vec![];
    let mut sent = 0usize;
    let mut seq = 0u64;
    let mut running = false;
    let mut retry_at = Instant::now();
    let mut last_event = Instant::now();
    let mut last_post = Instant::now();
    loop {
        let tick = if running || sent < events.len() { Duration::from_millis(500) } else { Duration::from_secs(30) };
        let mut flushes = vec![];
        let mut take = |m: Msg, events: &mut Vec<Value>, sent: &mut usize| match m {
            Msg::Event(mut ev) => {
                seq += 1;
                ev["seq"] = json!(seq);
                match ev["type"].as_str() {
                    Some("call_start") => {
                        // a new call: what the studio still lacks of the old one is moot
                        events.clear();
                        *sent = 0;
                        running = true;
                    }
                    Some("result" | "error") => running = false,
                    _ => {}
                }
                events.push(ev);
                last_event = Instant::now();
            }
            Msg::Flush(done) => flushes.push(done),
        };
        match rx.recv_timeout(tick) {
            Ok(m) => take(m, &mut events, &mut sent),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }
        while let Ok(m) = rx.try_recv() {
            take(m, &mut events, &mut sent);
        }
        let force = !flushes.is_empty();
        while sent < events.len() && (force || Instant::now() >= retry_at) {
            let end = (sent + BATCH).min(events.len());
            let body = serde_json::to_vec(&events[sent..end]).unwrap_or_default();
            match post(port, &body) {
                Ok(()) => {
                    sent = end;
                    last_post = Instant::now();
                }
                Err(_) => {
                    retry_at = Instant::now() + RETRY;
                    break;
                }
            }
        }
        if running && sent == events.len() && last_post.elapsed() >= HEARTBEAT && Instant::now() >= retry_at {
            if let Some(last) = events.last() {
                seq += 1;
                let beat = json!([{"type": "alive", "session": last["session"], "client": last["client"], "call": last["call"], "ts": now_ms(), "seq": seq}]);
                match post(port, beat.to_string().as_bytes()) {
                    Ok(()) => last_post = Instant::now(),
                    Err(_) => retry_at = Instant::now() + RETRY,
                }
            }
        }
        if sent < events.len() && !running && last_event.elapsed() > GIVE_UP {
            sent = events.len();
        }
        if events.len() > KEEP {
            // keep the call's structure (start, spec, assets, outcome) for a later replay; log
            // lines go once delivered, or undelivered ones too when there are far too many
            let mut rest = events.split_off(sent);
            events.retain(|e| e["type"] != "log");
            if rest.len() > KEEP {
                rest.retain(|e| e["type"] != "log");
            }
            sent = events.len();
            events.extend(rest);
        }
        for f in flushes {
            let _ = f.send(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_summary_counts_section_scenes() {
        let a = json!({"spec": {"project_name": "Seneca", "sections": [{"scenes": [{}, {}]}, {"scenes": [{}]}], "scenes": [{}], "overlays": [{}, {}], "bgm": [{}]}});
        assert_eq!(summarize("capcut_build_project", &a), "\"Seneca\" · 2 đoạn · 4 cảnh · 2 overlay · 1 nhạc");
        assert_eq!(summarize("capcut_create_draft", &json!({"project_name": "X", "video_clips": [{}]})), "\"X\" · 1 cảnh");
    }

    #[test]
    fn other_summary_is_short() {
        let long = "x".repeat(300);
        let s = summarize("filmcraft_probe", &json!({"path": long, "n": 3, "list": [1, 2]}));
        assert!(s.starts_with("list=[2] n=3 path=xxx"), "{s}");
        assert!(s.chars().count() <= 201, "{s}");
    }

    #[test]
    fn post_to_closed_port_fails_fast() {
        // nothing listens on a port we just released
        let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let t = Instant::now();
        assert!(post(port, b"[]").is_err());
        assert!(t.elapsed() < Duration::from_secs(1), "{:?}", t.elapsed());
        assert!(!studio_alive(port));
    }
}
