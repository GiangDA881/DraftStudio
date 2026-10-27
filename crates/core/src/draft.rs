//! Writes a CapCut PC draft folder from a resolved plan.
//!
//! The JSON is the compact skeleton CapCut 9.x accepts and expands on first open (it then
//! moves the timeline into `Timelines/<id>/`). Track layout follows what CapCut itself writes:
//! the main video track (`flag 0`), overlay video tracks (`flag 2`, rising `render_index`),
//! then audio tracks. Fades use `audio_fades` materials, Ken Burns / opacity / ducking use
//! `common_keyframes` (`KFTypeScaleX/Y`, `KFTypePositionX/Y`, `KFTypeAlpha`, `KFTypeVolume`).

use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

use crate::media::MediaKind;
use crate::new_id;
use crate::paths::{capcut_path, capcut_root};

pub const NEW_VERSION: &str = "164.0.0";
const PHOTO_MATERIAL_US: u64 = 10_800_000_000;

#[derive(Debug, Clone, Serialize)]
pub struct PlanVideo {
    pub path: PathBuf,
    pub kind: MediaKind,
    /// Source length (videos); photos use CapCut's 3 h still length.
    pub file_dur_us: u64,
    pub width: u32,
    pub height: u32,
    pub start_us: u64,
    pub dur_us: u64,
    pub src_start_us: u64,
    pub volume: f64,
    pub ken_burns: Option<KenBurns>,
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub opacity: f64,
    pub fade_in_us: u64,
    pub fade_out_us: u64,
    pub transition: Option<String>,
    pub transition_sec: f64,
    pub lut: Option<String>,
    pub video_effect: Option<String>,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KenBurns {
    pub zoom_start: f64,
    pub zoom_end: f64,
    pub pan: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanAudio {
    pub path: PathBuf,
    pub file_dur_us: u64,
    pub start_us: u64,
    pub dur_us: u64,
    pub src_start_us: u64,
    pub volume: f64,
    pub fade_in_us: u64,
    pub fade_out_us: u64,
    /// (offset inside the segment, volume) pairs.
    pub volume_keys: Vec<(u64, f64)>,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioTrack {
    pub label: String,
    pub clips: Vec<PlanAudio>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftPlan {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub main: Vec<PlanVideo>,
    pub overlay_tracks: Vec<Vec<PlanVideo>>,
    pub audio_tracks: Vec<AudioTrack>,
}

impl DraftPlan {
    pub fn duration_us(&self) -> u64 {
        let v = self.main.iter().chain(self.overlay_tracks.iter().flatten()).map(|c| c.start_us + c.dur_us);
        let a = self.audio_tracks.iter().flat_map(|t| t.clips.iter()).map(|c| c.start_us + c.dur_us);
        v.chain(a).max().unwrap_or(0)
    }

    pub fn media_paths(&self) -> Vec<&Path> {
        let v = self.main.iter().chain(self.overlay_tracks.iter().flatten()).map(|c| c.path.as_path());
        v.chain(self.audio_tracks.iter().flat_map(|t| t.clips.iter()).map(|c| c.path.as_path())).collect()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftResult {
    pub draft_dir: String,
    pub draft_id: String,
    pub duration_sec: f64,
    pub registered: bool,
    pub replaced_previous: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Default)]
struct Materials {
    videos: Vec<Value>,
    audios: Vec<Value>,
    speeds: Vec<Value>,
    canvases: Vec<Value>,
    sound_channel_mappings: Vec<Value>,
    audio_fades: Vec<Value>,
}

fn keyframes(property: &str, points: &[(u64, f64)]) -> Value {
    let list: Vec<Value> = points
        .iter()
        .map(|(t, v)| {
            json!({
                "id": new_id(), "curveType": "Line", "time_offset": t,
                "left_control": {"x": 0.0, "y": 0.0}, "right_control": {"x": 0.0, "y": 0.0},
                "values": [v], "string_value": "", "graphID": ""
            })
        })
        .collect();
    json!({"id": new_id(), "material_id": "", "property_type": property, "keyframe_list": list})
}

/// Opacity ramp for fades; empty when there is no fade.
fn alpha_keys(dur: u64, opacity: f64, fade_in: u64, fade_out: u64) -> Vec<(u64, f64)> {
    if fade_in == 0 && fade_out == 0 {
        return vec![];
    }
    let fade_in = fade_in.min(dur / 2);
    let fade_out = fade_out.min(dur / 2);
    let mut k = vec![];
    if fade_in > 0 {
        k.push((0, 0.0));
        k.push((fade_in, opacity));
    } else {
        k.push((0, opacity));
    }
    if fade_out > 0 {
        k.push((dur - fade_out, opacity));
        k.push((dur, 0.0));
    } else {
        k.push((dur, opacity));
    }
    k
}

fn video_segment(m: &mut Materials, c: &PlanVideo, track_index: usize) -> Value {
    let mat_id = new_id();
    let speed_id = new_id();
    let canvas_id = new_id();
    let sound_id = new_id();
    let is_video = c.kind == MediaKind::Video;
    let name = c.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    m.videos.push(json!({
        "id": mat_id,
        "type": if is_video { "video" } else { "photo" },
        "path": capcut_path(&c.path),
        "material_name": name,
        "duration": if is_video { c.file_dur_us.max(c.src_start_us + c.dur_us) } else { PHOTO_MATERIAL_US },
        "width": c.width,
        "height": c.height,
        "has_audio": is_video,
        "category_id": "",
        "category_name": "local",
        "crop": {"upper_left_x": 0.0, "upper_left_y": 0.0, "upper_right_x": 1.0, "upper_right_y": 0.0,
                 "lower_left_x": 0.0, "lower_left_y": 1.0, "lower_right_x": 1.0, "lower_right_y": 1.0},
        "crop_ratio": "free",
        "crop_scale": 1.0
    }));
    m.speeds.push(json!({"id": speed_id, "type": "speed", "mode": 0, "speed": 1.0, "curve_speed": null}));
    m.canvases.push(json!({"id": canvas_id, "type": "canvas_color", "color": "", "blur": 0.0, "image": ""}));
    m.sound_channel_mappings.push(json!({"id": sound_id, "type": "none", "audio_channel_mapping": 0}));

    let mut kfs = vec![];
    let mut scale = c.scale;
    let (mut x, y) = (c.x, c.y);
    if let Some(kb) = &c.ken_burns {
        let (mut z0, mut z1) = (kb.zoom_start, kb.zoom_end);
        let pan = kb.pan.as_str();
        if pan != "none" && !pan.is_empty() {
            // A drifting picture needs headroom on both sides, or its edge shows.
            z0 = z0.max(1.08);
            z1 = z1.max(z0);
            let d = (z0 - 1.0) * 0.85;
            let (axis, from, to) = match pan {
                "left" => ("KFTypePositionX", d, -d),
                "right" => ("KFTypePositionX", -d, d),
                "up" => ("KFTypePositionY", -d, d),
                _ => ("KFTypePositionY", d, -d),
            };
            kfs.push(keyframes(axis, &[(0, from), (c.dur_us, to)]));
            if axis == "KFTypePositionX" {
                x = from;
            }
        }
        kfs.push(keyframes("KFTypeScaleX", &[(0, z0), (c.dur_us, z1)]));
        kfs.push(keyframes("KFTypeScaleY", &[(0, z0), (c.dur_us, z1)]));
        scale = z0;
    }
    let ak = alpha_keys(c.dur_us, c.opacity, c.fade_in_us, c.fade_out_us);
    if !ak.is_empty() {
        kfs.push(keyframes("KFTypeAlpha", &ak));
    }
    let src_dur = c.dur_us;
    json!({
        "id": new_id(),
        "material_id": mat_id,
        "source_timerange": {"start": c.src_start_us, "duration": src_dur},
        "target_timerange": {"start": c.start_us, "duration": c.dur_us},
        "render_timerange": {"start": 0, "duration": 0},
        "desc": c.desc,
        "speed": 1.0,
        "volume": c.volume,
        "last_nonzero_volume": 1.0,
        "visible": true,
        "clip": {
            "scale": {"x": scale, "y": scale},
            "rotation": 0.0,
            "transform": {"x": x, "y": y},
            "flip": {"vertical": false, "horizontal": false},
            "alpha": c.opacity
        },
        "uniform_scale": {"on": true, "value": 1.0},
        "render_index": track_index,
        "track_render_index": track_index,
        "track_attribute": 0,
        "common_keyframes": kfs,
        "keyframe_refs": [],
        "extra_material_refs": [speed_id, canvas_id, sound_id]
    })
}

fn audio_segment(m: &mut Materials, c: &PlanAudio) -> Value {
    let mat_id = new_id();
    let speed_id = new_id();
    let sound_id = new_id();
    let name = c.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    m.audios.push(json!({
        "id": mat_id, "type": "extract_music", "name": name, "path": capcut_path(&c.path),
        "duration": c.file_dur_us.max(c.src_start_us + c.dur_us),
        "app_id": 0, "category_id": "", "category_name": "local", "check_flag": 1,
        "intensifies_path": "", "is_text_edit_overdub": false, "is_ugc": false, "music_id": "",
        "query": "", "request_id": "", "resource_id": "", "search_id": "", "source_platform": 0,
        "team_id": "", "text_id": "", "tone_category_id": "", "tone_category_name": "",
        "tone_effect_id": "", "tone_effect_name": "", "tone_speaker": "", "tone_type": "",
        "video_id": "", "wave_points": []
    }));
    m.speeds.push(json!({"id": speed_id, "type": "speed", "mode": 0, "speed": 1.0, "curve_speed": null}));
    m.sound_channel_mappings.push(json!({"id": sound_id, "type": "none", "audio_channel_mapping": 0}));
    let mut refs = vec![json!(speed_id), json!(sound_id)];
    if c.fade_in_us > 0 || c.fade_out_us > 0 {
        let fade_id = new_id();
        m.audio_fades.push(json!({
            "id": fade_id, "type": "audio_fade", "fade_type": 0,
            "fade_in_duration": c.fade_in_us.min(c.dur_us / 2),
            "fade_out_duration": c.fade_out_us.min(c.dur_us / 2)
        }));
        refs.push(json!(fade_id));
    }
    let kfs = if c.volume_keys.len() >= 2 { vec![keyframes("KFTypeVolume", &c.volume_keys)] } else { vec![] };
    json!({
        "id": new_id(),
        "material_id": mat_id,
        "source_timerange": {"start": c.src_start_us, "duration": c.dur_us},
        "target_timerange": {"start": c.start_us, "duration": c.dur_us},
        "render_timerange": {"start": 0, "duration": 0},
        "desc": c.desc,
        "speed": 1.0,
        "volume": c.volume,
        "last_nonzero_volume": if c.volume > 0.0 { c.volume } else { 1.0 },
        "visible": true,
        "render_index": 0,
        "track_render_index": 0,
        "track_attribute": 0,
        "common_keyframes": kfs,
        "keyframe_refs": [],
        "extra_material_refs": refs
    })
}

fn track(kind: &str, flag: u32, name: &str, segments: Vec<Value>) -> Value {
    json!({"id": new_id(), "type": kind, "flag": flag, "attribute": 0, "name": name,
           "is_default_name": name.is_empty(), "segments": segments})
}

/// The `draft_content.json` document for `plan`.
pub fn draft_content(plan: &DraftPlan, draft_id: &str) -> Value {
    let mut m = Materials::default();
    let mut tracks = vec![];
    let main: Vec<Value> = plan.main.iter().map(|c| video_segment(&mut m, c, 0)).collect();
    tracks.push(track("video", 0, "", main));
    for (i, ov) in plan.overlay_tracks.iter().enumerate() {
        let segs: Vec<Value> = ov.iter().map(|c| video_segment(&mut m, c, i + 1)).collect();
        if !segs.is_empty() {
            tracks.push(track("video", 2, "", segs));
        }
    }
    for t in &plan.audio_tracks {
        let segs: Vec<Value> = t.clips.iter().map(|c| audio_segment(&mut m, c)).collect();
        if !segs.is_empty() {
            tracks.push(track("audio", 0, &t.label, segs));
        }
    }
    let now = chrono::Utc::now().timestamp();
    json!({
        "id": draft_id,
        "name": plan.name,
        "version": 360000,
        "new_version": NEW_VERSION,
        "duration": plan.duration_us(),
        "create_time": now,
        "update_time": now,
        "fps": plan.fps,
        "canvas_config": {"width": plan.width, "height": plan.height, "ratio": "original"},
        "config": {"video_mute": false, "maintrack_adsorb": true, "subtitle_sync": true, "lyrics_sync": true},
        "materials": {
            "videos": m.videos,
            "audios": m.audios,
            "speeds": m.speeds,
            "canvases": m.canvases,
            "sound_channel_mappings": m.sound_channel_mappings,
            "audio_fades": m.audio_fades,
            "texts": [], "stickers": [], "effects": [], "transitions": []
        },
        "keyframes": {"adjusts": [], "audios": [], "effects": [], "filters": [], "handwrites": [], "stickers": [], "texts": [], "videos": []},
        "keyframe_graph_list": [],
        "tracks": tracks,
        "platform": {"os": "windows", "app_source": "cc", "app_version": "9.6.0"},
        "last_modified_platform": {"os": "windows", "app_source": "cc", "app_version": "9.6.0"}
    })
}

/// Write the draft into CapCut's folder (or `output_dir`) and register it on CapCut's home screen.
pub fn write_draft(plan: &DraftPlan, output_dir: Option<&str>, register: bool, cover_from: Option<&Path>) -> Result<DraftResult> {
    let root = capcut_root();
    let dir = match output_dir {
        Some(o) if !o.trim().is_empty() => PathBuf::from(o),
        _ => root.join(&plan.name),
    };
    let mut warnings = vec![];
    let replaced = retire_existing(&dir)?;
    fs::create_dir_all(&dir).with_context(|| format!("không tạo được thư mục draft {}", dir.display()))?;

    let draft_id = new_id();
    let content = draft_content(plan, &draft_id);
    let text = serde_json::to_string(&content)?;
    for f in ["draft_content.json", "draft_content.json.bak", "draft_info.json", "template-2.tmp"] {
        fs::write(dir.join(f), &text).with_context(|| format!("ghi {f}"))?;
    }
    fs::write(dir.join("draft.extra"), b"i\x04\x02\x04i\x04\x02\x04ei\x04\x02\x04\x01\x02\x01{}\x01e\x00")?;
    fs::write(dir.join("draft_virtual_store.json"), r#"{"draft_materials":[],"draft_virtual_store":[]}"#)?;
    let layout = json!({"dockItems": [{"dockIndex": 0, "ratio": 1, "timelineIds": [draft_id], "timelineNames": ["Timeline 01"]}], "layoutOrientation": 1});
    fs::write(dir.join("timeline_layout.json"), layout.to_string())?;
    let now_s = chrono::Utc::now().timestamp();
    let now_us = chrono::Utc::now().timestamp_micros();
    fs::write(
        dir.join("draft_settings"),
        format!("[General]\ndraft_create_time={now_s}\ndraft_last_edit_time={now_s}\nreal_edit_seconds=0\nreal_edit_keys=1\ncloud_last_modify_platform=windows\n"),
    )?;

    let cover = dir.join("draft_cover.jpg");
    if let Some(src) = cover_from {
        if let Err(e) = fs::copy(src, &cover) {
            warnings.push(format!("không tạo được ảnh bìa: {e}"));
        }
    }

    let size: u64 = plan.media_paths().iter().filter_map(|p| fs::metadata(p).ok()).map(|m| m.len()).sum();
    let duration = plan.duration_us();
    let fold = capcut_path(&dir);
    let root_s = capcut_path(&root);
    fs::write(dir.join("draft_meta_info.json"), serde_json::to_string_pretty(&meta_info(plan, &draft_id, &fold, &root_s, size, duration, now_us))?)?;

    let mut registered = false;
    if register {
        match register_in_root(&root, plan, &draft_id, &fold, &root_s, size, duration, now_us) {
            Ok(()) => registered = true,
            Err(e) => warnings.push(format!("chưa đăng ký được vào danh sách CapCut: {e:#}")),
        }
    }
    Ok(DraftResult {
        draft_dir: fold,
        draft_id,
        duration_sec: duration as f64 / 1e6,
        registered,
        replaced_previous: replaced,
        warnings,
    })
}

/// A draft with this name already exists: CapCut keeps its live copy in `Timelines/`, so writing
/// over it would be ignored. Move the old folder to a backup next to CapCut's projects instead.
fn retire_existing(dir: &Path) -> Result<Option<String>> {
    if !dir.exists() {
        return Ok(None);
    }
    let name = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "draft".into());
    let parent = dir.parent().ok_or_else(|| anyhow!("thư mục draft không hợp lệ"))?;
    let backups = parent.parent().unwrap_or(parent).join("capcut_studio_backups");
    fs::create_dir_all(&backups)?;
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let target = backups.join(format!("{name} @{stamp}"));
    fs::rename(dir, &target).with_context(|| {
        format!("không dời được bản cũ của \"{name}\" (CapCut có đang mở dự án này không?)")
    })?;
    prune_backups(&backups, &name, 3);
    Ok(Some(capcut_path(&target)))
}

fn prune_backups(backups: &Path, name: &str, keep: usize) {
    let prefix = format!("{name} @");
    let mut olds: Vec<PathBuf> = fs::read_dir(backups)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().map(|n| n.to_string_lossy().starts_with(&prefix)).unwrap_or(false))
        .collect();
    olds.sort();
    while olds.len() > keep {
        let p = olds.remove(0);
        let _ = fs::remove_dir_all(p);
    }
}

#[allow(clippy::too_many_arguments)]
fn meta_info(plan: &DraftPlan, id: &str, fold: &str, root: &str, size: u64, duration: u64, now_us: i64) -> Value {
    json!({
        "cloud_draft_cover": false, "cloud_draft_sync": false, "cloud_package_completed_time": "",
        "draft_cloud_capcut_purchase_info": "", "draft_cloud_last_action_download": false,
        "draft_cloud_package_type": "", "draft_cloud_purchase_info": "", "draft_cloud_template_id": "",
        "draft_cloud_tutorial_info": "", "draft_cloud_videocut_purchase_info": "",
        "draft_cover": "draft_cover.jpg", "draft_deeplink_url": "",
        "draft_enterprise_info": {"draft_enterprise_extra": "", "draft_enterprise_id": "", "draft_enterprise_name": "", "enterprise_material": []},
        "draft_fold_path": fold, "draft_has_unfinished_aigc_video_effect": false, "draft_id": id,
        "draft_is_ae_produce": false, "draft_is_ai_packaging_used": false, "draft_is_ai_shorts": false,
        "draft_is_ai_translate": false, "draft_is_article_video_draft": false, "draft_is_cloud_temp_draft": false,
        "draft_is_from_deeplink": "false", "draft_is_infinite_canvas_draft": false, "draft_is_invisible": false,
        "draft_is_pippit_draft": false, "draft_is_web_article_video": false, "draft_materials": [],
        "draft_materials_copied_info": [], "draft_name": plan.name, "draft_need_rename_folder": false,
        "draft_new_version": NEW_VERSION, "draft_removable_storage_device": "", "draft_root_path": root,
        "draft_segment_extra_info": [], "draft_timeline_materials_size_": size, "draft_type": "",
        "draft_web_article_video_enter_from": "", "pippit_avatar_url": "", "pippit_extra_info": "",
        "pippit_id": "", "pippit_user_name": "", "tm_draft_cloud_completed": "", "tm_draft_cloud_entry_id": 0,
        "tm_draft_cloud_modified": 0, "tm_draft_cloud_parent_entry_id": 0, "tm_draft_cloud_space_id": 0,
        "tm_draft_cloud_user_id": 0, "tm_draft_create": now_us, "tm_draft_modified": now_us,
        "tm_draft_removed": 0, "tm_duration": duration
    })
}

#[allow(clippy::too_many_arguments)]
fn register_in_root(root: &Path, plan: &DraftPlan, id: &str, fold: &str, root_s: &str, size: u64, duration: u64, now_us: i64) -> Result<()> {
    let file = root.join("root_meta_info.json");
    let mut doc: Value = if file.exists() {
        let raw = fs::read_to_string(&file)?;
        let _ = fs::write(root.join("root_meta_info.json.studio.bak"), &raw);
        serde_json::from_str(&raw).context("root_meta_info.json không đọc được")?
    } else {
        fs::create_dir_all(root)?;
        json!({"all_draft_store": [], "draft_ids": 0, "root_path": root_s})
    };
    let store = doc
        .get_mut("all_draft_store")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("root_meta_info.json thiếu all_draft_store"))?;
    store.retain(|d| d.get("draft_name").and_then(Value::as_str) != Some(plan.name.as_str()) && d.get("draft_fold_path").and_then(Value::as_str) != Some(fold));
    store.insert(
        0,
        json!({
            "cloud_draft_cover": false, "cloud_draft_sync": false, "draft_cloud_last_action_download": false,
            "draft_cloud_purchase_info": "", "draft_cloud_template_id": "", "draft_cloud_tutorial_info": "",
            "draft_cloud_videocut_purchase_info": "", "draft_cover": format!("{fold}/draft_cover.jpg"),
            "draft_fold_path": fold, "draft_id": id, "draft_is_ai_shorts": false, "draft_is_cloud_temp_draft": false,
            "draft_is_infinite_canvas_draft": false, "draft_is_invisible": false, "draft_is_pippit_draft": false,
            "draft_is_web_article_video": false, "draft_json_file": format!("{fold}/draft_content.json"),
            "draft_name": plan.name, "draft_new_version": NEW_VERSION, "draft_root_path": root_s,
            "draft_timeline_materials_size": size, "draft_type": "", "draft_web_article_video_enter_from": "",
            "pippit_avatar_url": "", "pippit_extra_info": "", "pippit_id": "", "pippit_user_name": "",
            "streaming_edit_draft_ready": true, "tm_draft_cloud_completed": "", "tm_draft_cloud_entry_id": 0,
            "tm_draft_cloud_modified": 0, "tm_draft_cloud_parent_entry_id": 0, "tm_draft_cloud_space_id": 0,
            "tm_draft_cloud_user_id": 0, "tm_draft_create": now_us, "tm_draft_modified": now_us,
            "tm_draft_removed": 0, "tm_duration": duration
        }),
    );
    let tmp = root.join("root_meta_info.json.tmp");
    fs::write(&tmp, serde_json::to_string(&doc)?)?;
    fs::rename(&tmp, &file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn photo(start: u64, dur: u64) -> PlanVideo {
        PlanVideo {
            path: PathBuf::from("E:/a.png"),
            kind: MediaKind::Image,
            file_dur_us: 0,
            width: 1920,
            height: 1080,
            start_us: start,
            dur_us: dur,
            src_start_us: 0,
            volume: 0.0,
            ken_burns: Some(KenBurns { zoom_start: 1.0, zoom_end: 1.06, pan: "none".into() }),
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            opacity: 1.0,
            fade_in_us: 0,
            fade_out_us: 0,
            transition: None,
            transition_sec: 0.45,
            lut: None,
            video_effect: None,
            desc: String::new(),
        }
    }

    #[test]
    fn overlay_tracks_get_flag_2_and_render_index() {
        let mut ov = photo(1_000_000, 2_000_000);
        ov.ken_burns = None;
        ov.fade_in_us = 300_000;
        let plan = DraftPlan {
            name: "t".into(),
            width: 1920,
            height: 1080,
            fps: 30.0,
            main: vec![photo(0, 5_000_000)],
            overlay_tracks: vec![vec![ov]],
            audio_tracks: vec![],
        };
        let c = draft_content(&plan, "ID");
        let tracks = c["tracks"].as_array().unwrap();
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[1]["flag"], 2);
        assert_eq!(tracks[1]["segments"][0]["render_index"], 1);
        assert_eq!(tracks[1]["segments"][0]["common_keyframes"][0]["property_type"], "KFTypeAlpha");
        assert_eq!(c["duration"], 5_000_000);
        assert_eq!(tracks[0]["segments"][0]["common_keyframes"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn alpha_keys_shape() {
        assert!(alpha_keys(1000, 1.0, 0, 0).is_empty());
        assert_eq!(alpha_keys(1000, 0.8, 100, 200), vec![(0, 0.0), (100, 0.8), (800, 0.8), (1000, 0.0)]);
    }
}
