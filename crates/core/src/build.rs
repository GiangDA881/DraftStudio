//! Spec → crafts → timeline plan → CapCut draft (→ FilmCraft preview).

use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::crafts::{effect, film, vector};
use crate::draft::{self, AudioTrack, DraftPlan, DraftResult, KenBurns, PlanAudio, PlanVideo};
use crate::graphics;
use crate::media::{self, MediaKind};
use crate::paths::{clean_path_str, slug};
use crate::sec_to_us;
use crate::settings::Settings;
use crate::spec::{AudioInput, OverlayInput, ProjectSpec, SceneInput};

/// Progress sink: the CLI prints, the studio streams to the UI.
pub type BuildLog<'a> = &'a (dyn Fn(&str) + Sync);
/// Called as each craft file is made, from the craft worker threads.
pub type AssetSink<'a> = &'a (dyn Fn(&Asset) + Sync);

#[derive(Debug, Clone, Serialize)]
pub struct Asset {
    pub overlay: usize,
    pub craft: &'static str,
    pub kind: &'static str,
    pub path: String,
    pub editable: Option<String>,
    pub cached: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackSummary {
    pub label: String,
    pub kind: &'static str,
    pub clips: Vec<ClipSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClipSummary {
    pub name: String,
    pub path: String,
    pub start_sec: f64,
    pub dur_sec: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BuildReport {
    pub project_name: String,
    pub draft: DraftResult,
    pub tracks: Vec<TrackSummary>,
    pub assets: Vec<Asset>,
    pub preview: Option<film::PreviewOutput>,
    pub warnings: Vec<String>,
    pub seconds: f64,
    pub capcut_running: bool,
}

pub fn project_assets_dir(name: &str) -> PathBuf {
    Settings::load().assets_root().join(slug(name))
}

/// Automatically save the spec into `%APPDATA%\CapCutDraftStudio\projects\<slug>.json` so any
/// CLI or MCP agent build/live-edit immediately appears in Draft Studio's saved projects list.
pub fn save_studio_project(spec: &ProjectSpec) -> Option<String> {
    let s = slug(&spec.project_name);
    let dir = crate::paths::app_data_dir().join("projects");
    let _ = std::fs::create_dir_all(&dir);
    let body = serde_json::to_string_pretty(spec).ok()?;
    std::fs::write(dir.join(format!("{s}.json")), body).ok()?;
    Some(s)
}

/// Make the graphic (VectorCraft) and/or motion (EffectCraft) files for one overlay.
/// Returns the file CapCut should use and the assets made on the way.
pub fn resolve_overlay(spec: &ProjectSpec, idx: usize, o: &OverlayInput, dur_sec: f64, dir: &Path, log: BuildLog, on_asset: AssetSink) -> Result<(PathBuf, Vec<Asset>)> {
    let mut assets = vec![];
    let graphic_png = match &o.graphic {
        Some(g) if o.kind != "file" => {
            log(&format!("VectorCraft ▸ vẽ \"{}\" (overlay {})", g.template, idx + 1));
            let out = vector::render(g, spec.width, spec.height, &dir.join("graphics")).with_context(|| format!("overlay {}: VectorCraft", idx + 1))?;
            assets.push(Asset { overlay: idx, craft: "vectorcraft", kind: "graphic", path: out.png.clone(), editable: out.vectorcraft_doc.clone(), cached: out.cached });
            on_asset(&assets[assets.len() - 1]);
            Some(PathBuf::from(out.png))
        }
        _ => None,
    };
    match o.kind.as_str() {
        "file" => Ok((PathBuf::from(o.path.clone().unwrap_or_default()), assets)),
        "graphic" => Ok((graphic_png.context("thiếu đồ họa")?, assets)),
        "motion" => {
            let m = o.motion.clone().unwrap_or_default();
            let src = graphic_png.or_else(|| o.path.clone().map(PathBuf::from)).context("motion cần đồ họa hoặc ảnh")?;
            let fmt = m.format.as_deref().unwrap_or("prores");
            log(&format!("EffectCraft ▸ chuyển động \"{}\" {:.1}s (overlay {})", m.preset, dur_sec, idx + 1));
            let out = effect::render(&src, &m.preset, fmt, spec.width, spec.height, spec.fps, dur_sec, &dir.join("motion")).with_context(|| format!("overlay {}: EffectCraft", idx + 1))?;
            assets.push(Asset { overlay: idx, craft: "effectcraft", kind: "motion", path: out.video.clone(), editable: Some(out.project.clone()), cached: out.cached });
            on_asset(&assets[assets.len() - 1]);
            Ok((PathBuf::from(out.video), assets))
        }
        k => bail!("overlay {}: loại '{k}' không hỗ trợ", idx + 1),
    }
}

/// Craft renders are separate processes, so a few run at once (each engine also multithreads,
/// hence the small pool). Results come back in overlay order; the first failure wins.
const CRAFT_WORKERS: usize = 3;

fn resolve_overlays_parallel(spec: &ProjectSpec, dir: &Path, log: BuildLog, on_asset: AssetSink) -> Result<Vec<(PathBuf, Vec<Asset>)>> {
    let n = spec.overlays.len();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let slots: Vec<std::sync::Mutex<Option<Result<(PathBuf, Vec<Asset>)>>>> = (0..n).map(|_| std::sync::Mutex::new(None)).collect();
    std::thread::scope(|s| {
        for _ in 0..CRAFT_WORKERS.min(n.max(1)) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(o) = spec.overlays.get(i) else { break };
                let r = resolve_overlay(spec, i, o, overlay_duration(o), dir, log, on_asset);
                if let Ok(mut slot) = slots[i].lock() {
                    *slot = Some(r);
                }
            });
        }
    });
    slots
        .into_iter()
        .enumerate()
        .map(|(i, m)| m.into_inner().ok().flatten().unwrap_or_else(|| Err(anyhow::anyhow!("overlay {}: không có kết quả", i + 1))))
        .collect()
}

fn overlay_duration(o: &OverlayInput) -> f64 {
    if let Some(d) = o.duration_sec.filter(|d| *d > 0.0) {
        return d;
    }
    if let Some(g) = &o.graphic {
        if let Some(t) = graphics::template(&g.template) {
            return t.suggested_sec;
        }
    }
    if let Some(p) = &o.path {
        if media::kind_of(p) == MediaKind::Video {
            let d = media::duration_us(p);
            if d > 0 {
                return d as f64 / 1e6;
            }
        }
    }
    4.0
}

pub fn build_project(spec: &ProjectSpec, log: BuildLog) -> Result<BuildReport> {
    build_project_with(spec, log, &|_| {})
}

fn scene_fx_badge(s: &SceneInput) -> String {
    let mut parts = vec![];
    if let Some(tr) = &s.transition {
        parts.push(format!("⚡{tr}"));
    }
    if let Some(fx) = &s.video_effect {
        parts.push(format!("✨{fx}"));
    }
    if let Some(lut) = &s.lut {
        parts.push(format!("🎨{}", lut.trim_start_matches("builtin:")));
    }
    if s.ken_burns {
        parts.push("🔍Zoom".into());
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" · {}", parts.join(" · "))
    }
}

/// `build_project`, also reporting each craft file as soon as it is made (the studio's live
/// agent view fills overlays in with them).
pub fn build_project_with(spec: &ProjectSpec, log: BuildLog, on_asset: AssetSink) -> Result<BuildReport> {
    let t0 = Instant::now();
    let errs = spec.validate();
    if !errs.is_empty() {
        bail!("{}", errs.join("\n"));
    }
    let mut warnings = vec![];
    let _ = save_studio_project(spec);
    let dir = project_assets_dir(&spec.project_name);
    std::fs::create_dir_all(&dir).with_context(|| format!("không tạo được thư mục tài nguyên {}", dir.display()))?;

    // 1. Sections: each voice starts where the previous section ended and its scenes are fitted
    //    to voice + gap, so every cut between sections lands exactly on a voice start.
    log("Đo giọng đọc và cảnh…");
    let mut voice = vec![];
    let mut main = vec![];
    let mut section_starts = vec![];
    let mut t = 0u64;
    for (si, sec) in spec.sections.iter().enumerate() {
        let label = sec.label(si);
        let (info, dur) = measure_voice(&sec.voice, &label)?;
        section_starts.push(t);
        voice.push(plan_voice(&sec.voice, &info, t, dur, &label));
        let mut pre = vec![];
        let mut items = vec![];
        for (k, s) in sec.scenes.iter().enumerate() {
            let p = measure_scene(s, &format!("{label} · cảnh {}", k + 1))?;
            let video_us = match p.kind {
                MediaKind::Video if p.info.duration_us > p.trim => Some(p.info.duration_us - p.trim),
                MediaKind::Video if p.info.duration_us > 0 => bail!("{label} · cảnh {}: trim_start_sec {:.2}s vượt quá độ dài video {:.2}s ({})", k + 1, p.trim as f64 / 1e6, p.info.duration_us as f64 / 1e6, s.path),
                MediaKind::Video if p.explicit.is_none() => bail!("{label} · cảnh {}: không đo được độ dài video {}; đặt duration_sec cho cảnh này.", k + 1, s.path),
                _ => None,
            };
            items.push(FitItem { video_us, fixed_us: p.explicit });
            pre.push(p);
        }
        let (durs, w) = fit_section(&label, dur + sec_to_us(sec.gap_after_sec), &items)?;
        warnings.extend(w);
        for (k, ((s, p), d)) in sec.scenes.iter().zip(&pre).zip(durs).enumerate() {
            let sc_idx = main.len() + 1;
            let fx_info = scene_fx_badge(s);
            let fname = Path::new(&s.path).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| s.path.clone());
            log(&format!("🎬 Cảnh {sc_idx} ({label}): {fname} · {:.1}s{fx_info}", d as f64 / 1e6));
            main.push(plan_scene(spec, s, p, t, d, || format!("Đoạn {} · cảnh {}", si + 1, k + 1)));
            t += d;
        }
    }
    let sections_end = t;

    // 2. Free voiceovers: explicit starts, else back to back after the sections.
    let mut cursor = sections_end;
    for (i, a) in spec.voiceovers.iter().enumerate() {
        let (info, dur) = measure_voice(a, &format!("Giọng đọc {}", i + 1))?;
        let start = a.start_sec.map(sec_to_us).unwrap_or(cursor);
        cursor = start + dur;
        voice.push(plan_voice(a, &info, start, dur, &format!("Giọng {}", i + 1)));
    }
    let voice_end = voice.iter().map(|v| v.start_us + v.dur_us).max().unwrap_or(0);

    // 3. Free scenes, after the sections. With fit_scenes_to_voice, photos without a duration
    //    share what is left of the narration.
    let mut pre = vec![];
    for (i, s) in spec.scenes.iter().enumerate() {
        pre.push(measure_scene(s, &format!("Cảnh {}", i + 1))?);
    }
    let dur_of = |p: &Scene| match (p.kind, p.explicit) {
        (_, Some(d)) => Some(d),
        (MediaKind::Video, None) if p.info.duration_us > p.trim => Some(p.info.duration_us - p.trim),
        (MediaKind::Video, None) => Some(5_000_000),
        _ => None,
    };
    let auto = pre.iter().filter(|p| dur_of(p).is_none()).count();
    let fixed: u64 = pre.iter().filter_map(dur_of).sum();
    let narration_left = voice_end.saturating_sub(sections_end);
    let auto_dur = if spec.fit_scenes_to_voice && auto > 0 && narration_left > fixed {
        (narration_left - fixed) / auto as u64
    } else {
        if spec.fit_scenes_to_voice && auto > 0 {
            warnings.push("Các cảnh có thời lượng cố định đã dài hơn giọng đọc; ảnh tự do dùng 5 giây.".into());
        }
        5_000_000
    };
    for (i, (s, p)) in spec.scenes.iter().zip(&pre).enumerate() {
        let mut dur = dur_of(p).unwrap_or(auto_dur).max(MIN_CLIP_US);
        if p.kind == MediaKind::Video && p.info.duration_us > 0 && p.trim + dur > p.info.duration_us {
            let avail = p.info.duration_us.saturating_sub(p.trim);
            warnings.push(format!("Cảnh {}: video chỉ còn {:.2}s sau điểm cắt, rút ngắn từ {:.2}s.", i + 1, avail as f64 / 1e6, dur as f64 / 1e6));
            dur = avail.max(MIN_CLIP_US);
        }
        let sc_idx = main.len() + 1;
        let fx_info = scene_fx_badge(s);
        let fname = Path::new(&s.path).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| s.path.clone());
        log(&format!("🎬 Cảnh {sc_idx}: {fname} · {:.1}s{fx_info}", dur as f64 / 1e6));
        main.push(plan_scene(spec, s, p, t, dur, || format!("Cảnh {}", i + 1)));
        t += dur;
    }
    let main_end = t;

    // 4. Overlays through the crafts, then onto as few overlay tracks as they need: footage and
    //    pictures (`file`, e.g. B-roll) below, graphics and motion above, so a cutaway that starts
    //    after a title never hides it. The studio's compute() (ui/app.js) mirrors this.
    let mut assets = vec![];
    let (mut under, mut over): (Vec<PlanVideo>, Vec<PlanVideo>) = (vec![], vec![]);
    let resolved = resolve_overlays_parallel(spec, &dir, log, on_asset)?;
    for ((i, o), (file, made)) in spec.overlays.iter().enumerate().zip(resolved) {
        let dur_sec = overlay_duration(o);
        assets.extend(made);
        let info = media::probe(&file);
        let kind = media::kind_of(&file);
        let dur = sec_to_us(dur_sec);
        let motion = o.kind == "motion";
        (if o.kind == "file" { &mut under } else { &mut over }).push(PlanVideo {
            file_dur_us: info.duration_us,
            width: if info.width > 0 { info.width } else { spec.width },
            height: if info.height > 0 { info.height } else { spec.height },
            path: file,
            kind,
            start_us: o.section.and_then(|s| section_starts.get(s)).copied().unwrap_or(0) + sec_to_us(o.start_sec),
            dur_us: if kind == MediaKind::Video && info.duration_us > 0 { dur.min(info.duration_us) } else { dur },
            src_start_us: 0,
            volume: 0.0,
            ken_burns: None,
            x: o.x,
            y: o.y,
            scale: if o.scale > 0.0 { o.scale } else { 1.0 },
            opacity: o.opacity.clamp(0.0, 1.0),
            fade_in_us: if motion { 0 } else { sec_to_us(o.fade_in_sec) },
            fade_out_us: if motion { 0 } else { sec_to_us(o.fade_out_sec) },
            transition: None,
            transition_sec: 0.45,
            lut: None,
            video_effect: None,
            desc: o.desc.clone().unwrap_or_else(|| format!("Overlay {}", i + 1)),
        });
    }
    let mut overlay_tracks = stack(under);
    overlay_tracks.extend(stack(over));
    let overlay_end = overlay_tracks.iter().flatten().map(|c| c.start_us + c.dur_us).max().unwrap_or(0);
    let total = main_end.max(voice_end).max(overlay_end);
    if main_end > 0 && voice_end > main_end + 500_000 {
        warnings.push(format!("Hình ({:.1}s) ngắn hơn giọng đọc ({:.1}s): cuối video sẽ đen. Bật \"khớp cảnh với giọng\", gom cảnh vào đoạn (sections) hoặc thêm cảnh.", main_end as f64 / 1e6, voice_end as f64 / 1e6));
    }

    // 5. Music: loop / trim to the timeline, fades, ducking under the voice.
    log("Xếp nhạc nền…");
    let mut bgm = vec![];
    for (i, b) in spec.bgm.iter().enumerate() {
        let info = media::probe(&b.path);
        let file = b.duration_us.or(b.duration_sec.map(sec_to_us)).filter(|d| *d > 0).unwrap_or(info.duration_us);
        if file == 0 {
            bail!("Nhạc nền {}: không đo được thời lượng {}", i + 1, b.path);
        }
        let start = sec_to_us(b.start_sec);
        if start >= total {
            warnings.push(format!("Nhạc nền {} bắt đầu sau khi video kết thúc, bỏ qua.", i + 1));
            continue;
        }
        let want = total - start;
        let mut pieces = vec![];
        let mut at = start;
        loop {
            let left = start + want - at;
            let d = left.min(file);
            pieces.push((at, d));
            at += d;
            if at >= start + want || !b.loop_to_fill {
                break;
            }
        }
        let n = pieces.len();
        for (k, (at, d)) in pieces.into_iter().enumerate() {
            let keys = if b.duck_under_voice && !voice.is_empty() { duck_keys(at, d, &voice, b.volume, b.volume * b.duck_ratio.clamp(0.0, 1.0)) } else { vec![] };
            bgm.push(PlanAudio {
                path: PathBuf::from(&b.path),
                file_dur_us: info.duration_us.max(file),
                start_us: at,
                dur_us: d,
                src_start_us: 0,
                volume: b.volume,
                fade_in_us: if k == 0 { sec_to_us(b.fade_in_sec) } else { 0 },
                fade_out_us: if k + 1 == n { sec_to_us(b.fade_out_sec) } else { 0 },
                volume_keys: keys,
                desc: b.desc.clone().unwrap_or_else(|| format!("Nhạc nền {}", i + 1)),
            });
        }
    }

    let mut audio_tracks = vec![];
    if !voice.is_empty() {
        audio_tracks.push(AudioTrack { label: "Giọng đọc".into(), clips: voice });
    }
    if !bgm.is_empty() {
        audio_tracks.push(AudioTrack { label: "Nhạc nền".into(), clips: bgm });
    }
    let plan = DraftPlan { name: spec.project_name.clone(), width: spec.width, height: spec.height, fps: spec.fps, main, overlay_tracks, audio_tracks };

    // 6. Cover from the first picture, else a frame of the first video.
    let cover = make_cover(&plan, &dir);
    log("Ghi draft CapCut…");
    let draft = draft::write_draft(&plan, spec.output_dir.as_deref(), spec.register_to_capcut, cover.as_deref())?;
    warnings.extend(draft.warnings.clone());

    // 7. Optional FilmCraft preview of the same cut.
    let mut preview = None;
    if spec.film_preview {
        log("FilmCraft ▸ render bản xem trước…");
        match film_preview(&plan, &dir) {
            Ok(p) => preview = Some(p),
            Err(e) => warnings.push(format!("FilmCraft preview: {e:#}")),
        }
    }
    let tracks = summarize(&plan);
    log(&format!("Xong: {} ({:.1}s)", draft.draft_dir, draft.duration_sec));
    Ok(BuildReport {
        project_name: spec.project_name.clone(),
        draft,
        tracks,
        assets,
        preview,
        warnings,
        seconds: t0.elapsed().as_secs_f64(),
        capcut_running: crate::paths::capcut_running(),
    })
}

/// Clips on as few tracks as they need, each on the first track free at its start. Later tracks
/// draw on top in CapCut.
fn stack(mut clips: Vec<PlanVideo>) -> Vec<Vec<PlanVideo>> {
    clips.sort_by_key(|c| c.start_us);
    let mut tracks: Vec<Vec<PlanVideo>> = vec![];
    for c in clips {
        match tracks.iter().position(|tr| tr.last().is_none_or(|l| l.start_us + l.dur_us <= c.start_us)) {
            Some(k) => tracks[k].push(c),
            None => tracks.push(vec![c]),
        }
    }
    tracks
}

/// Shortest clip put on the main track.
const MIN_CLIP_US: u64 = 100_000;
/// Free photos keep at least this long while a section's videos are trimmed to fit.
pub const MIN_PHOTO_US: u64 = 2_000_000;
/// A video trimmed below this to fit its section is reported as a flash cut.
const SHORT_CUT_US: u64 = 500_000;

/// One scene, measured.
struct Scene {
    kind: MediaKind,
    info: media::MediaInfo,
    trim: u64,
    /// The `duration_us` / `duration_sec` the caller set.
    explicit: Option<u64>,
}

fn measure_scene(s: &SceneInput, what: &str) -> Result<Scene> {
    let kind = media::kind_of(&s.path);
    if !matches!(kind, MediaKind::Image | MediaKind::Video) {
        bail!("{what}: {} không phải ảnh hay video", s.path);
    }
    Ok(Scene {
        kind,
        info: media::probe(&s.path),
        trim: s.trim_start_sec.map(sec_to_us).unwrap_or(0),
        explicit: s.duration_us.or(s.duration_sec.map(sec_to_us)).filter(|d| *d > 0),
    })
}

fn plan_scene(spec: &ProjectSpec, s: &SceneInput, p: &Scene, start: u64, dur: u64, desc: impl FnOnce() -> String) -> PlanVideo {
    let is_video = p.kind == MediaKind::Video;
    PlanVideo {
        path: PathBuf::from(&s.path),
        kind: p.kind,
        file_dur_us: p.info.duration_us,
        width: if p.info.width > 0 { p.info.width } else { spec.width },
        height: if p.info.height > 0 { p.info.height } else { spec.height },
        start_us: start,
        dur_us: dur,
        src_start_us: if is_video { p.trim } else { 0 },
        volume: s.volume.unwrap_or(if is_video { 1.0 } else { 0.0 }),
        ken_burns: s.ken_burns.then(|| KenBurns { zoom_start: s.zoom_start, zoom_end: s.zoom_end, pan: s.pan.clone().unwrap_or_else(|| "none".into()) }),
        x: 0.0,
        y: 0.0,
        scale: 1.0,
        opacity: 1.0,
        fade_in_us: 0,
        fade_out_us: 0,
        transition: s.transition.clone(),
        transition_sec: s.transition_sec.unwrap_or(0.45),
        lut: s.lut.clone(),
        video_effect: s.video_effect.clone(),
        desc: s.desc.clone().unwrap_or_else(desc),
    }
}

/// The voiceover's file info and length on the timeline (the caller's duration, else the file's).
fn measure_voice(a: &AudioInput, what: &str) -> Result<(media::MediaInfo, u64)> {
    let info = media::probe(&a.path);
    let dur = a.duration_us.or(a.duration_sec.map(sec_to_us)).filter(|d| *d > 0).unwrap_or(info.duration_us);
    if dur == 0 {
        bail!("{what}: không đo được thời lượng {}", a.path);
    }
    Ok((info, dur))
}

fn plan_voice(a: &AudioInput, info: &media::MediaInfo, start: u64, dur: u64, desc: &str) -> PlanAudio {
    PlanAudio {
        path: PathBuf::from(&a.path),
        file_dur_us: info.duration_us.max(dur),
        start_us: start,
        dur_us: dur,
        src_start_us: 0,
        volume: a.volume,
        fade_in_us: 0,
        fade_out_us: 0,
        volume_keys: vec![],
        desc: a.desc.clone().unwrap_or_else(|| desc.to_string()),
    }
}

/// One main-track scene of a section, as `fit_section` sees it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FitItem {
    /// Video length left after `trim_start_sec`; `None` for a photo (or a video of unknown
    /// length that has a set duration).
    pub video_us: Option<u64>,
    /// The duration the caller set: kept, except that a video is capped at its length.
    pub fixed_us: Option<u64>,
}

/// Lengths for a section's scenes, in order, adding up to exactly `target_us` (voice + gap).
/// Set durations are kept. Free photos absorb the slack, each keeping at least `MIN_PHOTO_US`
/// while there is video to trim instead. Free videos are trimmed in proportion to their length
/// and never run past it, so footage shorter than the voice is an error, not a black gap.
/// The studio's `fitSection` (ui/app.js) mirrors this.
pub fn fit_section(label: &str, target_us: u64, items: &[FitItem]) -> Result<(Vec<u64>, Vec<String>)> {
    let s = |us: u64| us as f64 / 1e6;
    if items.is_empty() {
        bail!("{label}: chưa có cảnh nào.");
    }
    let mut warns = vec![];
    let mut out = vec![0u64; items.len()];
    let (mut fixed, mut videos, mut photos) = (0u64, vec![], vec![]);
    for (i, it) in items.iter().enumerate() {
        match (it.fixed_us, it.video_us) {
            (Some(f), cap) => {
                let d = cap.map_or(f, |c| f.min(c));
                if d < f {
                    warns.push(format!("{label} · cảnh {}: video chỉ còn {:.2}s sau điểm cắt, rút ngắn từ {:.2}s.", i + 1, s(d), s(f)));
                }
                out[i] = d;
                fixed += d;
            }
            (None, Some(_)) => videos.push(i),
            (None, None) => photos.push(i),
        }
    }
    if fixed > target_us {
        bail!("{label}: các cảnh đặt sẵn duration_sec dài {:.2}s, quá giọng đọc + khoảng lặng ({:.2}s). Bỏ duration_sec hoặc bớt cảnh.", s(fixed), s(target_us));
    }
    let room = target_us - fixed;
    let caps: Vec<u64> = videos.iter().map(|&i| items[i].video_us.unwrap_or(0)).collect();
    let footage: u64 = caps.iter().sum();
    let for_videos = if photos.is_empty() {
        if footage < room {
            let have = fixed + footage;
            bail!(
                "{label}: thiếu hình — cảnh chỉ đủ {:.2}s nhưng giọng đọc + khoảng lặng dài {:.2}s (thiếu {:.2}s). Video không kéo giãn được: thêm cảnh, thêm một ảnh (ảnh tự giãn) hoặc bỏ duration_sec ở một ảnh.",
                s(have),
                s(target_us),
                s(target_us - have)
            );
        }
        room
    } else {
        let floor = MIN_PHOTO_US.min(room / (photos.len() + videos.len()) as u64) * photos.len() as u64;
        footage.min(room - floor)
    };
    for ((&i, d), cap) in videos.iter().zip(share(&caps, for_videos)).zip(&caps) {
        if d < *cap && d < SHORT_CUT_US {
            warns.push(format!("{label} · cảnh {}: video bị cắt còn {:.2}s cho khớp giọng đọc.", i + 1, s(d)));
        }
        out[i] = d;
    }
    if let Some(&last) = photos.last() {
        let rest = room - for_videos;
        let n = photos.len() as u64;
        for &i in &photos {
            out[i] = rest / n;
        }
        out[last] += rest % n;
    }
    if let Some(k) = out.iter().position(|d| *d < MIN_CLIP_US) {
        bail!("{label} · cảnh {}: chỉ còn {:.2}s ({} cảnh cho {:.2}s giọng đọc + khoảng lặng). Bớt cảnh trong đoạn này.", k + 1, s(out[k]), items.len(), s(target_us));
    }
    debug_assert_eq!(out.iter().sum::<u64>(), target_us);
    Ok((out, warns))
}

/// `total` (at most the sum of `caps`) split in proportion to `caps`, exactly, none above its cap.
fn share(caps: &[u64], total: u64) -> Vec<u64> {
    let sum: u64 = caps.iter().sum();
    if sum == 0 {
        return vec![0; caps.len()];
    }
    let mut out: Vec<u64> = caps.iter().map(|&c| (c as u128 * total as u128 / sum as u128) as u64).collect();
    // the rounding remainder is under 1 µs per clip that has room: hand it out 1 µs each from the end
    let mut left = total - out.iter().sum::<u64>();
    for (o, &c) in out.iter_mut().zip(caps).rev() {
        if left > 0 && *o < c {
            *o += 1;
            left -= 1;
        }
    }
    out
}

/// Volume keyframes inside a music segment [at, at+d): full level, dipping to `low` while any
/// voiceover plays, with short ramps.
pub fn duck_keys(at: u64, d: u64, voice: &[PlanAudio], high: f64, low: f64) -> Vec<(u64, f64)> {
    const RAMP: u64 = 300_000;
    const MERGE_GAP: u64 = 800_000;
    let end = at + d;
    let mut spans: Vec<(u64, u64)> = voice
        .iter()
        .map(|v| (v.start_us, v.start_us + v.dur_us))
        .filter(|(a, b)| *b > at && *a < end)
        .map(|(a, b)| (a.max(at) - at, b.min(end) - at))
        .collect();
    spans.sort();
    let mut merged: Vec<(u64, u64)> = vec![];
    for s in spans {
        match merged.last_mut() {
            Some(l) if s.0 <= l.1 + MERGE_GAP => l.1 = l.1.max(s.1),
            _ => merged.push(s),
        }
    }
    if merged.is_empty() {
        return vec![];
    }
    fn push(keys: &mut Vec<(u64, f64)>, d: u64, t: u64, v: f64) {
        let t = t.min(d);
        match keys.last() {
            Some((lt, _)) if *lt >= t => {}
            _ => keys.push((t, v)),
        }
    }
    let mut keys: Vec<(u64, f64)> = vec![];
    for (a, b) in merged {
        if a >= RAMP {
            push(&mut keys, d, 0, high);
            push(&mut keys, d, a - RAMP, high);
        }
        push(&mut keys, d, a, low);
        push(&mut keys, d, b, low);
        push(&mut keys, d, b + RAMP, high);
    }
    let last = keys.last().map(|k| k.1).unwrap_or(high);
    push(&mut keys, d, d, last);
    keys
}

fn make_cover(plan: &DraftPlan, dir: &Path) -> Option<PathBuf> {
    let first = plan.main.first().or_else(|| plan.overlay_tracks.iter().flatten().next())?;
    let ext = first.path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp") {
        return Some(first.path.clone());
    }
    if first.kind == MediaKind::Video {
        let out = dir.join("cover.jpg");
        let ok = crate::crafts::command("ffmpeg")
            .args(["-y", "-v", "error", "-ss", "0.5", "-i"])
            .arg(&first.path)
            .args(["-frames:v", "1", "-q:v", "3"])
            .arg(&out)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if ok && out.is_file() {
            return Some(out);
        }
    }
    None
}

fn film_preview(plan: &DraftPlan, dir: &Path) -> Result<film::PreviewOutput> {
    let mut clips = vec![];
    let us = |v: u64| v as f64 / 1e6;
    for c in &plan.main {
        clips.push(film::PreviewClip {
            path: &c.path, track: "V1".into(), start_sec: us(c.start_us), dur_sec: us(c.dur_us), src_in_sec: us(c.src_start_us), volume: c.volume,
            transition: c.transition.clone(), transition_sec: c.transition_sec, lut: c.lut.clone(), video_effect: c.video_effect.clone(),
            zoom: c.ken_burns.as_ref().map(|k| (k.zoom_start, k.zoom_end)), place: None,
        });
    }
    for (i, tr) in plan.overlay_tracks.iter().enumerate() {
        for c in tr {
            clips.push(film::PreviewClip {
                path: &c.path, track: format!("V{}", i + 2), start_sec: us(c.start_us), dur_sec: us(c.dur_us), src_in_sec: 0.0, volume: 0.0,
                transition: None, transition_sec: 0.0, lut: None, video_effect: None, zoom: None,
                place: Some(film::Place { x: c.x, y: c.y, scale: c.scale, opacity: c.opacity, width: c.width, height: c.height }),
            });
        }
    }
    for (i, tr) in plan.audio_tracks.iter().enumerate() {
        for c in &tr.clips {
            clips.push(film::PreviewClip {
                path: &c.path, track: format!("A{}", i + 1), start_sec: us(c.start_us), dur_sec: us(c.dur_us), src_in_sec: us(c.src_start_us), volume: c.volume,
                transition: None, transition_sec: 0.0, lut: None, video_effect: None, zoom: None, place: None,
            });
        }
    }
    // FilmCraft lays video clips down without their sound: a scene's own audio (someone talking
    // to camera, an interview) goes on one more audio track, as CapCut plays it from the clip.
    let scene_audio = format!("A{}", plan.audio_tracks.len() + 1);
    for c in plan.main.iter().filter(|c| c.kind == MediaKind::Video && c.volume > 0.001) {
        if media::probe(&c.path).has_audio {
            clips.push(film::PreviewClip {
                path: &c.path, track: scene_audio.clone(), start_sec: us(c.start_us), dur_sec: us(c.dur_us), src_in_sec: us(c.src_start_us), volume: c.volume,
                transition: None, transition_sec: 0.0, lut: None, video_effect: None, zoom: None, place: None,
            });
        }
    }
    film::preview(&clips, plan.width, plan.height, plan.fps, &dir.join("preview"), &slug(&plan.name))
}

fn summarize(plan: &DraftPlan) -> Vec<TrackSummary> {
    let v = |c: &PlanVideo| ClipSummary {
        name: c.desc.clone(),
        path: clean_path_str(&c.path.to_string_lossy()),
        start_sec: c.start_us as f64 / 1e6,
        dur_sec: c.dur_us as f64 / 1e6,
    };
    let mut out = vec![];
    for (i, tr) in plan.overlay_tracks.iter().enumerate().rev() {
        out.push(TrackSummary { label: format!("Overlay {}", i + 1), kind: "overlay", clips: tr.iter().map(v).collect() });
    }
    out.push(TrackSummary { label: "Hình chính".into(), kind: "video", clips: plan.main.iter().map(v).collect() });
    for tr in &plan.audio_tracks {
        out.push(TrackSummary {
            label: tr.label.clone(),
            kind: "audio",
            clips: tr
                .clips
                .iter()
                .map(|c| ClipSummary { name: c.desc.clone(), path: clean_path_str(&c.path.to_string_lossy()), start_sec: c.start_us as f64 / 1e6, dur_sec: c.dur_us as f64 / 1e6 })
                .collect(),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vo(start: u64, dur: u64) -> PlanAudio {
        PlanAudio { path: PathBuf::new(), file_dur_us: dur, start_us: start, dur_us: dur, src_start_us: 0, volume: 1.0, fade_in_us: 0, fade_out_us: 0, volume_keys: vec![], desc: String::new() }
    }

    #[test]
    fn ducking_dips_under_voice_and_recovers() {
        let k = duck_keys(0, 20_000_000, &[vo(2_000_000, 3_000_000), vo(5_200_000, 2_000_000)], 0.2, 0.08);
        // the two lines merge (gap 0.2 s), so one dip from 2 s to 7.2 s
        assert_eq!(k, vec![(0, 0.2), (1_700_000, 0.2), (2_000_000, 0.08), (7_200_000, 0.08), (7_500_000, 0.2), (20_000_000, 0.2)]);
        let times: Vec<u64> = k.iter().map(|x| x.0).collect();
        assert!(times.windows(2).all(|w| w[0] < w[1]));
    }

    const S: u64 = 1_000_000;
    fn video(len: u64) -> FitItem {
        FitItem { video_us: Some(len), fixed_us: None }
    }
    fn photo() -> FitItem {
        FitItem { video_us: None, fixed_us: None }
    }
    fn fixed(item: FitItem, d: u64) -> FitItem {
        FitItem { fixed_us: Some(d), ..item }
    }
    fn fit(target: u64, items: &[FitItem]) -> Vec<u64> {
        let (d, _) = fit_section("Đoạn 1", target, items).unwrap();
        assert_eq!(d.iter().sum::<u64>(), target, "section must end exactly on the next voice start");
        d
    }
    fn fit_err(target: u64, items: &[FitItem]) -> String {
        format!("{:#}", fit_section("Đoạn 1", target, items).unwrap_err())
    }

    #[test]
    fn section_photos_absorb_slack_videos_keep_length() {
        assert_eq!(fit(12 * S, &[video(3 * S), photo(), video(4 * S)]), vec![3 * S, 5 * S, 4 * S]);
        // two photos share it; the odd microsecond goes to the last one
        assert_eq!(fit(10 * S + 1, &[photo(), video(2 * S), photo()]), vec![4 * S, 2 * S, 4 * S + 1]);
    }

    #[test]
    fn section_videos_trimmed_proportionally() {
        assert_eq!(fit(5 * S, &[video(4 * S), video(6 * S)]), vec![2 * S, 3 * S]);
        assert_eq!(fit(10 * S, &[video(4 * S), video(6 * S)]), vec![4 * S, 6 * S]);
    }

    #[test]
    fn section_exact_on_real_lengths() {
        // a real case: 5 clips (15.63 s) under a 14.72 s voice
        let caps = [2_700_000, 2_866_667, 1_000_000, 7_433_333, 1_633_333];
        let items: Vec<FitItem> = caps.iter().map(|&c| video(c)).collect();
        let d = fit(14_717_914, &items);
        let k = 14_717_914.0 / caps.iter().sum::<u64>() as f64;
        for (d, c) in d.iter().zip(caps) {
            assert!(*d <= c);
            assert!((*d as f64 - c as f64 * k).abs() <= 1.0, "{d} vs {}", c as f64 * k);
        }
    }

    #[test]
    fn section_videos_never_stretched() {
        let e = fit_err(8 * S, &[video(3 * S), video(4 * S)]);
        assert!(e.contains("thiếu hình") && e.contains("thiếu 1.00s"), "{e}");
        // set durations alone must cover the voice too
        let e = fit_err(8 * S, &[fixed(photo(), 6 * S)]);
        assert!(e.contains("thiếu 2.00s"), "{e}");
    }

    #[test]
    fn section_fixed_durations_kept() {
        // a 6 s portrait, then clips trimmed into the rest
        assert_eq!(fit(9 * S, &[fixed(photo(), 6 * S), video(2 * S), video(4 * S)]), vec![6 * S, 1 * S, 2 * S]);
        // a set duration past the video's end is capped; the photo takes the difference
        let (d, w) = fit_section("Đoạn 1", 8 * S, &[fixed(video(3 * S), 5 * S), photo()]).unwrap();
        assert_eq!(d, vec![3 * S, 5 * S]);
        assert!(w[0].contains("rút ngắn"), "{w:?}");
        let e = fit_err(5 * S, &[fixed(photo(), 4 * S), fixed(photo(), 2 * S)]);
        assert!(e.contains("quá giọng đọc"), "{e}");
    }

    #[test]
    fn section_photo_floor_while_trimming() {
        assert_eq!(fit(5 * S, &[photo(), video(10 * S)]), vec![MIN_PHOTO_US, 3 * S]);
        // too short for the floor: everyone gets an even share
        assert_eq!(fit(1_800_000, &[photo(), video(5 * S)]), vec![900_000, 900_000]);
    }

    #[test]
    fn section_too_many_clips_is_an_error() {
        let items = vec![video(S); 30];
        let e = fit_err(2 * S, &items);
        assert!(e.contains("Bớt cảnh"), "{e}");
        let (_, w) = fit_section("Đoạn 1", 2 * S, &[video(S); 5]).unwrap();
        assert_eq!(w.len(), 5, "flash cuts are reported: {w:?}");
        assert!(fit_err(S, &[]).contains("chưa có cảnh"));
    }

    #[test]
    fn share_is_exact_and_capped() {
        for total in [0, 1, 7, 999_999, 3_333_333, 9_999_999, 10_000_000] {
            let caps = [3_333_333, 3_333_333, 3_333_334];
            let d = share(&caps, total);
            assert_eq!(d.iter().sum::<u64>(), total);
            assert!(d.iter().zip(caps).all(|(d, c)| *d <= c));
        }
    }

    fn clip(start: u64, dur: u64, desc: &str) -> PlanVideo {
        PlanVideo { path: PathBuf::new(), kind: MediaKind::Video, file_dur_us: dur, width: 1920, height: 1080, start_us: start, dur_us: dur, src_start_us: 0, volume: 0.0,
            ken_burns: None, x: 0.0, y: 0.0, scale: 1.0, opacity: 1.0, fade_in_us: 0, fade_out_us: 0, transition: None, transition_sec: 0.45, lut: None, video_effect: None, desc: desc.into() }
    }

    #[test]
    fn stack_reuses_free_tracks() {
        let t = stack(vec![clip(5 * S, 2 * S, "b"), clip(0, 4 * S, "a"), clip(2 * S, 4 * S, "c")]);
        let names: Vec<Vec<&str>> = t.iter().map(|tr| tr.iter().map(|c| c.desc.as_str()).collect()).collect();
        assert_eq!(names, vec![vec!["a", "b"], vec!["c"]]);
    }

    #[test]
    fn ducking_relative_to_segment_start() {
        let k = duck_keys(10_000_000, 5_000_000, &[vo(0, 12_000_000)], 0.2, 0.05);
        assert_eq!(k[0], (0, 0.05));
        assert_eq!(k.last().unwrap().0, 5_000_000);
    }
}
