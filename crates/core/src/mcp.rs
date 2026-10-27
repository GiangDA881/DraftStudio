//! MCP tools (JSON-RPC 2.0 over stdio). The studio's HTTP API calls the same `call_tool`, so an
//! agent and the UI always do exactly the same thing.

use anyhow::{anyhow, bail, Result};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;

use crate::activity;
use crate::build::{build_project, build_project_with, project_assets_dir};
use crate::crafts::{self, effect, film, vector};
use crate::graphics;
use crate::library;
use crate::license;
use crate::media::{self, MediaKind};
use crate::paths::{capcut_root, capcut_running};
use crate::settings::Settings;
use crate::spec::{AudioInput, BgmInput, GraphicSpec, ProjectSpec, SceneInput};

const PROTOCOLS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

fn spec_schema() -> Value {
    let scene = json!({"type": "object", "required": ["path"], "properties": {
        "path": {"type": "string"}, "duration_sec": {"type": "number"}, "trim_start_sec": {"type": "number"},
        "ken_burns": {"type": "boolean"}, "zoom_start": {"type": "number"}, "zoom_end": {"type": "number"},
        "pan": {"type": "string", "enum": ["none", "left", "right", "up", "down"]}, "volume": {"type": "number"},
        "transition": {"type": "string", "description": "FilmCraft video transition into this scene (e.g. cross_zoom, whip, flash, glitch, light_leak, neon_wipe, spin_3d, earthquake, zoom_blur)"},
        "transition_sec": {"type": "number", "default": 0.45},
        "lut": {"type": "string", "description": "FilmCraft Look LUT (e.g. builtin:look-teal-orange, builtin:look-golden-hour, builtin:look-night, builtin:look-cool-blue)"},
        "video_effect": {"type": "string", "description": "FilmCraft video effect (e.g. wonder_glow, rgb_split, camera_shake, glint, light_leaks, volumetric_rays, edge_glow, vignette)"},
        "desc": {"type": "string"}}});
    json!({
        "type": "object",
        "description": "Project spec. Times in seconds. For narration use sections: one voiceover each, its scenes are fitted to it, so no per-clip duration_sec is needed. Overlay kind: file (path) | graphic (VectorCraft draws graphic.template) | motion (EffectCraft animates graphic or path with motion.preset). See craft_list_templates.",
        "required": ["project_name"],
        "properties": {
            "project_name": {"type": "string"},
            "width": {"type": "integer", "default": 1920}, "height": {"type": "integer", "default": 1080}, "fps": {"type": "number", "default": 30},
            "sections": {"type": "array", "description": "Narrated blocks laid out first, back to back; section N+1 starts exactly when voice N ends (+ gap_after_sec). Within a section, scenes without duration_sec are fitted to voice + gap: photos absorb the slack (at least 2 s each while videos are being trimmed), videos are trimmed proportionally from their tail and never stretched. Too little footage for the voice is an error that says how many seconds are missing; add a photo to let it stretch. Top-level scenes and voiceovers play after the last section.",
                "items": {"type": "object", "required": ["voice", "scenes"], "properties": {
                    "voice": {"type": "object", "required": ["path"], "properties": {"path": {"type": "string"}, "volume": {"type": "number"}, "duration_sec": {"type": "number", "description": "override the file length"}, "desc": {"type": "string"}}},
                    "scenes": {"type": "array", "items": scene.clone()},
                    "gap_after_sec": {"type": "number", "default": 0, "description": "silence after the voice, covered by this section's pictures"},
                    "desc": {"type": "string"}}}},
            "scenes": {"type": "array", "description": "Main track after the sections (or alone), back to back; use duration_sec or fit_scenes_to_voice.", "items": scene},
            "overlays": {"type": "array", "items": {"type": "object", "properties": {
                "kind": {"type": "string", "enum": ["file", "graphic", "motion"]}, "path": {"type": "string"},
                "graphic": {"type": "object", "properties": {"template": {"type": "string"}, "fields": {"type": "object"}, "accent": {"type": "string"}, "font": {"type": "string"}}},
                "motion": {"type": "object", "properties": {"preset": {"type": "string"}, "format": {"type": "string", "enum": ["prores", "webm"]}}},
                "start_sec": {"type": "number"}, "section": {"type": "integer", "description": "0-based index into sections: start_sec then counts from that section's start"},
                "duration_sec": {"type": "number"}, "x": {"type": "number"}, "y": {"type": "number"},
                "scale": {"type": "number"}, "opacity": {"type": "number"}, "fade_in_sec": {"type": "number"}, "fade_out_sec": {"type": "number"}}}},
            "voiceovers": {"type": "array", "description": "Free voiceovers: start_sec, else back to back after the sections.", "items": {"type": "object", "required": ["path"], "properties": {"path": {"type": "string"}, "start_sec": {"type": "number"}, "volume": {"type": "number"}}}},
            "bgm": {"type": "array", "items": {"type": "object", "required": ["path"], "properties": {
                "path": {"type": "string"}, "volume": {"type": "number", "default": 0.3, "description": "level in silences; under the voice it drops to volume × duck_ratio"}, "fade_in_sec": {"type": "number"}, "fade_out_sec": {"type": "number"},
                "loop_to_fill": {"type": "boolean"}, "duck_under_voice": {"type": "boolean"}, "duck_ratio": {"type": "number"}}}},
            "fit_scenes_to_voice": {"type": "boolean"},
            "register_to_capcut": {"type": "boolean", "default": true},
            "output_dir": {"type": "string"},
            "film_preview": {"type": "boolean", "description": "Also render an MP4 preview with FilmCraft"}
        }
    })
}

pub fn tools() -> Value {
    let name_arg = json!({"type": "object", "required": ["name"], "properties": {"name": {"type": "string", "description": "Draft name or folder path"}}});
    json!([
        {"name": "capcut_get_status", "description": "CapCut folder, whether CapCut is running, the three craft engines (VectorCraft, EffectCraft, FilmCraft) and ffprobe/ffmpeg.", "inputSchema": {"type": "object", "properties": {}}},
        {"name": "capcut_list_drafts", "description": "List CapCut PC drafts: name, duration, clips, overlay tracks, audio clips, modified time.", "inputSchema": {"type": "object", "properties": {"custom_root": {"type": "string"}}}},
        {"name": "capcut_get_draft", "description": "Tracks and segments of one draft (reads CapCut's live Timelines copy), with missing media.", "inputSchema": name_arg},
        {"name": "capcut_live_edit", "description": "Live-update a project spec on Draft Studio's stage and timeline in real time (like live code editing in an IDE) without running a full export yet. Automatically saves the project to Draft Studio and renders any VectorCraft graphic overlays onto the live canvas. Use this to show step-by-step progress (e.g., Step 1: add scenes, Step 2: apply transitions/LUTs/FX, Step 3: add overlays/BGM) before calling capcut_build_project.", "inputSchema": {"type": "object", "required": ["spec"], "properties": {"spec": spec_schema(), "step_note": {"type": "string", "description": "Short step description displayed live in Draft Studio HUD"}}}},
        {"name": "capcut_build_project", "description": "Build a CapCut PC draft from a project spec: main scenes (Ken Burns, transitions, LUTs, video FX), overlay graphics (VectorCraft) and motion (EffectCraft alpha video), voiceovers, music (loop, fades, ducking under voice); registers it on CapCut's home screen; optional FilmCraft MP4 preview. For narrated videos put each voiceover with its clips/photos in a section: clip lengths are fitted to the voice automatically and every section's cut lands on its voice start; overlays with `section` are timed from that section's start.", "inputSchema": {"type": "object", "required": ["spec"], "properties": {"spec": spec_schema()}}},
        {"name": "capcut_create_draft", "description": "Same as capcut_build_project with the spec fields at top level (v1: video_clips, voiceovers, bgm_tracks are accepted).", "inputSchema": spec_schema()},
        {"name": "capcut_quick_slideshow", "description": "Slideshow from a folder of images (sorted by name), optional voiceover and music; with fit_to_voice the pictures share the narration length.", "inputSchema": {"type": "object", "required": ["name", "images_dir"], "properties": {
            "name": {"type": "string"}, "images_dir": {"type": "string"}, "duration_sec": {"type": "number", "default": 5}, "ken_burns": {"type": "boolean", "default": true},
            "voiceover": {"type": "string"}, "bgm": {"type": "string"}, "fit_to_voice": {"type": "boolean", "default": true}, "width": {"type": "integer"}, "height": {"type": "integer"}}}},
        {"name": "capcut_repair_draft", "description": "Strip Windows \\\\?\\ path prefixes that crash CapCut or show 'media not found'.", "inputSchema": name_arg},
        {"name": "craft_list_templates", "description": "VectorCraft graphic templates (with fields) and EffectCraft motion presets.", "inputSchema": {"type": "object", "properties": {}}},
        {"name": "vectorcraft_render_graphic", "description": "Draw a graphic template with VectorCraft → transparent PNG (+ editable .vectorcraft).", "inputSchema": {"type": "object", "required": ["template"], "properties": {
            "template": {"type": "string"}, "fields": {"type": "object"}, "accent": {"type": "string"}, "font": {"type": "string"},
            "width": {"type": "integer", "default": 1920}, "height": {"type": "integer", "default": 1080}, "project_name": {"type": "string", "description": "asset folder"}}}},
        {"name": "effectcraft_render_motion", "description": "Animate a PNG (or a graphic template) with an EffectCraft preset → alpha video (.mov ProRes 4444 or .webm) + .ecproj.", "inputSchema": {"type": "object", "required": ["preset"], "properties": {
            "preset": {"type": "string"}, "image": {"type": "string"}, "graphic": {"type": "object"}, "duration_sec": {"type": "number", "default": 4},
            "format": {"type": "string", "enum": ["prores", "webm"]}, "width": {"type": "integer"}, "height": {"type": "integer"}, "fps": {"type": "number"}, "project_name": {"type": "string"}}}},
        {"name": "filmcraft_probe", "description": "Media info (duration, size, audio) via FilmCraft's pure-Rust prober (ffprobe fallback).", "inputSchema": {"type": "object", "required": ["path"], "properties": {"path": {"type": "string"}}}},
        {"name": "capcut_activate", "description": "Activate Draft Studio on this machine with the user's access key (DS-XXXX-XXXX-XXXX-XXXX). Use it only after a tool reports that Draft Studio is not activated, with the key the user gives you; never guess keys.", "inputSchema": {"type": "object", "required": ["key"], "properties": {"key": {"type": "string"}}}}
    ])
}

fn arg_str<'a>(a: &'a Value, k: &str) -> Option<&'a str> {
    a.get(k).and_then(Value::as_str).filter(|s| !s.trim().is_empty())
}

pub fn status() -> Value {
    let has = |exe: &str| crafts::command(exe).arg("-version").output().map(|o| o.status.success()).unwrap_or(false);
    let root = capcut_root();
    json!({
        "version": crate::VERSION,
        "exe_dir": crate::paths::clean_path_str(&crate::paths::exe_dir().to_string_lossy()),
        "capcut_root": crate::paths::capcut_path(&root),
        "capcut_root_exists": root.exists(),
        "capcut_running": capcut_running(),
        "drafts": library::list_drafts(None).map(|d| d.len()).unwrap_or(0),
        "crafts": crafts::status(),
        "ffprobe": has("ffprobe"),
        "ffmpeg": has("ffmpeg"),
        "assets_dir": crate::paths::clean_path_str(&Settings::load().assets_root().to_string_lossy()),
        "license": license::status(),
    })
}

pub fn catalog() -> Value {
    let graphics = graphics::templates();
    let motion = effect::presets();
    let layer_effects = effect::layer_effects();
    let transitions = film::transitions();
    let luts = film::luts();
    let video_effects = film::video_effects();
    let audio_effects = film::audio_effects();
    let total = graphics.len() + motion.len() + layer_effects.len() + transitions.len() + luts.len() + video_effects.len() + audio_effects.len();
    json!({
        "counts": {
            "total": total,
            "graphics": graphics.len(),
            "motion": motion.len(),
            "layer_effects": layer_effects.len(),
            "transitions": transitions.len(),
            "luts": luts.len(),
            "video_effects": video_effects.len(),
            "audio_effects": audio_effects.len(),
        },
        "graphics": graphics,
        "motion": motion,
        "layer_effects": layer_effects,
        "transitions": transitions,
        "luts": luts,
        "video_effects": video_effects,
        "audio_effects": audio_effects,
    })
}

pub fn slideshow_spec(a: &Value) -> Result<ProjectSpec> {
    let name = arg_str(a, "name").ok_or_else(|| anyhow!("thiếu name"))?;
    let dir = PathBuf::from(arg_str(a, "images_dir").ok_or_else(|| anyhow!("thiếu images_dir"))?);
    let mut imgs: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| anyhow!("không đọc được {}: {e}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| media::kind_of(p) == MediaKind::Image)
        .collect();
    imgs.sort();
    if imgs.is_empty() {
        bail!("không có ảnh trong {}", dir.display());
    }
    let mut spec = ProjectSpec::new(name);
    if let Some(w) = a.get("width").and_then(Value::as_u64) {
        spec.width = w as u32;
    }
    if let Some(h) = a.get("height").and_then(Value::as_u64) {
        spec.height = h as u32;
    }
    let voice = arg_str(a, "voiceover");
    let fit = a.get("fit_to_voice").and_then(Value::as_bool).unwrap_or(true) && voice.is_some();
    let dur = a.get("duration_sec").and_then(Value::as_f64).unwrap_or(5.0);
    let kb = a.get("ken_burns").and_then(Value::as_bool).unwrap_or(true);
    let pans = ["none", "right", "none", "left"];
    for (i, p) in imgs.iter().enumerate() {
        spec.scenes.push(SceneInput {
            path: crate::paths::clean_path_str(&p.to_string_lossy()),
            duration_sec: (!fit).then_some(dur),
            ken_burns: kb,
            zoom_start: 1.0,
            zoom_end: 1.06,
            pan: Some(pans[i % pans.len()].into()),
            ..Default::default()
        });
    }
    if let Some(v) = voice {
        spec.voiceovers.push(AudioInput { path: v.into(), volume: 1.0, ..Default::default() });
    }
    if let Some(b) = arg_str(a, "bgm") {
        spec.bgm.push(serde_json::from_value::<BgmInput>(json!({"path": b}))?);
    }
    spec.fit_scenes_to_voice = fit;
    Ok(spec)
}

/// The spec a build tool (`activity::BUILD_TOOLS`) would build; `None` for the other tools.
pub fn tool_spec(name: &str, a: &Value) -> Option<Result<ProjectSpec>> {
    Some(match name {
        "capcut_build_project" => a.get("spec").cloned().ok_or_else(|| anyhow!("thiếu spec")).and_then(|v| Ok(serde_json::from_value(v)?)),
        "capcut_create_draft" => serde_json::from_value(a.clone()).map_err(Into::into),
        "capcut_quick_slideshow" => slideshow_spec(a),
        _ => return None,
    })
}

/// Live-edit a project spec on the Studio timeline and render any VectorCraft graphics so the
/// user sees the changes immediately on screen (like live code editing).
pub fn live_edit_project(spec: &ProjectSpec, step_note: Option<&str>, log: &(dyn Fn(&str) + Sync), on_asset: &(dyn Fn(&crate::build::Asset) + Sync)) -> Result<Value> {
    if let Some(note) = step_note.filter(|s| !s.trim().is_empty()) {
        log(&format!("⚡ {note}"));
    }
    let slug = crate::build::save_studio_project(spec).unwrap_or_else(|| crate::paths::slug(&spec.project_name));
    let dir = project_assets_dir(&spec.project_name);
    let _ = std::fs::create_dir_all(&dir);
    let mut rendered = vec![];
    for (idx, o) in spec.overlays.iter().enumerate() {
        if let Some(g) = &o.graphic {
            if o.kind != "file" {
                log(&format!("VectorCraft ▸ vẽ \"{}\" (overlay {})", g.template, idx + 1));
                if let Ok(out) = vector::render(g, spec.width, spec.height, &dir.join("graphics")) {
                    let asset = crate::build::Asset {
                        overlay: idx,
                        craft: "vectorcraft",
                        kind: "graphic",
                        path: out.png.clone(),
                        editable: out.vectorcraft_doc.clone(),
                        cached: out.cached,
                    };
                    on_asset(&asset);
                    rendered.push(asset);
                }
            }
        }
    }
    let scenes = spec.scenes.len() + spec.sections.iter().map(|s| s.scenes.len()).sum::<usize>();
    log(&format!("✓ Đã cập nhật trực tiếp lên Timeline: {scenes} cảnh · {} overlay · {} nhạc nền", spec.overlays.len(), spec.bgm.len()));
    Ok(json!({
        "live_updated": true,
        "project_name": spec.project_name,
        "slug": slug,
        "scenes": scenes,
        "overlays": spec.overlays.len(),
        "bgm": spec.bgm.len(),
        "step_note": step_note,
        "graphics_rendered": rendered.len()
    }))
}

/// Run one tool. `log` receives progress lines of long builds.
pub fn call_tool(name: &str, a: &Value, log: &(dyn Fn(&str) + Sync)) -> Result<Value> {
    if name == "capcut_live_edit" {
        let spec: ProjectSpec = serde_json::from_value(a.get("spec").cloned().unwrap_or_else(|| a.clone()))?;
        return live_edit_project(&spec, arg_str(a, "step_note"), log, &|_| {});
    }
    if let Some(spec) = tool_spec(name, a) {
        return Ok(json!(build_project(&spec?, log)?));
    }
    match name {
        "capcut_get_status" => Ok(status()),
        "capcut_list_drafts" => Ok(json!(library::list_drafts(arg_str(a, "custom_root").map(std::path::Path::new))?)),
        "capcut_get_draft" => Ok(json!(library::inspect_draft(arg_str(a, "name").ok_or_else(|| anyhow!("thiếu name"))?)?)),
        "capcut_repair_draft" => Ok(json!(library::repair_draft(arg_str(a, "name").ok_or_else(|| anyhow!("thiếu name"))?)?)),
        "craft_list_templates" => Ok(catalog()),
        "capcut_activate" => Ok(json!(license::activate(arg_str(a, "key").ok_or_else(|| anyhow!("thiếu key"))?)?)),
        "vectorcraft_render_graphic" => {
            let g = GraphicSpec {
                template: arg_str(a, "template").ok_or_else(|| anyhow!("thiếu template"))?.into(),
                fields: serde_json::from_value(a.get("fields").cloned().unwrap_or(json!({})))?,
                accent: arg_str(a, "accent").map(str::to_string),
                font: arg_str(a, "font").map(str::to_string),
            };
            let (w, h) = dims(a);
            let dir = project_assets_dir(arg_str(a, "project_name").unwrap_or("thu-vien")).join("graphics");
            Ok(json!(vector::render(&g, w, h, &dir)?))
        }
        "effectcraft_render_motion" => {
            let (w, h) = dims(a);
            let fps = a.get("fps").and_then(Value::as_f64).unwrap_or(30.0);
            let dur = a.get("duration_sec").and_then(Value::as_f64).unwrap_or(4.0);
            let base = project_assets_dir(arg_str(a, "project_name").unwrap_or("thu-vien"));
            let image = match (arg_str(a, "image"), a.get("graphic")) {
                (Some(p), _) => PathBuf::from(p),
                (None, Some(g)) => {
                    let g: GraphicSpec = serde_json::from_value(g.clone())?;
                    PathBuf::from(vector::render(&g, w, h, &base.join("graphics"))?.png)
                }
                _ => bail!("cần image hoặc graphic"),
            };
            let preset = arg_str(a, "preset").ok_or_else(|| anyhow!("thiếu preset"))?;
            Ok(json!(effect::render(&image, preset, arg_str(a, "format").unwrap_or("prores"), w, h, fps, dur, &base.join("motion"))?))
        }
        "filmcraft_probe" => {
            let p = arg_str(a, "path").ok_or_else(|| anyhow!("thiếu path"))?;
            let film = film::probe(std::path::Path::new(p));
            Ok(json!({"kind": media::kind_of(p), "info": media::probe(p), "filmcraft": film.is_some()}))
        }
        other => bail!("không có tool '{other}'"),
    }
}

fn dims(a: &Value) -> (u32, u32) {
    (
        a.get("width").and_then(Value::as_u64).unwrap_or(1920).clamp(16, 8192) as u32,
        a.get("height").and_then(Value::as_u64).unwrap_or(1080).clamp(16, 8192) as u32,
    )
}

fn reply(id: &Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn error(id: &Value, code: i64, msg: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": msg}})
}

pub fn handle(msg: &Value) -> Option<Value> {
    let id = msg.get("id")?.clone();
    let method = msg["method"].as_str().unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(json!({}));
    Some(match method {
        "initialize" => {
            if let Some(c) = params["clientInfo"]["name"].as_str() {
                activity::set_client(c);
            }
            let asked = params["protocolVersion"].as_str().unwrap_or("2024-11-05");
            let v = if PROTOCOLS.contains(&asked) { asked } else { PROTOCOLS[0] };
            reply(&id, json!({
                "protocolVersion": v,
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": "draft-studio", "version": crate::VERSION},
            "instructions": "Build and live-edit CapCut PC drafts in Draft Studio (Unlocked Pro build — no access key needed). Call capcut_live_edit to stream step-by-step changes (scenes, transitions, LUTs, video FX, overlays, BGM) live onto the Draft Studio stage & timeline in real time (like live code editing in an IDE), then call capcut_build_project to export the final CapCut draft and optional FilmCraft MP4 preview. Call craft_list_templates for all 9 VectorCraft graphic templates, 16 EffectCraft motion presets, 20 FilmCraft transitions, 8 Look LUTs, and 12 Video FX."
            }))
        }
        "ping" => reply(&id, json!({})),
        "tools/list" => reply(&id, json!({"tools": tools()})),
        "tools/call" => {
            let name = params["name"].as_str().unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            if !matches!(name, "capcut_get_status" | "capcut_activate") {
                if let Err(e) = license::require() {
                    return Some(reply(&id, json!({"content": [{"type": "text", "text": format!("Lỗi: {e:#}")}], "isError": true})));
                }
            }
            // An open studio shows the call live (activity.rs); the log still goes to stderr too.
            let call = activity::Call::start(name, &args);
            let log = |s: &str| {
                eprintln!("[draft-studio] {s}");
                call.log(s);
            };
            let r = if name == "capcut_live_edit" {
                serde_json::from_value::<ProjectSpec>(args.get("spec").cloned().unwrap_or_else(|| args.clone()))
                    .map_err(Into::into)
                    .and_then(|spec| {
                        call.spec(&spec);
                        live_edit_project(&spec, arg_str(&args, "step_note"), &log, &|a| call.asset(a))
                    })
            } else {
                match tool_spec(name, &args) {
                    Some(spec) => spec.and_then(|spec| {
                        call.spec(&spec);
                        Ok(json!(build_project_with(&spec, &log, &|a| call.asset(a))?))
                    }),
                    None => call_tool(name, &args, &log),
                }
            };
            call.finish(&r);
            match r {
                Ok(v) => {
                    let text = serde_json::to_string_pretty(&v).unwrap_or_default();
                    let structured = if v.is_object() { v } else { json!({"result": v}) };
                    reply(&id, json!({"content": [{"type": "text", "text": text}], "structuredContent": structured, "isError": false}))
                }
                Err(e) => reply(&id, json!({"content": [{"type": "text", "text": format!("Lỗi: {e:#}")}], "isError": true})),
            }
        }
        m if m.starts_with("notifications/") => return None,
        m => error(&id, -32601, &format!("Method not found: {m}")),
    })
}

pub fn serve_stdio() -> Result<()> {
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let resp = match serde_json::from_str::<Value>(&line) {
            Ok(msg) => handle(&msg),
            Err(e) => Some(error(&Value::Null, -32700, &format!("Parse error: {e}"))),
        };
        if let Some(r) = resp {
            writeln!(out, "{r}")?;
            out.flush()?;
        }
    }
    // stdin closed: give the studio a moment to get the last call's outcome
    activity::flush(std::time::Duration::from_secs(2));
    Ok(())
}
