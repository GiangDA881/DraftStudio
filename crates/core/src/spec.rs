//! The project spec: what the studio UI edits, what `create --spec` reads and what agents send
//! over MCP. Field aliases keep v1 specs (`video_clips`, `bgm_tracks`) working.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSpec {
    pub project_name: String,
    #[serde(default = "d_width")]
    pub width: u32,
    #[serde(default = "d_height")]
    pub height: u32,
    #[serde(default = "d_fps")]
    pub fps: f64,
    /// Narrated blocks, laid out first and back to back: each binds its scenes to one voiceover
    /// and fits their lengths to it (`build::fit_section`). `scenes` and `voiceovers` below
    /// follow after the last section.
    #[serde(default)]
    pub sections: Vec<SectionInput>,
    /// Main track, played back to back.
    #[serde(default, alias = "video_clips")]
    pub scenes: Vec<SceneInput>,
    /// Graphics and motion on overlay tracks above the main track.
    #[serde(default)]
    pub overlays: Vec<OverlayInput>,
    #[serde(default)]
    pub voiceovers: Vec<AudioInput>,
    #[serde(default, alias = "bgm_tracks")]
    pub bgm: Vec<BgmInput>,
    /// Stretch photo scenes without a duration so the pictures end with the narration.
    #[serde(default)]
    pub fit_scenes_to_voice: bool,
    #[serde(default = "d_true")]
    pub register_to_capcut: bool,
    /// Draft folder; default `<CapCut root>/<project_name>`.
    #[serde(default)]
    pub output_dir: Option<String>,
    /// Also render a low-res MP4 preview of the cut with FilmCraft.
    #[serde(default)]
    pub film_preview: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SceneInput {
    pub path: String,
    #[serde(default)]
    pub duration_sec: Option<f64>,
    #[serde(default)]
    pub duration_us: Option<u64>,
    /// Skip this much of a video's start.
    #[serde(default)]
    pub trim_start_sec: Option<f64>,
    #[serde(default)]
    pub ken_burns: bool,
    #[serde(default = "d_zoom_start")]
    pub zoom_start: f64,
    #[serde(default = "d_zoom_end")]
    pub zoom_end: f64,
    /// Drift during the Ken Burns zoom: none | left | right | up | down.
    #[serde(default)]
    pub pan: Option<String>,
    /// Clip audio (default 1.0 for video, 0 for photos).
    #[serde(default)]
    pub volume: Option<f64>,
    /// Optional FilmCraft video transition into this scene (e.g. "cross_zoom", "whip", "flash", "glitch", "light_leak").
    #[serde(default)]
    pub transition: Option<String>,
    /// Transition duration in seconds (default 0.45).
    #[serde(default)]
    pub transition_sec: Option<f64>,
    /// Optional FilmCraft LUT (e.g. "builtin:look-teal-orange", "builtin:look-golden-hour", "builtin:look-night").
    #[serde(default)]
    pub lut: Option<String>,
    /// Optional FilmCraft video effect on this scene (e.g. "wonder_glow", "rgb_split", "camera_shake", "glint").
    #[serde(default)]
    pub video_effect: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionInput {
    /// The narration. The section starts where the previous one ends, so `start_sec` is not
    /// used here (`gap_after_sec` makes a pause).
    pub voice: AudioInput,
    /// Shown back to back for the length of the voice plus the gap. Without `duration_sec`,
    /// photos share the slack and videos are trimmed proportionally, never stretched.
    #[serde(default)]
    pub scenes: Vec<SceneInput>,
    /// Silence after the voice before the next section starts; the pictures cover it.
    #[serde(default)]
    pub gap_after_sec: f64,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayInput {
    /// file: an image/video as is · graphic: VectorCraft draws it · motion: EffectCraft animates
    /// the graphic (or `path`) into an alpha video.
    #[serde(default = "d_kind")]
    pub kind: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub graphic: Option<GraphicSpec>,
    #[serde(default)]
    pub motion: Option<MotionSpec>,
    #[serde(default)]
    pub start_sec: f64,
    /// Index (from 0) into `sections`: `start_sec` then counts from that section's start.
    #[serde(default)]
    pub section: Option<usize>,
    #[serde(default)]
    pub duration_sec: Option<f64>,
    /// Offset from the centre in half-canvas units (CapCut's own unit); y up is positive.
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "d_one")]
    pub scale: f64,
    #[serde(default = "d_one")]
    pub opacity: f64,
    /// CapCut opacity keyframes (graphic/file overlays; motion overlays animate themselves).
    #[serde(default)]
    pub fade_in_sec: f64,
    #[serde(default)]
    pub fade_out_sec: f64,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphicSpec {
    /// Template id from `graphics::templates()`.
    pub template: String,
    #[serde(default)]
    pub fields: BTreeMap<String, String>,
    /// Accent colour `#RRGGBB`.
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub font: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MotionSpec {
    /// Preset id from `crafts::effect::presets()`.
    pub preset: String,
    /// prores (ProRes 4444 .mov, default) or webm (VP9 alpha).
    #[serde(default)]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioInput {
    pub path: String,
    /// Default: right after the previous voiceover.
    #[serde(default)]
    pub start_sec: Option<f64>,
    #[serde(default)]
    pub duration_sec: Option<f64>,
    #[serde(default)]
    pub duration_us: Option<u64>,
    #[serde(default = "d_one")]
    pub volume: f64,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgmInput {
    pub path: String,
    #[serde(default = "d_bgm_volume")]
    pub volume: f64,
    #[serde(default)]
    pub start_sec: f64,
    #[serde(default)]
    pub duration_sec: Option<f64>,
    #[serde(default)]
    pub duration_us: Option<u64>,
    #[serde(default = "d_fade_in")]
    pub fade_in_sec: f64,
    #[serde(default = "d_fade_out")]
    pub fade_out_sec: f64,
    /// Repeat the track until the timeline ends.
    #[serde(default = "d_true")]
    pub loop_to_fill: bool,
    /// Lower the music while a voiceover plays (CapCut volume keyframes).
    #[serde(default = "d_true")]
    pub duck_under_voice: bool,
    /// Music level under the voice, as a fraction of `volume`.
    #[serde(default = "d_duck")]
    pub duck_ratio: f64,
    #[serde(default)]
    pub desc: Option<String>,
}

fn d_width() -> u32 {
    1920
}
fn d_height() -> u32 {
    1080
}
fn d_fps() -> f64 {
    30.0
}
fn d_true() -> bool {
    true
}
fn d_zoom_start() -> f64 {
    1.0
}
fn d_zoom_end() -> f64 {
    1.06
}
fn d_kind() -> String {
    "file".into()
}
fn d_one() -> f64 {
    1.0
}
fn d_bgm_volume() -> f64 {
    0.3
}
fn d_fade_in() -> f64 {
    1.0
}
fn d_fade_out() -> f64 {
    2.0
}
fn d_duck() -> f64 {
    0.4
}

impl SectionInput {
    /// "Đoạn 3 (SC03_adler)" for messages: the section's desc, else the voice's, else its file.
    pub fn label(&self, idx: usize) -> String {
        let name = self.desc.clone().or_else(|| self.voice.desc.clone()).or_else(|| std::path::Path::new(&self.voice.path).file_stem().map(|s| s.to_string_lossy().to_string()));
        match name.filter(|n| !n.trim().is_empty()) {
            Some(n) => format!("Đoạn {} ({n})", idx + 1),
            None => format!("Đoạn {}", idx + 1),
        }
    }
}

impl ProjectSpec {
    pub fn new(name: &str) -> Self {
        ProjectSpec {
            project_name: name.into(),
            width: d_width(),
            height: d_height(),
            fps: d_fps(),
            sections: vec![],
            scenes: vec![],
            overlays: vec![],
            voiceovers: vec![],
            bgm: vec![],
            fit_scenes_to_voice: false,
            register_to_capcut: true,
            output_dir: None,
            film_preview: false,
        }
    }

    /// Problems that stop a build, in the user's language.
    pub fn validate(&self) -> Vec<String> {
        let mut errs = vec![];
        if self.project_name.trim().is_empty() {
            errs.push("Chưa đặt tên dự án.".into());
        }
        if self.project_name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
            errs.push("Tên dự án không được chứa / \\ : * ? \" < > |".into());
        }
        if self.width < 16 || self.height < 16 || self.width > 8192 || self.height > 8192 {
            errs.push(format!("Khung hình {}x{} không hợp lệ.", self.width, self.height));
        }
        if !(1.0..=240.0).contains(&self.fps) {
            errs.push(format!("FPS {} không hợp lệ.", self.fps));
        }
        if self.sections.is_empty() && self.scenes.is_empty() && self.overlays.is_empty() && self.voiceovers.is_empty() {
            errs.push("Dự án trống: thêm ít nhất một đoạn, cảnh, overlay hoặc giọng đọc.".into());
        }
        let check = |errs: &mut Vec<String>, what: &str, p: &str| {
            if p.trim().is_empty() {
                errs.push(format!("{what}: thiếu đường dẫn file."));
            } else if !std::path::Path::new(p).exists() {
                errs.push(format!("{what}: không tìm thấy file {p}"));
            }
        };
        for (i, sec) in self.sections.iter().enumerate() {
            let what = sec.label(i);
            check(&mut errs, &format!("{what} · giọng đọc"), &sec.voice.path);
            if sec.voice.start_sec.is_some() {
                errs.push(format!("{what}: giọng đọc trong đoạn không dùng start_sec (đoạn tự nối tiếp đoạn trước); muốn có khoảng lặng thì đặt gap_after_sec."));
            }
            if !sec.gap_after_sec.is_finite() || sec.gap_after_sec < 0.0 {
                errs.push(format!("{what}: gap_after_sec phải ≥ 0."));
            }
            if sec.scenes.is_empty() {
                errs.push(format!("{what}: chưa có cảnh nào."));
            }
            for (k, s) in sec.scenes.iter().enumerate() {
                check(&mut errs, &format!("{what} · cảnh {}", k + 1), &s.path);
            }
        }
        for (i, s) in self.scenes.iter().enumerate() {
            check(&mut errs, &format!("Cảnh {}", i + 1), &s.path);
        }
        for (i, a) in self.voiceovers.iter().enumerate() {
            check(&mut errs, &format!("Giọng đọc {}", i + 1), &a.path);
        }
        for (i, b) in self.bgm.iter().enumerate() {
            check(&mut errs, &format!("Nhạc nền {}", i + 1), &b.path);
        }
        for (i, o) in self.overlays.iter().enumerate() {
            let what = format!("Overlay {}", i + 1);
            if let Some(s) = o.section.filter(|s| *s >= self.sections.len()) {
                errs.push(format!("{what}: section {s} không có (dự án có {} đoạn, đếm từ 0).", self.sections.len()));
            }
            match o.kind.as_str() {
                "file" => check(&mut errs, &what, o.path.as_deref().unwrap_or("")),
                "graphic" => {
                    if o.graphic.is_none() {
                        errs.push(format!("{what}: thiếu mẫu đồ họa."));
                    }
                }
                "motion" => {
                    if o.motion.is_none() {
                        errs.push(format!("{what}: thiếu preset chuyển động."));
                    }
                    match (&o.graphic, &o.path) {
                        (None, None) => errs.push(format!("{what}: cần đồ họa hoặc ảnh để làm chuyển động.")),
                        (None, Some(p)) => check(&mut errs, &what, p),
                        _ => {}
                    }
                }
                k => errs.push(format!("{what}: loại '{k}' không hỗ trợ (file | graphic | motion).")),
            }
        }
        errs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_parse_and_validate() {
        let spec: ProjectSpec = serde_json::from_value(serde_json::json!({
            "project_name": "t",
            "sections": [
                {"voice": {"path": "nope/a.mp3", "desc": "SC01"}, "scenes": [{"path": "nope/x.mp4"}], "gap_after_sec": 0.5},
                {"voice": {"path": "nope/b.mp3", "start_sec": 3.0}, "scenes": [], "gap_after_sec": -1}
            ],
            "overlays": [{"kind": "file", "path": "nope/o.png", "section": 2, "start_sec": 0.4}]
        }))
        .unwrap();
        assert_eq!(spec.sections[0].gap_after_sec, 0.5);
        assert_eq!(spec.sections[0].voice.volume, 1.0);
        assert_eq!(spec.sections[0].label(0), "Đoạn 1 (SC01)");
        assert_eq!(spec.sections[1].label(1), "Đoạn 2 (b)");
        let errs = spec.validate().join("\n");
        for want in ["Đoạn 1 (SC01) · cảnh 1: không tìm thấy", "Đoạn 2 (b): giọng đọc trong đoạn không dùng start_sec", "Đoạn 2 (b): gap_after_sec", "Đoạn 2 (b): chưa có cảnh", "Overlay 1: section 2 không có"] {
            assert!(errs.contains(want), "missing {want:?} in\n{errs}");
        }
        // v1/v2 specs without sections still parse as before (see also public_examples_are_valid)
        let old: ProjectSpec = serde_json::from_value(serde_json::json!({"project_name": "t", "video_clips": [{"path": "a.jpg"}]})).unwrap();
        assert!(old.sections.is_empty() && old.scenes.len() == 1);
    }

    /// The examples shipped in the repository parse, and name real templates, fields and presets.
    #[test]
    fn public_examples_are_valid() {
        let examples = [("narrated_sections", include_str!("../../../examples/narrated_sections.json")), ("slideshow", include_str!("../../../examples/slideshow.json"))];
        for (name, raw) in examples {
            let spec: ProjectSpec = serde_json::from_str(raw).unwrap_or_else(|e| panic!("{name}: {e}"));
            for o in &spec.overlays {
                if let Some(g) = &o.graphic {
                    let t = crate::graphics::template(&g.template).unwrap_or_else(|| panic!("{name}: no template {}", g.template));
                    for k in g.fields.keys() {
                        assert!(t.fields.iter().any(|f| f.key == k), "{name}: {} has no field {k}", g.template);
                    }
                }
                if let Some(m) = &o.motion {
                    assert!(crate::crafts::effect::presets().iter().any(|p| p.id == m.preset), "{name}: no motion preset {}", m.preset);
                }
            }
            let errs: Vec<String> = spec.validate().into_iter().filter(|e| !e.contains("không tìm thấy file")).collect();
            assert!(errs.is_empty(), "{name}: {errs:?}");
        }
    }
}
