//! Reading the drafts already in CapCut: list, inspect, repair.

use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::paths::{capcut_path, capcut_root, clean_path_str};

#[derive(Debug, Clone, Serialize)]
pub struct DraftSummary {
    pub name: String,
    pub path: String,
    pub cover: Option<String>,
    pub duration_sec: f64,
    pub width: u32,
    pub height: u32,
    pub video_clips: usize,
    pub overlay_tracks: usize,
    pub audio_clips: usize,
    pub modified: String,
    pub modified_ts: i64,
    pub registered: bool,
    /// CapCut has opened it (it keeps the live timeline in `Timelines/`).
    pub opened_in_capcut: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SegmentInfo {
    pub start_sec: f64,
    pub dur_sec: f64,
    pub name: String,
    pub path: String,
    pub missing: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackInfo {
    pub kind: String,
    pub overlay: bool,
    pub name: String,
    pub segments: Vec<SegmentInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftDetail {
    pub summary: DraftSummary,
    pub id: String,
    pub fps: f64,
    pub content_file: String,
    pub tracks: Vec<TrackInfo>,
    pub missing_media: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepairResult {
    pub draft_path: String,
    pub cleaned_files: Vec<String>,
    pub fixed_paths: usize,
    pub message: String,
}

fn registered_folders(root: &Path) -> HashSet<String> {
    fs::read_to_string(root.join("root_meta_info.json"))
        .ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .and_then(|v| v["all_draft_store"].as_array().cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|d| d["draft_fold_path"].as_str().map(|p| clean_path_str(p).to_lowercase()))
        .collect()
}

/// The JSON CapCut actually edits: `Timelines/<main>/draft_content.json` once the draft has been
/// opened, else the folder's own `draft_content.json`.
pub fn live_content_file(dir: &Path) -> PathBuf {
    let tl = dir.join("Timelines");
    if let Ok(s) = fs::read_to_string(tl.join("project.json")) {
        if let Some(main) = serde_json::from_str::<Value>(&s).ok().and_then(|v| v["main_timeline_id"].as_str().map(str::to_string)) {
            let f = tl.join(main).join("draft_content.json");
            if f.is_file() {
                return f;
            }
        }
    }
    dir.join("draft_content.json")
}

fn read_content(dir: &Path) -> Option<Value> {
    let raw = fs::read_to_string(live_content_file(dir)).ok()?;
    serde_json::from_str(&raw).ok()
}

fn summarize(dir: &Path, registered: &HashSet<String>) -> Option<DraftSummary> {
    let content = dir.join("draft_content.json");
    if !content.is_file() {
        return None;
    }
    let v = read_content(dir).unwrap_or(Value::Null);
    let tracks = v["tracks"].as_array().cloned().unwrap_or_default();
    let count = |kind: &str, overlay: Option<bool>| -> usize {
        tracks
            .iter()
            .filter(|t| t["type"] == kind && overlay.map(|o| (t["flag"].as_u64() == Some(2)) == o).unwrap_or(true))
            .map(|t| t["segments"].as_array().map(|s| s.len()).unwrap_or(0))
            .sum()
    };
    let overlay_tracks = tracks.iter().filter(|t| t["type"] == "video" && t["flag"].as_u64() == Some(2)).count();
    let modified = fs::metadata(live_content_file(dir)).and_then(|m| m.modified()).ok().map(chrono::DateTime::<chrono::Local>::from);
    let cover = dir.join("draft_cover.jpg");
    // The folder name: unique, and what `inspect` resolves (meta names are sometimes stale copies).
    let name = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let path = capcut_path(dir);
    Some(DraftSummary {
        name,
        registered: registered.contains(&path.to_lowercase()),
        cover: cover.is_file().then(|| capcut_path(&cover)),
        duration_sec: v["duration"].as_f64().unwrap_or(0.0) / 1e6,
        width: v["canvas_config"]["width"].as_u64().unwrap_or(0) as u32,
        height: v["canvas_config"]["height"].as_u64().unwrap_or(0) as u32,
        video_clips: count("video", Some(false)),
        overlay_tracks,
        audio_clips: count("audio", None),
        modified: modified.map(|d| d.format("%Y-%m-%d %H:%M").to_string()).unwrap_or_default(),
        modified_ts: modified.map(|d| d.timestamp()).unwrap_or(0),
        opened_in_capcut: dir.join("Timelines").is_dir(),
        path,
    })
}

pub fn list_drafts(custom_root: Option<&Path>) -> Result<Vec<DraftSummary>> {
    let root = custom_root.map(Path::to_path_buf).unwrap_or_else(capcut_root);
    if !root.exists() {
        return Ok(vec![]);
    }
    let reg = registered_folders(&root);
    let mut out: Vec<DraftSummary> = fs::read_dir(&root)?.flatten().map(|e| e.path()).filter(|p| p.is_dir()).filter_map(|p| summarize(&p, &reg)).collect();
    out.sort_by(|a, b| b.modified_ts.cmp(&a.modified_ts));
    Ok(out)
}

pub fn resolve_draft(name_or_path: &str) -> Result<PathBuf> {
    let p = PathBuf::from(name_or_path);
    if p.is_dir() {
        return Ok(p);
    }
    let root = capcut_root();
    let direct = root.join(name_or_path);
    if direct.is_dir() {
        return Ok(direct);
    }
    for e in fs::read_dir(&root).into_iter().flatten().flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        if n.eq_ignore_ascii_case(name_or_path) {
            return Ok(e.path());
        }
    }
    Err(anyhow!("không tìm thấy draft \"{name_or_path}\" trong {}", root.display()))
}

pub fn inspect_draft(name_or_path: &str) -> Result<DraftDetail> {
    let dir = resolve_draft(name_or_path)?;
    let file = live_content_file(&dir);
    let raw = fs::read_to_string(&file).with_context(|| format!("đọc {}", file.display()))?;
    let v: Value = serde_json::from_str(&raw).with_context(|| format!("{} không phải JSON (CapCut bản mới có thể đã mã hoá)", file.display()))?;
    let reg = registered_folders(&capcut_root());
    let summary = summarize(&dir, &reg).ok_or_else(|| anyhow!("thư mục không có draft_content.json"))?;
    let mut by_id = std::collections::HashMap::new();
    for kind in ["videos", "audios"] {
        for m in v["materials"][kind].as_array().into_iter().flatten() {
            let name = m["material_name"].as_str().or(m["name"].as_str()).unwrap_or("").to_string();
            let path = clean_path_str(m["path"].as_str().unwrap_or(""));
            if let Some(id) = m["id"].as_str() {
                by_id.insert(id.to_string(), (name, path));
            }
        }
    }
    let mut missing = vec![];
    let tracks = v["tracks"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|t| {
            let segments = t["segments"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|s| {
                    let (name, path) = by_id.get(s["material_id"].as_str().unwrap_or("")).cloned().unwrap_or_default();
                    let is_missing = !path.is_empty() && !Path::new(&path).exists();
                    if is_missing && !missing.contains(&path) {
                        missing.push(path.clone());
                    }
                    SegmentInfo {
                        start_sec: s["target_timerange"]["start"].as_f64().unwrap_or(0.0) / 1e6,
                        dur_sec: s["target_timerange"]["duration"].as_f64().unwrap_or(0.0) / 1e6,
                        name: if name.is_empty() { s["desc"].as_str().unwrap_or("").to_string() } else { name },
                        path,
                        missing: is_missing,
                    }
                })
                .collect();
            TrackInfo {
                kind: t["type"].as_str().unwrap_or("").to_string(),
                overlay: t["flag"].as_u64() == Some(2),
                name: t["name"].as_str().unwrap_or("").to_string(),
                segments,
            }
        })
        .collect();
    Ok(DraftDetail {
        summary,
        id: v["id"].as_str().unwrap_or("").to_string(),
        fps: v["fps"].as_f64().unwrap_or(30.0),
        content_file: capcut_path(&file),
        tracks,
        missing_media: missing,
    })
}

/// Strip `\\?\` / `//?/` prefixes from every JSON-ish file of a draft (including `Timelines/`).
pub fn repair_draft(name_or_path: &str) -> Result<RepairResult> {
    let dir = resolve_draft(name_or_path)?;
    let mut cleaned = vec![];
    let mut fixed = 0;
    for e in walkdir::WalkDir::new(&dir).max_depth(4).into_iter().flatten() {
        let p = e.path();
        if !p.is_file() {
            continue;
        }
        let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("").to_ascii_lowercase();
        if !matches!(ext.as_str(), "json" | "tmp" | "bak") && p.file_name().and_then(|n| n.to_str()) != Some("draft_settings") {
            continue;
        }
        let Ok(s) = fs::read_to_string(p) else { continue };
        let n = s.matches("//?/").count() + s.matches(r"\\\\?\\").count() + s.matches(r"\\?\").count();
        if n == 0 {
            continue;
        }
        let out = s.replace(r"\\\\?\\", "").replace(r"\\?\", "").replace("//?/", "");
        fs::write(p, out)?;
        fixed += n;
        cleaned.push(p.strip_prefix(&dir).map(|r| r.to_string_lossy().to_string()).unwrap_or_default());
    }
    Ok(RepairResult {
        draft_path: capcut_path(&dir),
        message: if fixed == 0 { "Draft sạch, không có đường dẫn lỗi.".into() } else { format!("Đã sửa {fixed} đường dẫn trong {} file.", cleaned.len()) },
        cleaned_files: cleaned,
        fixed_paths: fixed,
    })
}
