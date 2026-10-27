//! `capcut_draft_app` — CLI and MCP server (stdio) of Draft Studio.

use anyhow::{anyhow, bail, Result};
use capcut_core::{build_project, crafts, graphics, library, license, mcp, ProjectSpec};
use serde_json::{json, Value};

const HELP: &str = "\
capcut_draft_app — Draft Studio (CLI + MCP)

LỆNH
  activate <access key>                kích hoạt máy này (DS-XXXX-XXXX-XXXX-XXXX, key nhận trong nhóm Zalo)
  deactivate                           gỡ access key khỏi máy này
  status                               CapCut, 3 craft engine, ffmpeg, access key
  list                                 danh sách draft trong CapCut
  inspect <tên|thư mục>                track / segment / media thiếu của một draft
  create --spec <spec.json>            dựng draft từ spec (cảnh, overlay, giọng, nhạc)
  slideshow --name N --images DIR [--duration S] [--no-ken-burns] [--voice F] [--bgm F]
  repair <tên|thư mục>                 xoá tiền tố \\\\?\\ gây crash CapCut
  templates                            mẫu đồ họa VectorCraft + preset EffectCraft
  graphic --template T [--field k=v]... [--accent #RRGGBB] [--size 1920x1080]
  motion --preset P (--image F | --template T [--field k=v]...) [--duration S] [--format prores|webm]
  probe <file>                         đo media (FilmCraft, dự phòng ffprobe)
  tool <tên-tool> [json]               gọi trực tiếp một tool MCP
  mcp                                  MCP server qua stdio (cho Claude, Cursor, Antigravity…)
  --json                               in kết quả dạng JSON
";

struct Args {
    pos: Vec<String>,
    opts: Vec<(String, String)>,
    flags: Vec<String>,
}

impl Args {
    fn parse(raw: Vec<String>) -> Args {
        const VALUED: &[&str] = &["--spec", "--name", "--images", "--duration", "--voice", "--bgm", "--template", "--field", "--accent", "--size", "--preset", "--image", "--format", "--font", "--note"];
        let mut a = Args { pos: vec![], opts: vec![], flags: vec![] };
        let mut it = raw.into_iter();
        while let Some(s) = it.next() {
            if VALUED.contains(&s.as_str()) {
                let v = it.next().unwrap_or_default();
                a.opts.push((s, v));
            } else if s.starts_with("--") {
                a.flags.push(s);
            } else {
                a.pos.push(s);
            }
        }
        a
    }
    fn opt(&self, k: &str) -> Option<&str> {
        self.opts.iter().rev().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
    }
    fn all(&self, k: &str) -> Vec<&str> {
        self.opts.iter().filter(|(n, _)| n == k).map(|(_, v)| v.as_str()).collect()
    }
    fn flag(&self, k: &str) -> bool {
        self.flags.iter().any(|f| f == k)
    }
}

fn print(v: &Value) {
    println!("{}", serde_json::to_string_pretty(v).unwrap_or_default());
}

fn fields(a: &Args) -> Value {
    let mut m = serde_json::Map::new();
    for kv in a.all("--field") {
        if let Some((k, v)) = kv.split_once('=') {
            m.insert(k.to_string(), json!(v.replace("\\n", "\n")));
        }
    }
    Value::Object(m)
}

fn size(a: &Args) -> (u64, u64) {
    a.opt("--size").and_then(|s| s.split_once('x')).and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))).unwrap_or((1920, 1080))
}

fn log(s: &str) {
    eprintln!("  · {s}");
}

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(raw) {
        eprintln!("Lỗi: {e:#}");
        std::process::exit(1);
    }
}

fn run(raw: Vec<String>) -> Result<()> {
    let a = Args::parse(raw);
    let json_out = a.flag("--json");
    let Some(cmd) = a.pos.first().cloned() else {
        print!("{HELP}");
        return Ok(());
    };
    // MCP gates each tool call itself (an agent can still read the status and activate)
    if !matches!(cmd.as_str(), "mcp" | "help" | "-h" | "version" | "-V" | "status" | "activate" | "deactivate") {
        license::require()?;
    }
    match cmd.as_str() {
        "mcp" => mcp::serve_stdio()?,
        "activate" => {
            let s = license::activate(a.pos.get(1).ok_or_else(|| anyhow!("activate <access key>"))?)?;
            println!("✔ Đã kích hoạt Draft Studio trên máy này ({}).", s.key.unwrap_or_default());
        }
        "deactivate" => {
            license::deactivate()?;
            println!("Đã gỡ access key khỏi máy này.");
        }
        "help" | "-h" => print!("{HELP}"),
        "version" | "-V" => println!("capcut_draft_app {}", capcut_core::VERSION),
        "status" => {
            let s = mcp::status();
            if json_out {
                print(&s);
            } else {
                println!("Draft Studio {}", capcut_core::VERSION);
                println!("CapCut drafts : {} ({} draft){}", s["capcut_root"].as_str().unwrap_or(""), s["drafts"], if s["capcut_running"] == true { "  ⚠ CapCut đang mở" } else { "" });
                for c in s["crafts"].as_array().into_iter().flatten() {
                    let ok = c["ready"] == true;
                    println!("{} {:<12} {}", if ok { "✔" } else { "✘" }, c["name"].as_str().unwrap_or(""), if ok { c["version"].as_str().or(c["path"].as_str()).unwrap_or("") } else { "chưa có (xem Cài đặt hoặc README)" });
                }
                println!("ffprobe {}  ffmpeg {}", if s["ffprobe"] == true { "✔" } else { "✘" }, if s["ffmpeg"] == true { "✔" } else { "✘" });
                let l = &s["license"];
                if l["activated"] == true {
                    println!("Access key    : ✔ đã kích hoạt ({})", l["key"].as_str().unwrap_or(""));
                } else {
                    println!("Access key    : ✘ chưa kích hoạt — capcut_draft_app activate DS-XXXX-XXXX-XXXX-XXXX");
                }
                println!("Tài nguyên sinh ra: {}", s["assets_dir"].as_str().unwrap_or(""));
            }
        }
        "list" => {
            let d = library::list_drafts(None)?;
            if json_out {
                print(&json!(d));
            } else {
                for x in d {
                    println!("{:<52} {:>7.1}s  {:>3} cảnh  {} overlay  {:>2} audio  {}", x.name, x.duration_sec, x.video_clips, x.overlay_tracks, x.audio_clips, x.modified);
                }
            }
        }
        "inspect" => {
            let name = a.pos.get(1).ok_or_else(|| anyhow!("inspect <tên>"))?;
            let d = library::inspect_draft(name)?;
            if json_out {
                print(&json!(d));
            } else {
                println!("{}  {}x{} @{}fps  {:.1}s", d.summary.name, d.summary.width, d.summary.height, d.fps, d.summary.duration_sec);
                for t in &d.tracks {
                    println!("  [{}{}] {} đoạn", t.kind, if t.overlay { " overlay" } else { "" }, t.segments.len());
                    for s in t.segments.iter().take(6) {
                        println!("     {:>7.2}s +{:<6.2} {}{}", s.start_sec, s.dur_sec, s.name, if s.missing { "  ✘ THIẾU FILE" } else { "" });
                    }
                }
                if !d.missing_media.is_empty() {
                    println!("Thiếu {} file media.", d.missing_media.len());
                }
            }
        }
        "create" | "build" => {
            let p = a.opt("--spec").or(a.pos.get(1).map(String::as_str)).ok_or_else(|| anyhow!("create --spec <spec.json>"))?;
            let spec: ProjectSpec = serde_json::from_str(&std::fs::read_to_string(p)?)?;
            capcut_core::activity::set_client("Windows Agent");
            let spec_val = serde_json::to_value(&spec).unwrap_or_else(|_| json!({}));
            let call = capcut_core::activity::Call::start("capcut_build_project", &json!({"spec": spec_val}));
            call.spec(&spec);
            let live_log = |s: &str| {
                log(s);
                call.log(s);
            };
            let res = capcut_core::build::build_project_with(&spec, &live_log, &|asset| call.asset(asset));
            match res {
                Ok(rep) => {
                    call.finish(&Ok(json!(&rep)));
                    capcut_core::activity::flush(std::time::Duration::from_secs(3));
                    report(&rep, json_out);
                }
                Err(e) => {
                    call.finish(&Err(anyhow!("{e:#}")));
                    capcut_core::activity::flush(std::time::Duration::from_secs(3));
                    return Err(e);
                }
            }
        }
        "live" => {
            let p = a.opt("--spec").or(a.pos.get(1).map(String::as_str)).ok_or_else(|| anyhow!("live --spec <spec.json> [--note <text>]"))?;
            let spec: ProjectSpec = serde_json::from_str(&std::fs::read_to_string(p)?)?;
            let note = a.opt("--note");
            capcut_core::activity::set_client("Windows Agent");
            let spec_val = serde_json::to_value(&spec).unwrap_or_else(|_| json!({}));
            let call = capcut_core::activity::Call::start("capcut_live_edit", &json!({"spec": spec_val, "step_note": note}));
            call.spec(&spec);
            let live_log = |s: &str| {
                log(s);
                call.log(s);
            };
            let res = mcp::live_edit_project(&spec, note, &live_log, &|asset| call.asset(asset));
            call.finish(&res);
            capcut_core::activity::flush(std::time::Duration::from_secs(3));
            print(&res?);
        }
        "slideshow" => {
            let mut args = json!({
                "name": a.opt("--name").ok_or_else(|| anyhow!("thiếu --name"))?,
                "images_dir": a.opt("--images").ok_or_else(|| anyhow!("thiếu --images"))?,
                "ken_burns": !a.flag("--no-ken-burns"),
            });
            if let Some(d) = a.opt("--duration").and_then(|d| d.parse::<f64>().ok()) {
                args["duration_sec"] = json!(d);
            }
            if let Some(v) = a.opt("--voice") {
                args["voiceover"] = json!(v);
            }
            if let Some(b) = a.opt("--bgm") {
                args["bgm"] = json!(b);
            }
            let spec = mcp::slideshow_spec(&args)?;
            capcut_core::activity::set_client("Windows Agent");
            let call = capcut_core::activity::Call::start("capcut_quick_slideshow", &args);
            call.spec(&spec);
            let live_log = |s: &str| {
                log(s);
                call.log(s);
            };
            let rep = capcut_core::build::build_project_with(&spec, &live_log, &|asset| call.asset(asset))?;
            call.finish(&Ok(json!(&rep)));
            capcut_core::activity::flush(std::time::Duration::from_secs(3));
            report(&rep, json_out);
        }
        "repair" => {
            let r = library::repair_draft(a.pos.get(1).ok_or_else(|| anyhow!("repair <tên>"))?)?;
            if json_out {
                print(&json!(r));
            } else {
                println!("{}", r.message);
            }
        }
        "templates" => {
            if json_out {
                print(&mcp::catalog());
            } else {
                println!("Đồ họa (VectorCraft):");
                for t in graphics::templates() {
                    let f: Vec<&str> = t.fields.iter().map(|f| f.key).collect();
                    println!("  {:<14} {:<22} trường: {}", t.id, t.name, f.join(", "));
                }
                println!("Chuyển động (EffectCraft):");
                for p in crafts::effect::presets() {
                    println!("  {:<14} {}", p.id, p.description);
                }
            }
        }
        "graphic" => {
            let (w, h) = size(&a);
            let mut args = json!({"template": a.opt("--template").ok_or_else(|| anyhow!("thiếu --template"))?, "fields": fields(&a), "width": w, "height": h});
            if let Some(c) = a.opt("--accent") {
                args["accent"] = json!(c);
            }
            if let Some(f) = a.opt("--font") {
                args["font"] = json!(f);
            }
            print(&mcp::call_tool("vectorcraft_render_graphic", &args, &log)?);
        }
        "motion" => {
            let (w, h) = size(&a);
            let mut args = json!({"preset": a.opt("--preset").ok_or_else(|| anyhow!("thiếu --preset"))?, "width": w, "height": h,
                                  "format": a.opt("--format").unwrap_or("prores")});
            if let Some(d) = a.opt("--duration").and_then(|d| d.parse::<f64>().ok()) {
                args["duration_sec"] = json!(d);
            }
            match (a.opt("--image"), a.opt("--template")) {
                (Some(i), _) => args["image"] = json!(i),
                (None, Some(t)) => args["graphic"] = json!({"template": t, "fields": fields(&a), "accent": a.opt("--accent")}),
                _ => bail!("cần --image hoặc --template"),
            }
            print(&mcp::call_tool("effectcraft_render_motion", &args, &log)?);
        }
        "probe" => print(&mcp::call_tool("filmcraft_probe", &json!({"path": a.pos.get(1).ok_or_else(|| anyhow!("probe <file>"))?}), &log)?),
        "tool" => {
            let name = a.pos.get(1).ok_or_else(|| anyhow!("tool <tên> [json]"))?;
            let args: Value = match a.pos.get(2) {
                Some(s) => serde_json::from_str(s)?,
                None => json!({}),
            };
            print(&mcp::call_tool(name, &args, &log)?);
        }
        other => bail!("lệnh không rõ '{other}'\n\n{HELP}"),
    }
    Ok(())
}

fn report(r: &capcut_core::BuildReport, json_out: bool) {
    if json_out {
        print(&json!(r));
        return;
    }
    println!("\n✔ Draft \"{}\" ({:.1}s) → {}", r.project_name, r.draft.duration_sec, r.draft.draft_dir);
    for t in &r.tracks {
        println!("   {:<11} {} đoạn", t.label, t.clips.len());
    }
    for a in &r.assets {
        println!("   {} {:<8} {}{}", a.craft, a.kind, a.path, if a.cached { " (dùng lại)" } else { "" });
    }
    if let Some(p) = &r.preview {
        println!("   Xem trước FilmCraft: {}", p.mp4);
    }
    if let Some(old) = &r.draft.replaced_previous {
        println!("   Bản cũ cùng tên đã cất ở: {old}");
    }
    for w in &r.warnings {
        println!("   ⚠ {w}");
    }
    if r.capcut_running {
        println!("   ⚠ CapCut đang mở: tắt hẳn CapCut rồi mở lại để thấy dự án ở đầu danh sách.");
    } else if r.draft.registered {
        println!("   Mở CapCut PC: dự án nằm đầu tab Projects.");
    }
}
