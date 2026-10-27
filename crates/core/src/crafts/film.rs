//! FilmCraft: `filmcraft-cli probe` measures media with FilmCraft's pure-Rust demuxers and
//! decoders, and a preview of the whole cut is rebuilt as a FilmCraft sequence and exported to
//! H.264 MP4 (plus the `.fcproj`, so the edit also opens in FilmCraft).

use anyhow::{bail, Result};
use serde::Serialize;
use serde_json::Value;
use std::path::Path;
use std::time::Duration;

use super::{run, Craft};
use crate::media::MediaInfo;

#[derive(Debug, Clone, Serialize)]
pub struct FilmPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
}

/// Full catalog of 112 FilmCraft Video Transitions.
pub fn transitions() -> Vec<FilmPreset> {
    vec![
        FilmPreset { id: "cross_zoom", name: "Phóng Xuyên Cảnh (Cross Zoom)", category: "Tốc độ & Lia máy (Speed)", description: "Phóng vút qua điểm cắt kiểu CapCut Car Edit" },
        FilmPreset { id: "whip", name: "Lia Máy Vút (Whip Pan)", category: "Tốc độ & Lia máy (Speed)", description: "Lia ngang tốc độ cao kèm nhoè chuyển động" },
        FilmPreset { id: "zoom_blur", name: "Hút Tâm Tốc Độ (Zoom Blur)", category: "Tốc độ & Lia máy (Speed)", description: "Nhoè hướng tâm lao thẳng vào cảnh kế tiếp" },
        FilmPreset { id: "flash", name: "Chớp Trắng (Camera Flash)", category: "Ánh sáng & Tia chớp (Light)", description: "Chớp sáng mạnh đúng nhịp bass" },
        FilmPreset { id: "light_leak", name: "Lọt Sáng (Light Leak)", category: "Ánh sáng & Tia chớp (Light)", description: "Vệt nắng ấm tràn qua khung hình" },
        FilmPreset { id: "light_sweep", name: "Quét Tia Sáng (Light Sweep)", category: "Ánh sáng & Tia chớp (Light)", description: "Luồng sáng sắc nét quét ngang" },
        FilmPreset { id: "flare", name: "Loá Ống Kính (Lens Flare)", category: "Ánh sáng & Tia chớp (Light)", description: "Quầng sáng quang học lướt qua điểm cắt" },
        FilmPreset { id: "burn_chroma", name: "Cháy Sắc Độ (Burn Chroma)", category: "Ánh sáng & Tia chớp (Light)", description: "Cháy màu rực rỡ ở mép chuyển cảnh" },
        FilmPreset { id: "glitch", name: "Nhiễu Số (Digital Glitch)", category: "Glitch & Rung chấn", description: "Giật hình RGB kỹ thuật số" },
        FilmPreset { id: "earthquake", name: "Động Đất (Earthquake)", category: "Glitch & Rung chấn", description: "Va đập rung lắc mạnh khi sang cảnh mới" },
        FilmPreset { id: "vhs_damage", name: "Băng Từ VHS (VHS Damage)", category: "Glitch & Rung chấn", description: "Nhiễu sọc ngang hoài cổ" },
        FilmPreset { id: "liquid_distortion", name: "Sóng Chất Lỏng (Liquid Distortion)", category: "Glitch & Rung chấn", description: "Biến dạng uốn lượn mượt mà" },
        FilmPreset { id: "neon_wipe", name: "Quét Viền Neon (Neon Wipe)", category: "3D & Hình học", description: "Đường cắt phát sáng Neon hiện đại" },
        FilmPreset { id: "spin_3d", name: "Xoay Lật 3D (3D Spin)", category: "3D & Hình học", description: "Xoay lật không gian 3 chiều" },
        FilmPreset { id: "pop_motion", name: "Bật Nảy (Pop Motion)", category: "3D & Hình học", description: "Bật nảy đàn hồi năng động" },
        FilmPreset { id: "spring_motion", name: "Lò Xo (Spring Motion)", category: "3D & Hình học", description: "Co giãn nhịp nhàng" },
        FilmPreset { id: "luma_fade", name: "Tan Theo Vùng Sáng (Luma Fade)", category: "Điện ảnh (Cinematic)", description: "Chuyển mượt từ vùng sáng sang vùng tối" },
        FilmPreset { id: "film_dissolve", name: "Hoà Tan Phim Nhựa (Film Dissolve)", category: "Điện ảnh (Cinematic)", description: "Hoà trộn mượt chuẩn màu điện ảnh" },
        FilmPreset { id: "blur_dissolve", name: "Mờ Nhoè Chuyển Cảnh (Blur Dissolve)", category: "Điện ảnh (Cinematic)", description: "Mờ bokeh rồi lấy nét sang cảnh sau" },
        FilmPreset { id: "dip_to_black", name: "Chìm Vào Đen (Dip to Black)", category: "Điện ảnh (Cinematic)", description: "Tối dần rồi mở sang cảnh mới" },
        FilmPreset { id: "block_motion", name: "Block Motion (block_motion)", category: "Chuyển động đàn hồi (Animation)", description: "Chuyển cảnh Block Motion (Animation) · tham số: direction, blocks, motion_blur" },
        FilmPreset { id: "flip_motion", name: "Flip Motion (flip_motion)", category: "Chuyển động đàn hồi (Animation)", description: "Chuyển cảnh Flip Motion (Animation) · tham số: direction, perspective, motion_blur" },
        FilmPreset { id: "fold_motion", name: "Fold Motion (fold_motion)", category: "Chuyển động đàn hồi (Animation)", description: "Chuyển cảnh Fold Motion (Animation) · tham số: direction, perspective" },
        FilmPreset { id: "pull_motion", name: "Pull Motion (pull_motion)", category: "Chuyển động đàn hồi (Animation)", description: "Chuyển cảnh Pull Motion (Animation) · tham số: direction, motion_blur" },
        FilmPreset { id: "spin_motion", name: "Spin Motion (spin_motion)", category: "Chuyển động đàn hồi (Animation)", description: "Chuyển cảnh Spin Motion (Animation) · tham số: rotation, spin, motion_blur" },
        FilmPreset { id: "travel_motion", name: "Travel Motion (travel_motion)", category: "Chuyển động đàn hồi (Animation)", description: "Chuyển cảnh Travel Motion (Animation) · tham số: direction, zoom, motion_blur" },
        FilmPreset { id: "additive_dissolve", name: "Additive Dissolve (additive_dissolve)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Additive Dissolve (Dissolve)" },
        FilmPreset { id: "burn_alpha", name: "Burn Alpha (burn_alpha)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Burn Alpha (Dissolve) · tham số: burn_color, softness, seed" },
        FilmPreset { id: "cross_dissolve", name: "Cross Dissolve (cross_dissolve)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Cross Dissolve (Dissolve)" },
        FilmPreset { id: "dip_to_color", name: "Dip to Color (dip_to_color)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Dip to Color (Dissolve) · tham số: color, hold" },
        FilmPreset { id: "dip_to_white", name: "Dip to White (dip_to_white)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Dip to White (Dissolve)" },
        FilmPreset { id: "morph_cut", name: "Morph Cut (morph_cut)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Morph Cut (Dissolve)" },
        FilmPreset { id: "mosaic_transition", name: "Mosaic (mosaic_transition)", category: "Hoà tan (Dissolve)", description: "Chuyển cảnh Mosaic (Dissolve) · tham số: block_size, dissolve" },
        FilmPreset { id: "chaos", name: "Chaos (chaos)", category: "Glitch & Biến dạng (Grunge & Distort)", description: "Chuyển cảnh Chaos (Grunge & Distort) · tham số: amount, seed" },
        FilmPreset { id: "flicker", name: "Flicker (flicker)", category: "Glitch & Biến dạng (Grunge & Distort)", description: "Chuyển cảnh Flicker (Grunge & Distort) · tham số: flickers, seed" },
        FilmPreset { id: "glass", name: "Glass (glass)", category: "Glitch & Biến dạng (Grunge & Distort)", description: "Chuyển cảnh Glass (Grunge & Distort) · tham số: cell_size, refraction, seed" },
        FilmPreset { id: "grunge", name: "Grunge (grunge)", category: "Glitch & Biến dạng (Grunge & Distort)", description: "Chuyển cảnh Grunge (Grunge & Distort) · tham số: scale, edge_color, edge_width, seed" },
        FilmPreset { id: "kaleidoscope", name: "Kaleidoscope (kaleidoscope)", category: "Glitch & Biến dạng (Grunge & Distort)", description: "Chuyển cảnh Kaleidoscope (Grunge & Distort) · tham số: segments, zoom" },
        FilmPreset { id: "tv_power", name: "TV Power (tv_power)", category: "Glitch & Biến dạng (Grunge & Distort)", description: "Chuyển cảnh TV Power (Grunge & Distort) · tham số: glow_color, line" },
        FilmPreset { id: "vr_chroma_leaks", name: "VR Chroma Leaks (vr_chroma_leaks)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Chroma Leaks (Immersive Video) · tham số: amount, seed" },
        FilmPreset { id: "vr_gradient_wipe", name: "VR Gradient Wipe (vr_gradient_wipe)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Gradient Wipe (Immersive Video) · tham số: softness, invert" },
        FilmPreset { id: "vr_iris_wipe", name: "VR Iris Wipe (vr_iris_wipe)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Iris Wipe (Immersive Video) · tham số: center, feather, border_width, border_color" },
        FilmPreset { id: "vr_light_leaks", name: "VR Light Leaks (vr_light_leaks)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Light Leaks (Immersive Video) · tham số: amount, seed" },
        FilmPreset { id: "vr_light_rays", name: "VR Light Rays (vr_light_rays)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Light Rays (Immersive Video) · tham số: center, amount" },
        FilmPreset { id: "vr_mobius_zoom", name: "VR Mobius Zoom (vr_mobius_zoom)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Mobius Zoom (Immersive Video) · tham số: zoom, twist" },
        FilmPreset { id: "vr_random_blocks", name: "VR Random Blocks (vr_random_blocks)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Random Blocks (Immersive Video) · tham số: block_size, softness, seed" },
        FilmPreset { id: "vr_spherical_blur", name: "VR Spherical Blur (vr_spherical_blur)", category: "Không gian VR 360 (Immersive)", description: "Chuyển cảnh VR Spherical Blur (Immersive Video) · tham số: blur" },
        FilmPreset { id: "chroma_leak", name: "Chroma Leak (chroma_leak)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Chroma Leak (Lights & Blurs) · tham số: amount, seed" },
        FilmPreset { id: "directional_blur_transition", name: "Directional Blur (directional_blur_transition)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Directional Blur (Lights & Blurs) · tham số: angle, blur" },
        FilmPreset { id: "glow", name: "Glow (glow)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Glow (Lights & Blurs) · tham số: radius, intensity" },
        FilmPreset { id: "lens_blur", name: "Lens Blur (lens_blur)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Lens Blur (Lights & Blurs) · tham số: radius" },
        FilmPreset { id: "phosphor", name: "Phosphor (phosphor)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Phosphor (Lights & Blurs) · tham số: color, decay" },
        FilmPreset { id: "radial_blur", name: "Radial Blur (radial_blur)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Radial Blur (Lights & Blurs) · tham số: center, angle" },
        FilmPreset { id: "ray", name: "Ray (ray)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Ray (Lights & Blurs) · tham số: center, amount, length" },
        FilmPreset { id: "solarize", name: "Solarize (solarize)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Solarize (Lights & Blurs) · tham số: amount" },
        FilmPreset { id: "stripe", name: "Stripe (stripe)", category: "Ánh sáng & Nhoè (Lights & Blurs)", description: "Chuyển cảnh Stripe (Lights & Blurs) · tham số: stripes, angle, color" },
        FilmPreset { id: "roll_3d", name: "3D Roll (roll_3d)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh 3D Roll (Slide) · tham số: direction, perspective" },
        FilmPreset { id: "film_roll", name: "Film Roll (film_roll)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh Film Roll (Slide) · tham số: direction, gap, gap_color, motion_blur" },
        FilmPreset { id: "push", name: "Push (push)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh Push (Slide) · tham số: direction, motion_blur" },
        FilmPreset { id: "roll", name: "Roll (roll)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh Roll (Slide) · tham số: direction, motion_blur" },
        FilmPreset { id: "slide", name: "Slide (slide)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh Slide (Slide) · tham số: direction, motion_blur" },
        FilmPreset { id: "split", name: "Split (split)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh Split (Slide) · tham số: orientation, motion_blur" },
        FilmPreset { id: "stretch", name: "Stretch (stretch)", category: "Trượt & Cuộn (Slide)", description: "Chuyển cảnh Stretch (Slide) · tham số: direction" },
        FilmPreset { id: "motion_camera", name: "Motion Camera (motion_camera)", category: "Thông minh (Smart Tools)", description: "Chuyển cảnh Motion Camera (Smart Tools) · tham số: zoom, rotation, motion_blur" },
        FilmPreset { id: "motion_tween", name: "Motion Tween (motion_tween)", category: "Thông minh (Smart Tools)", description: "Chuyển cảnh Motion Tween (Smart Tools) · tham số: scale" },
        FilmPreset { id: "shape_dissolve", name: "Shape Dissolve (shape_dissolve)", category: "Thông minh (Smart Tools)", description: "Chuyển cảnh Shape Dissolve (Smart Tools) · tham số: shape, size, softness, rotation" },
        FilmPreset { id: "shape_flow", name: "Shape Flow (shape_flow)", category: "Thông minh (Smart Tools)", description: "Chuyển cảnh Shape Flow (Smart Tools) · tham số: shape, direction, softness" },
        FilmPreset { id: "text_animator", name: "Text Animator (text_animator)", category: "Chữ động (Text)", description: "Chuyển cảnh Text Animator (Text) · tham số: columns, rows" },
        FilmPreset { id: "typewriter", name: "Typewriter (typewriter)", category: "Chữ động (Text)", description: "Chuyển cảnh Typewriter (Text) · tham số: columns, rows, cursor" },
        FilmPreset { id: "spinback_3d", name: "3D Spinback (spinback_3d)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh 3D Spinback (Transformers) · tham số: direction, depth" },
        FilmPreset { id: "frame", name: "Frame (frame)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh Frame (Transformers) · tham số: frame_width, frame_color, scale" },
        FilmPreset { id: "louver", name: "Louver (louver)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh Louver (Transformers) · tham số: slats, orientation" },
        FilmPreset { id: "mirror_transition", name: "Mirror (mirror_transition)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh Mirror (Transformers) · tham số: direction" },
        FilmPreset { id: "page_peel", name: "Page Peel (page_peel)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh Page Peel (Transformers) · tham số: corner, radius, shadow" },
        FilmPreset { id: "slice", name: "Slice (slice)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh Slice (Transformers) · tham số: slices, angle, motion_blur" },
        FilmPreset { id: "wave", name: "Wave (wave)", category: "Biến hình 3D (Transformers)", description: "Chuyển cảnh Wave (Transformers) · tham số: direction, amplitude, wavelength" },
        FilmPreset { id: "clock_wipe", name: "Clock Wipe (clock_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Clock Wipe (Wipe) · tham số: center, start, feather, border_width" },
        FilmPreset { id: "linear_wipe", name: "Linear Wipe (linear_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Linear Wipe (Wipe) · tham số: angle, feather, border_width, border_color" },
        FilmPreset { id: "panel_wipe", name: "Panel Wipe (panel_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Panel Wipe (Wipe) · tham số: direction, panels, feather, border_width" },
        FilmPreset { id: "plateau_wipe", name: "Plateau Wipe (plateau_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Plateau Wipe (Wipe) · tham số: angle, plateaus, feather, border_width" },
        FilmPreset { id: "radial_wipe", name: "Radial Wipe (radial_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Radial Wipe (Wipe) · tham số: corner, feather, border_width, border_color" },
        FilmPreset { id: "soft_wipe", name: "Soft Wipe (soft_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Soft Wipe (Wipe) · tham số: angle, feather" },
        FilmPreset { id: "star_wipe", name: "Star Wipe (star_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Star Wipe (Wipe) · tham số: center, points, inner, rotation" },
        FilmPreset { id: "stretch_wipe", name: "Stretch Wipe (stretch_wipe)", category: "Quét màn hình (Wipe)", description: "Chuyển cảnh Stretch Wipe (Wipe) · tham số: direction, stretch" },
        FilmPreset { id: "additive_dissolve_legacy", name: "Additive Dissolve (Legacy) (additive_dissolve_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Additive Dissolve (Legacy) (Video Transitions)" },
        FilmPreset { id: "barn_doors", name: "Barn Doors (barn_doors)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Barn Doors (Video Transitions) · tham số: orientation, border_width, border_color, antialias" },
        FilmPreset { id: "center_split", name: "Center Split (center_split)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Center Split (Video Transitions)" },
        FilmPreset { id: "clock_wipe_legacy", name: "Clock Wipe (Legacy) (clock_wipe_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Clock Wipe (Legacy) (Video Transitions)" },
        FilmPreset { id: "cross_dissolve_legacy", name: "Cross Dissolve (Legacy) (cross_dissolve_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Cross Dissolve (Legacy) (Video Transitions)" },
        FilmPreset { id: "cross_zoom_legacy", name: "Cross Zoom (Legacy) (cross_zoom_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Cross Zoom (Legacy) (Video Transitions)" },
        FilmPreset { id: "dip_to_black_legacy", name: "Dip to Black (Legacy) (dip_to_black_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Dip to Black (Legacy) (Video Transitions)" },
        FilmPreset { id: "dip_to_white_legacy", name: "Dip to White (Legacy) (dip_to_white_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Dip to White (Legacy) (Video Transitions)" },
        FilmPreset { id: "film_dissolve_legacy", name: "Film Dissolve (Legacy) (film_dissolve_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Film Dissolve (Legacy) (Video Transitions)" },
        FilmPreset { id: "inset", name: "Inset (inset)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Inset (Video Transitions) · tham số: corner, border_width, border_color, antialias" },
        FilmPreset { id: "iris_box", name: "Iris Box (iris_box)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Iris Box (Video Transitions) · tham số: center, border_width, border_color, antialias" },
        FilmPreset { id: "iris_cross", name: "Iris Cross (iris_cross)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Iris Cross (Video Transitions) · tham số: center, border_width, border_color, antialias" },
        FilmPreset { id: "iris_diamond", name: "Iris Diamond (iris_diamond)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Iris Diamond (Video Transitions) · tham số: center, border_width, border_color, antialias" },
        FilmPreset { id: "iris_round", name: "Iris Round (iris_round)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Iris Round (Video Transitions) · tham số: center, border_width, border_color, antialias" },
        FilmPreset { id: "non_additive_dissolve", name: "Non-Additive Dissolve (non_additive_dissolve)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Non-Additive Dissolve (Video Transitions)" },
        FilmPreset { id: "push_legacy", name: "Push (Legacy) (push_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Push (Legacy) (Video Transitions) · tham số: direction" },
        FilmPreset { id: "radial_wipe_legacy", name: "Radial Wipe (Legacy) (radial_wipe_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Radial Wipe (Legacy) (Video Transitions)" },
        FilmPreset { id: "slide_legacy", name: "Slide (Legacy) (slide_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Slide (Legacy) (Video Transitions) · tham số: direction" },
        FilmPreset { id: "split_legacy", name: "Split (Legacy) (split_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Split (Legacy) (Video Transitions)" },
        FilmPreset { id: "whip_legacy", name: "Whip (Legacy) (whip_legacy)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Whip (Legacy) (Video Transitions) · tham số: direction" },
        FilmPreset { id: "wipe", name: "Wipe (wipe)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Wipe (Video Transitions) · tham số: direction" },
        FilmPreset { id: "band_slide", name: "Band Slide (band_slide)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Band Slide (Video Transitions)" },
        FilmPreset { id: "checker_wipe", name: "Checker Wipe (checker_wipe)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Checker Wipe (Video Transitions)" },
        FilmPreset { id: "cube_spin", name: "Cube Spin (cube_spin)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Cube Spin (Video Transitions)" },
        FilmPreset { id: "flip_over", name: "Flip Over (flip_over)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Flip Over (Video Transitions)" },
        FilmPreset { id: "gradient_wipe", name: "Gradient Wipe (gradient_wipe)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Gradient Wipe (Video Transitions) · tham số: softness" },
        FilmPreset { id: "page_turn", name: "Page Turn (page_turn)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Page Turn (Video Transitions)" },
        FilmPreset { id: "venetian_blinds", name: "Venetian Blinds (venetian_blinds)", category: "Chuyển cảnh cổ điển (Classic)", description: "Chuyển cảnh Venetian Blinds (Video Transitions)" },
    ]
}

/// Full catalog of 26 FilmCraft Look LUTs & Lumetri Color Presets.
pub fn luts() -> Vec<FilmPreset> {
    vec![
        FilmPreset { id: "builtin:look-teal-orange", name: "Teal & Orange (Bom tấn)", category: "Creative Look LUT", description: "Tương phản xanh lạnh & cam ấm rực rỡ" },
        FilmPreset { id: "builtin:look-golden-hour", name: "Golden Hour (Hoàng hôn vàng)", category: "Creative Look LUT", description: "Ánh vàng rực ấm áp, sang trọng" },
        FilmPreset { id: "builtin:look-night", name: "Cyber Night (Đêm điện ảnh)", category: "Creative Look LUT", description: "Tông xanh đêm sâu và sắc lạnh" },
        FilmPreset { id: "builtin:look-cool-blue", name: "Cool Blue (Xanh kim khí)", category: "Creative Look LUT", description: "Tông lạnh sắc sảo cho siêu xe & công nghệ" },
        FilmPreset { id: "builtin:look-warm-film", name: "Warm Film (Phim nhựa ấm)", category: "Creative Look LUT", description: "Màu phim Kodak cổ điển" },
        FilmPreset { id: "builtin:look-bleach-bypass", name: "Bleach Bypass (Gai góc)", category: "Creative Look LUT", description: "Tương phản gắt kiểu phim hành động tốc độ" },
        FilmPreset { id: "builtin:look-faded-matte", name: "Faded Matte (Mờ sương)", category: "Creative Look LUT", description: "Nâng nhẹ vùng tối kiểu điện ảnh nghệ thuật" },
        FilmPreset { id: "builtin:look-monochrome", name: "Monochrome (Đen trắng)", category: "Creative Look LUT", description: "Đen trắng tương phản cao" },
        FilmPreset { id: "lumetri:Teal Shadows, Warm Skin", name: "Lumetri · Teal Shadows, Warm Skin", category: "Lumetri Cinematic", description: "Cyan-teal vùng tối và giữ ấm tông da với tương phản điện ảnh" },
        FilmPreset { id: "lumetri:Night Exterior", name: "Lumetri · Night Exterior", category: "Lumetri Cinematic", description: "Hiệu ứng Day-for-Night: tối sâu, sắc lạnh và giảm bão hoà" },
        FilmPreset { id: "lumetri:Golden Dusk", name: "Lumetri · Golden Dusk", category: "Lumetri Cinematic", description: "Ánh nắng chiều tà ấm dịu và mềm mại" },
        FilmPreset { id: "lumetri:Steel Blue", name: "Lumetri · Steel Blue", category: "Lumetri Cinematic", description: "Tông thép xanh lạnh, giảm bão hoà và sắc nét" },
        FilmPreset { id: "lumetri:Hard Sun", name: "Lumetri · Hard Sun", category: "Lumetri Cinematic", description: "Tương phản mạnh, giữ chi tiết vùng cháy sáng và bóng đổ sâu" },
        FilmPreset { id: "lumetri:Faded Print", name: "Lumetri · Faded Print", category: "Lumetri Film Emulation", description: "Màu phim in cổ điển: nâng vùng đen, tương phản êm, màu trầm" },
        FilmPreset { id: "lumetri:Silver Retention", name: "Lumetri · Silver Retention", category: "Lumetri Film Emulation", description: "Giữ bạc kiểu Bleach-Bypass: giảm bão hoà kèm tương phản cứng" },
        FilmPreset { id: "lumetri:Cross Processed", name: "Lumetri · Cross Processed", category: "Lumetri Film Emulation", description: "Tráng chéo kênh màu: vùng sáng vàng-lục, vùng tối xanh lam" },
        FilmPreset { id: "lumetri:Warm Negative", name: "Lumetri · Warm Negative", category: "Lumetri Film Emulation", description: "Phim âm bản ấm áp với đường cong chuyển sáng mượt" },
        FilmPreset { id: "lumetri:Neutral Mono", name: "Lumetri · Neutral Mono", category: "Lumetri Monochrome", description: "Đen trắng trung tính chuẩn điện ảnh" },
        FilmPreset { id: "lumetri:Hard Mono", name: "Lumetri · Hard Mono", category: "Lumetri Monochrome", description: "Đen trắng tương phản gắt, vùng tối sâu" },
        FilmPreset { id: "lumetri:Sepia Tone", name: "Lumetri · Sepia Tone", category: "Lumetri Monochrome", description: "Đơn sắc nhuộm tông nâu ấm hoài cổ" },
        FilmPreset { id: "lumetri:Cold Mono", name: "Lumetri · Cold Mono", category: "Lumetri Monochrome", description: "Đơn sắc nhuộm tông xanh băng giá" },
        FilmPreset { id: "lumetri:Lift Shadows", name: "Lumetri · Lift Shadows", category: "Lumetri Technical", description: "Mở sáng chi tiết vùng tối (Shadows)" },
        FilmPreset { id: "lumetri:Protect Highlights", name: "Lumetri · Protect Highlights", category: "Lumetri Technical", description: "Hạ dịu và cứu chi tiết vùng cháy sáng (Highlights)" },
        FilmPreset { id: "lumetri:Half Saturation", name: "Lumetri · Half Saturation", category: "Lumetri Technical", description: "Giảm 50% độ bão hoà màu toàn khung" },
        FilmPreset { id: "lumetri:Boost Vibrance", name: "Lumetri · Boost Vibrance", category: "Lumetri Technical", description: "Tăng độ tươi màu thông minh cho vùng nhạt mà không làm cháy da" },
        FilmPreset { id: "lumetri:Legal Range Squeeze", name: "Lumetri · Legal Range Squeeze", category: "Lumetri Technical", description: "Nén dải sáng chuẩn truyền hình 16–235 bằng đường cong Luma" },
    ]
}

/// Full catalog of 139 FilmCraft Video Effects & Built-in Effect Presets.
pub fn video_effects() -> Vec<FilmPreset> {
    vec![
        FilmPreset { id: "wonder_glow", name: "Hào Quang Rực Rỡ (Wonder Glow)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Phát sáng mịn vùng đèn pha & kim loại bóng" },
        FilmPreset { id: "glint", name: "Tia Sao Lấp Lánh (Star Glint)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Tia sao lấp lánh trên thân xe và mâm hợp kim" },
        FilmPreset { id: "light_leaks", name: "Vệt Sáng Trôi (Light Leaks)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Luồng sáng màu điện ảnh trôi trên khung hình" },
        FilmPreset { id: "volumetric_rays", name: "Tia Nắng Khối (God Rays)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Luồng tia sáng xuyên không gian" },
        FilmPreset { id: "edge_glow", name: "Viền Phát Sáng (Edge Glow)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Tôn đường gân khí động học phát sáng" },
        FilmPreset { id: "echo_glow", name: "Dư Ảnh Phát Sáng (Echo Glow)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Hào quang đa tầng ngân dài" },
        FilmPreset { id: "lens_flare", name: "Loá Đèn Pha (Lens Flare)", category: "Ánh sáng & Glow (Lights & Glows)", description: "Điểm lóa quang học rực rỡ" },
        FilmPreset { id: "rgb_split", name: "Tách Màu RGB (Chromatic Split)", category: "Hành động & Glitch", description: "Lệch kênh màu Đỏ-Xanh kiểu CapCut Pro" },
        FilmPreset { id: "camera_shake", name: "Rung Máy Tốc Độ (Camera Shake)", category: "Hành động & Glitch", description: "Rung lắc hành động kèm nhoè chuyển động" },
        FilmPreset { id: "vr_digital_glitch", name: "Nhiễu Sóng Cyber (Digital Glitch)", category: "Hành động & Glitch", description: "Sọc nhiễu kỹ thuật số hiện đại" },
        FilmPreset { id: "vignette", name: "Tối Bốn Góc (Vignette)", category: "Ống kính & Màu", description: "Tập trung ánh nhìn vào chủ thể trung tâm" },
        FilmPreset { id: "bokeh_blur", name: "Xoá Phông Bokeh (Bokeh Blur)", category: "Ống kính & Màu", description: "Làm mờ lung linh kiểu ống kính khẩu lớn" },
        FilmPreset { id: "motion", name: "Motion (motion)", category: "Cơ bản (Core)", description: "Kỹ xảo Motion · tham số: position, scale, scale_width, uniform_scale" },
        FilmPreset { id: "opacity", name: "Opacity (opacity)", category: "Cơ bản (Core)", description: "Kỹ xảo Opacity · tham số: opacity, blend" },
        FilmPreset { id: "time_remap", name: "Time Remapping (time_remap)", category: "Cơ bản (Core)", description: "Kỹ xảo Time Remapping · tham số: speed" },
        FilmPreset { id: "brightness_contrast", name: "Brightness & Contrast (brightness_contrast)", category: "Chỉnh màu (Color Correction)", description: "Kỹ xảo Brightness & Contrast · tham số: brightness, contrast" },
        FilmPreset { id: "tint", name: "Tint (tint)", category: "Chỉnh màu (Color Correction)", description: "Kỹ xảo Tint · tham số: black, white, amount" },
        FilmPreset { id: "color_balance", name: "Color Balance (color_balance)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Color Balance · tham số: shadow_r, shadow_g, shadow_b, mid_r" },
        FilmPreset { id: "leave_color", name: "Leave Color (leave_color)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Leave Color · tham số: amount, color, tolerance, softness" },
        FilmPreset { id: "change_to_color", name: "Change to Color (change_to_color)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Change to Color · tham số: from, to, hue_tol, softness" },
        FilmPreset { id: "lumetri", name: "Lumetri Color (lumetri)", category: "Chỉnh màu (Color Correction)", description: "Kỹ xảo Lumetri Color · tham số: basic_on, input_lut, temperature, tint" },
        FilmPreset { id: "gaussian_blur", name: "Gaussian Blur (gaussian_blur)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Gaussian Blur · tham số: blurriness, dimensions, repeat_edge" },
        FilmPreset { id: "directional_blur", name: "Directional Blur (directional_blur)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Directional Blur · tham số: direction, length" },
        FilmPreset { id: "sharpen", name: "Sharpen (sharpen)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Sharpen · tham số: amount" },
        FilmPreset { id: "unsharp_mask", name: "Unsharp Mask (unsharp_mask)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Unsharp Mask · tham số: amount, radius, threshold" },
        FilmPreset { id: "camera_blur", name: "Camera Blur (camera_blur)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Camera Blur · tham số: percent" },
        FilmPreset { id: "black_white", name: "Black & White (black_white)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Black & White" },
        FilmPreset { id: "color_pass", name: "Color Pass (color_pass)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Color Pass · tham số: color, similarity, reverse" },
        FilmPreset { id: "gamma_correction", name: "Gamma Correction (gamma_correction)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Gamma Correction · tham số: gamma" },
        FilmPreset { id: "invert", name: "Invert (invert)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Invert · tham số: channel, blend" },
        FilmPreset { id: "levels", name: "Levels (levels)", category: "Điều chỉnh ánh sáng (Adjust)", description: "Kỹ xảo Levels · tham số: in_black, in_white, out_black, out_white" },
        FilmPreset { id: "proc_amp", name: "ProcAmp (proc_amp)", category: "Điều chỉnh ánh sáng (Adjust)", description: "Kỹ xảo ProcAmp · tham số: brightness, contrast, hue, saturation" },
        FilmPreset { id: "extract", name: "Extract (extract)", category: "Điều chỉnh ánh sáng (Adjust)", description: "Kỹ xảo Extract · tham số: black, white, softness, invert" },
        FilmPreset { id: "posterize", name: "Posterize (posterize)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Posterize · tham số: levels" },
        FilmPreset { id: "mosaic", name: "Mosaic (mosaic)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Mosaic · tham số: horizontal, vertical, sharp, softness" },
        FilmPreset { id: "find_edges", name: "Find Edges (find_edges)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Find Edges · tham số: invert, blend" },
        FilmPreset { id: "emboss", name: "Emboss (emboss)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Emboss · tham số: direction, relief, contrast, blend" },
        FilmPreset { id: "replicate", name: "Replicate (replicate)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Replicate · tham số: count" },
        FilmPreset { id: "strobe", name: "Strobe Light (strobe)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Strobe Light · tham số: color, blend, duration, period" },
        FilmPreset { id: "noise", name: "Noise (noise)", category: "Noise & Grain", description: "Kỹ xảo Noise · tham số: amount, color, clip, grain_size" },
        FilmPreset { id: "median", name: "Median (median)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Median · tham số: radius" },
        FilmPreset { id: "crop", name: "Crop (crop)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Crop · tham số: left, top, right, bottom" },
        FilmPreset { id: "horizontal_flip", name: "Horizontal Flip (horizontal_flip)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Horizontal Flip" },
        FilmPreset { id: "vertical_flip", name: "Vertical Flip (vertical_flip)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Vertical Flip" },
        FilmPreset { id: "edge_feather", name: "Edge Feather (edge_feather)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Edge Feather · tham số: amount" },
        FilmPreset { id: "transform", name: "Transform (transform)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Transform · tham số: anchor, position, uniform_scale, scale_height" },
        FilmPreset { id: "mirror", name: "Mirror (mirror)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Mirror · tham số: center, angle" },
        FilmPreset { id: "offset", name: "Offset (offset)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Offset · tham số: shift, blend" },
        FilmPreset { id: "lens_distortion", name: "Lens Distortion (lens_distortion)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Lens Distortion · tham số: curvature, v_decentering, h_decentering" },
        FilmPreset { id: "twirl", name: "Twirl (twirl)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Twirl · tham số: angle, radius, center" },
        FilmPreset { id: "wave_warp", name: "Wave Warp (wave_warp)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Wave Warp · tham số: height, width, direction, speed" },
        FilmPreset { id: "basic_3d", name: "Basic 3D (basic_3d)", category: "Phối cảnh 3D & Bóng đổ (Perspective)", description: "Kỹ xảo Basic 3D · tham số: swivel, tilt, distance" },
        FilmPreset { id: "drop_shadow", name: "Drop Shadow (drop_shadow)", category: "Phối cảnh 3D & Bóng đổ (Perspective)", description: "Kỹ xảo Drop Shadow · tham số: color, opacity, direction, distance" },
        FilmPreset { id: "bevel_alpha", name: "Bevel Alpha (bevel_alpha)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Bevel Alpha · tham số: thickness, angle, color, intensity" },
        FilmPreset { id: "ultra_key", name: "Ultra Key (ultra_key)", category: "Tách nền & Alpha (Keying)", description: "Kỹ xảo Ultra Key · tham số: key_color, output, setting, transparency" },
        FilmPreset { id: "track_matte", name: "Track Matte Key (track_matte)", category: "Tách nền & Alpha (Keying)", description: "Kỹ xảo Track Matte Key · tham số: matte, composite, reverse" },
        FilmPreset { id: "color_key", name: "Color Key (color_key)", category: "Tách nền & Alpha (Keying)", description: "Kỹ xảo Color Key · tham số: color, tolerance, thin, feather" },
        FilmPreset { id: "luma_key", name: "Luma Key (luma_key)", category: "Tách nền & Alpha (Keying)", description: "Kỹ xảo Luma Key · tham số: threshold, cutoff" },
        FilmPreset { id: "four_color_gradient", name: "4-Color Gradient (four_color_gradient)", category: "Tạo dải màu (Generate)", description: "Kỹ xảo 4-Color Gradient · tham số: c1, c2, c3, c4" },
        FilmPreset { id: "ramp", name: "Ramp (ramp)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Ramp · tham số: start, start_color, end, end_color" },
        FilmPreset { id: "circle", name: "Circle (circle)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Circle · tham số: center, radius, color, opacity" },
        FilmPreset { id: "grid", name: "Grid (grid)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Grid · tham số: size, border, color, opacity" },
        FilmPreset { id: "timecode", name: "Timecode (timecode)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Timecode · tham số: position, size, opacity" },
        FilmPreset { id: "clip_name", name: "Clip Name (clip_name)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Clip Name · tham số: position, size" },
        FilmPreset { id: "simple_text", name: "Simple Text (simple_text)", category: "Tiện ích (Utility)", description: "Kỹ xảo Simple Text · tham số: text, position, size, font" },
        FilmPreset { id: "lighting_effects", name: "Lighting Effects (lighting_effects)", category: "Điều chỉnh ánh sáng (Adjust)", description: "Kỹ xảo Lighting Effects · tham số: l1_type, l1_color, l1_center, l1_major" },
        FilmPreset { id: "channel_blur", name: "Channel Blur (channel_blur)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Channel Blur · tham số: red, green, blue, alpha" },
        FilmPreset { id: "compound_blur", name: "Compound Blur (compound_blur)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Compound Blur · tham số: layer, max, stretch, invert" },
        FilmPreset { id: "focus_blur", name: "Focus Blur (focus_blur)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Focus Blur · tham số: shape, center, angle, size" },
        FilmPreset { id: "reduce_interlace_flicker", name: "Reduce Interlace Flicker (reduce_interlace_flicker)", category: "Làm mờ & Sắc nét (Blur & Sharpen)", description: "Kỹ xảo Reduce Interlace Flicker · tham số: softness" },
        FilmPreset { id: "asc_cdl", name: "ASC CDL (asc_cdl)", category: "Chỉnh màu (Color Correction)", description: "Kỹ xảo ASC CDL · tham số: r_slope, r_offset, r_power, g_slope" },
        FilmPreset { id: "video_limiter", name: "Video Limiter (video_limiter)", category: "Chỉnh màu (Color Correction)", description: "Kỹ xảo Video Limiter · tham số: clip_level, compression, axis, gamut_warning" },
        FilmPreset { id: "corner_pin", name: "Corner Pin (corner_pin)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Corner Pin · tham số: upper_left, upper_right, lower_left, lower_right" },
        FilmPreset { id: "magnify", name: "Magnify (magnify)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Magnify · tham số: shape, center, magnification, size" },
        FilmPreset { id: "spherize", name: "Spherize (spherize)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Spherize · tham số: radius, center" },
        FilmPreset { id: "turbulent_displace", name: "Turbulent Displace (turbulent_displace)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Turbulent Displace · tham số: displacement, amount, size, offset" },
        FilmPreset { id: "warp_stabilizer", name: "Warp Stabilizer (warp_stabilizer)", category: "Biến dạng quang học (Distort)", description: "Kỹ xảo Warp Stabilizer · tham số: result, smoothness, method, preserve_scale" },
        FilmPreset { id: "gradient", name: "Gradient (gradient)", category: "Tạo dải màu (Generate)", description: "Kỹ xảo Gradient · tham số: start, start_color, end, end_color" },
        FilmPreset { id: "channel_mix", name: "Channel Mix (channel_mix)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Channel Mix · tham số: rr, rg, rb, rc" },
        FilmPreset { id: "color_replace", name: "Color Replace (color_replace)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Color Replace · tham số: similarity, solid, target, replace" },
        FilmPreset { id: "rounded_crop", name: "Rounded Crop (rounded_crop)", category: "Kiểm soát ảnh (Image Control)", description: "Kỹ xảo Rounded Crop · tham số: left, top, right, bottom" },
        FilmPreset { id: "alpha_adjust", name: "Alpha Adjust (alpha_adjust)", category: "Tách nền & Alpha (Keying)", description: "Kỹ xảo Alpha Adjust · tham số: opacity, ignore, invert, mask_only" },
        FilmPreset { id: "logo_cutout", name: "Logo Cutout (logo_cutout)", category: "Tách nền & Alpha (Keying)", description: "Kỹ xảo Logo Cutout · tham số: background, color, threshold, softness" },
        FilmPreset { id: "long_shadow", name: "Long Shadow (long_shadow)", category: "Phối cảnh 3D & Bóng đổ (Perspective)", description: "Kỹ xảo Long Shadow · tham số: angle, length, color, opacity" },
        FilmPreset { id: "brush_strokes", name: "Brush Strokes (brush_strokes)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Brush Strokes · tham số: angle, size, length, density" },
        FilmPreset { id: "color_emboss", name: "Color Emboss (color_emboss)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Color Emboss · tham số: direction, relief, contrast, blend" },
        FilmPreset { id: "roughen_edges", name: "Roughen Edges (roughen_edges)", category: "Phong cách hoá (Stylize)", description: "Kỹ xảo Roughen Edges · tham số: edge_type, edge_color, border, sharpness" },
        FilmPreset { id: "posterize_time", name: "Posterize Time (posterize_time)", category: "Thời gian (Time)", description: "Kỹ xảo Posterize Time · tham số: rate" },
        FilmPreset { id: "rotate_3d", name: "3D Rotate (rotate_3d)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo 3D Rotate · tham số: rot_x, rot_y, rot_z, perspective" },
        FilmPreset { id: "auto_reframe", name: "Auto Reframe (auto_reframe)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Auto Reframe · tham số: preset, offset, zoom, aspect" },
        FilmPreset { id: "grow", name: "Grow (grow)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Grow · tham số: from, to, center, easing" },
        FilmPreset { id: "shrink", name: "Shrink (shrink)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Shrink · tham số: from, to, center, easing" },
        FilmPreset { id: "move", name: "Move (move)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Move · tham số: from, to, easing, motion_blur" },
        FilmPreset { id: "spin", name: "Spin (spin)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Spin · tham số: amount, center, easing, scale" },
        FilmPreset { id: "spacer", name: "Spacer (spacer)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Spacer · tham số: left, top, right, bottom" },
        FilmPreset { id: "wiggle", name: "Wiggle (wiggle)", category: "Biến đổi & Chuyển động (Transform)", description: "Kỹ xảo Wiggle · tham số: frequency, amount, rotation, scale" },
        FilmPreset { id: "auto_align", name: "Auto Align (auto_align)", category: "Tiện ích (Utility)", description: "Kỹ xảo Auto Align · tham số: horizontal, vertical, margin_x, margin_y" },
        FilmPreset { id: "cineon_converter", name: "Cineon Converter (cineon_converter)", category: "Tiện ích (Utility)", description: "Kỹ xảo Cineon Converter · tham số: conversion, black10, black_internal, white10" },
        FilmPreset { id: "clone", name: "Clone (clone)", category: "Tiện ích (Utility)", description: "Kỹ xảo Clone · tham số: columns, rows, gap, mirror" },
        FilmPreset { id: "stroke", name: "Stroke (stroke)", category: "Tiện ích (Utility)", description: "Kỹ xảo Stroke · tham số: color, width, position, opacity" },
        FilmPreset { id: "metadata_burnin", name: "Metadata & Timecode Burn-in (metadata_burnin)", category: "Hiển thị (Video)", description: "Kỹ xảo Metadata & Timecode Burn-in · tham số: source, position, alignment, size" },
        FilmPreset { id: "echo", name: "Echo (echo)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Echo · tham số: time, count, start, decay" },
        FilmPreset { id: "lightning", name: "Lightning (lightning)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Lightning · tham số: start, end, segments, amplitude" },
        FilmPreset { id: "cell_pattern", name: "Cell Pattern (cell_pattern)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Cell Pattern · tham số: pattern, invert, contrast, overflow" },
        FilmPreset { id: "checkerboard", name: "Checkerboard (checkerboard)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Checkerboard · tham số: anchor, width, height, square" },
        FilmPreset { id: "ellipse", name: "Ellipse (ellipse)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Ellipse · tham số: center, width, height, thickness" },
        FilmPreset { id: "paint_bucket", name: "Paint Bucket (paint_bucket)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Paint Bucket · tham số: point, selector, tolerance, invert" },
        FilmPreset { id: "write_on", name: "Write-on (write_on)", category: "Kỹ xảo cổ điển (Classic)", description: "Kỹ xảo Write-on · tham số: brush, color, size, hardness" },
        FilmPreset { id: "alpha_glow", name: "Alpha Glow (alpha_glow)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Alpha Glow · tham số: glow, brightness, start_color, end_color" },
        FilmPreset { id: "block_dissolve", name: "Block Dissolve (block_dissolve)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Block Dissolve · tham số: completion, block_w, block_h, feather" },
        FilmPreset { id: "directional_blur_legacy", name: "Directional Blur (Legacy) (directional_blur_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Directional Blur (Legacy) · tham số: direction, length" },
        FilmPreset { id: "gaussian_blur_legacy", name: "Gaussian Blur (Legacy) (gaussian_blur_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Gaussian Blur (Legacy) · tham số: blurriness, dimensions" },
        FilmPreset { id: "gradient_wipe_legacy", name: "Gradient Wipe (Legacy) (gradient_wipe_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Gradient Wipe (Legacy) · tham số: completion, softness, layer, placement" },
        FilmPreset { id: "linear_wipe_legacy", name: "Linear Wipe (Legacy) (linear_wipe_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Linear Wipe (Legacy) · tham số: completion, angle, feather" },
        FilmPreset { id: "magnify_legacy", name: "Magnify (Legacy) (magnify_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Magnify (Legacy) · tham số: shape, center, magnification, size" },
        FilmPreset { id: "mosaic_legacy", name: "Mosaic (Legacy) (mosaic_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Mosaic (Legacy) · tham số: horizontal, vertical, sharp" },
        FilmPreset { id: "noise_legacy", name: "Noise (Legacy) (noise_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Noise (Legacy) · tham số: amount, color, clip" },
        FilmPreset { id: "twirl_legacy", name: "Twirl (Legacy) (twirl_legacy)", category: "Kỹ xảo video (Video Effects)", description: "Kỹ xảo Twirl (Legacy) · tham số: angle, radius, center" },
        FilmPreset { id: "vr_blur", name: "VR Blur (vr_blur)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Blur · tham số: frame_layout, blurriness" },
        FilmPreset { id: "vr_chromatic_aberrations", name: "VR Chromatic Aberrations (vr_chromatic_aberrations)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Chromatic Aberrations · tham số: frame_layout, red, green, blue" },
        FilmPreset { id: "vr_color_gradients", name: "VR Color Gradients (vr_color_gradients)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Color Gradients · tham số: frame_layout, c1, p1_lon, p1_lat" },
        FilmPreset { id: "vr_denoise", name: "VR De-Noise (vr_denoise)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR De-Noise · tham số: frame_layout, amount, noise_type, show_noise" },
        FilmPreset { id: "vr_fractal_noise", name: "VR Fractal Noise (vr_fractal_noise)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Fractal Noise · tham số: frame_layout, fractal_type, invert, contrast" },
        FilmPreset { id: "vr_glow", name: "VR Glow (vr_glow)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Glow · tham số: frame_layout, threshold, radius, brightness" },
        FilmPreset { id: "vr_plane_to_sphere", name: "VR Plane to Sphere (vr_plane_to_sphere)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Plane to Sphere · tham số: frame_layout, scale, pan, tilt" },
        FilmPreset { id: "vr_projection", name: "VR Projection (vr_projection)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Projection · tham số: frame_layout, fov, tilt, pan" },
        FilmPreset { id: "vr_rotate_sphere", name: "VR Rotate Sphere (vr_rotate_sphere)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Rotate Sphere · tham số: frame_layout, tilt, pan, roll" },
        FilmPreset { id: "vr_sharpen", name: "VR Sharpen (vr_sharpen)", category: "Không gian Cyber & VR (Immersive)", description: "Kỹ xảo VR Sharpen · tham số: frame_layout, amount" },
        FilmPreset { id: "graphic_text", name: "Text (graphic_text)", category: "Cơ bản (Core)", description: "Kỹ xảo Text · tham số: name, text, font, font_style" },
        FilmPreset { id: "graphic_shape", name: "Shape (graphic_shape)", category: "Cơ bản (Core)", description: "Kỹ xảo Shape · tham số: name, shape, size, sides" },
        FilmPreset { id: "preset:Blur In", name: "Preset · Lấy Nét Đầu Cảnh (Blur In)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Gaussian Blur từ 60 về 0 trong 1 giây đầu" },
        FilmPreset { id: "preset:Blur Out", name: "Preset · Mờ Dần Cuối Cảnh (Blur Out)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Gaussian Blur từ 0 lên 60 trong 1 giây cuối" },
        FilmPreset { id: "preset:Fade In", name: "Preset · Hiện Dần Đầu Cảnh (Fade In)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Độ mờ Opacity 0% → 100% trong 1 giây đầu" },
        FilmPreset { id: "preset:Fade Out", name: "Preset · Tối Dần Cuối Cảnh (Fade Out)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Độ mờ Opacity 100% → 0% trong 1 giây cuối" },
        FilmPreset { id: "preset:Mosaic In", name: "Preset · Giải Mã Ô Vuông (Mosaic In)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Khối pixel lớn hội tụ về ảnh nét trong 1 giây đầu" },
        FilmPreset { id: "preset:Soft Vignette", name: "Preset · Mặt Nạ Tối Viền (Soft Vignette)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Làm tối bên ngoài mặt nạ bầu dục viền mềm" },
        FilmPreset { id: "preset:Spotlight", name: "Preset · Đèn Rọi Tâm (Spotlight)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Làm sáng vùng trung tâm bằng mặt nạ tròn viền mềm" },
        FilmPreset { id: "preset:Warm Tint", name: "Preset · Nhuộm Ấm Hoài Cổ (Warm Tint)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Nhuộm vùng sáng ấm và vùng tối nâu cổ điển" },
        FilmPreset { id: "preset:PiP 35% Upper Right", name: "Preset · Hình Trong Hình 35% (PiP Upper Right)", category: "Preset Động & Mặt Nạ (Animated Presets)", description: "Thu nhỏ 35% đặt ở góc trên bên phải khung hình" },
    ]
}

/// Full catalog of 61 FilmCraft Audio Effects & Audio Transitions.
pub fn audio_effects() -> Vec<FilmPreset> {
    vec![
        FilmPreset { id: "volume", name: "Volume (volume)", category: "Core Audio", description: "Hiệu ứng âm thanh Volume · tham số: bypass, level" },
        FilmPreset { id: "channel_volume", name: "Channel Volume (channel_volume)", category: "Core Audio", description: "Hiệu ứng âm thanh Channel Volume · tham số: bypass, left, right" },
        FilmPreset { id: "panner", name: "Panner (panner)", category: "Core Audio", description: "Hiệu ứng âm thanh Panner · tham số: balance" },
        FilmPreset { id: "constant_power", name: "Constant Power (constant_power)", category: "Crossfade", description: "Hiệu ứng âm thanh Constant Power" },
        FilmPreset { id: "constant_gain", name: "Constant Gain (constant_gain)", category: "Crossfade", description: "Hiệu ứng âm thanh Constant Gain" },
        FilmPreset { id: "exponential_fade", name: "Exponential Fade (exponential_fade)", category: "Crossfade", description: "Hiệu ứng âm thanh Exponential Fade" },
        FilmPreset { id: "amplify", name: "Amplify (amplify)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Amplify · tham số: gain" },
        FilmPreset { id: "channel_mixer_a", name: "Channel Mixer (channel_mixer_a)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Channel Mixer · tham số: l_from_l, l_from_r, invert_l, r_from_l" },
        FilmPreset { id: "channel_volume_a", name: "Channel Volume (channel_volume_a)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Channel Volume · tham số: left, right" },
        FilmPreset { id: "deesser", name: "DeEsser (deesser)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh DeEsser · tham số: frequency, threshold, reduction" },
        FilmPreset { id: "dynamics_rack", name: "Dynamics (dynamics_rack)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Dynamics · tham số: gate_on, gate_threshold, gate_attack, gate_release" },
        FilmPreset { id: "dynamics", name: "Dynamics Processing (dynamics)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Dynamics Processing · tham số: threshold, ratio, attack, release" },
        FilmPreset { id: "hard_limiter", name: "Hard Limiter (hard_limiter)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Hard Limiter · tham số: max, boost, lookahead, release" },
        FilmPreset { id: "multiband_compressor", name: "Multiband Compressor (multiband_compressor)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Multiband Compressor · tham số: xo1, xo2, xo3, b1_threshold" },
        FilmPreset { id: "single_band_compressor", name: "Single-band Compressor (single_band_compressor)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Single-band Compressor · tham số: threshold, ratio, attack, release" },
        FilmPreset { id: "tube_compressor", name: "Tube-modeled Compressor (tube_compressor)", category: "Amplitude and Compression", description: "Hiệu ứng âm thanh Tube-modeled Compressor · tham số: threshold, ratio, attack, release" },
        FilmPreset { id: "analog_delay", name: "Analog Delay (analog_delay)", category: "Delay and Echo", description: "Hiệu ứng âm thanh Analog Delay · tham số: mode, dry, wet, delay" },
        FilmPreset { id: "delay", name: "Delay (delay)", category: "Delay and Echo", description: "Hiệu ứng âm thanh Delay · tham số: delay, feedback, mix" },
        FilmPreset { id: "multitap_delay", name: "Multitap Delay (multitap_delay)", category: "Delay and Echo", description: "Hiệu ứng âm thanh Multitap Delay · tham số: delay1, feedback1, level1, delay2" },
        FilmPreset { id: "bandpass", name: "Bandpass (bandpass)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Bandpass · tham số: center, q" },
        FilmPreset { id: "bass", name: "Bass (bass)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Bass · tham số: boost" },
        FilmPreset { id: "fft_filter", name: "FFT Filter (fft_filter)", category: "Filter and EQ", description: "Hiệu ứng âm thanh FFT Filter · tham số: p1_freq, p1_gain, p2_freq, p2_gain" },
        FilmPreset { id: "graphic_eq", name: "Graphic Equalizer (10 Bands) (graphic_eq)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Graphic Equalizer (10 Bands) · tham số: b1, b2, b3, b4" },
        FilmPreset { id: "graphic_eq_20", name: "Graphic Equalizer (20 Bands) (graphic_eq_20)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Graphic Equalizer (20 Bands) · tham số: b1, b2, b3, b4" },
        FilmPreset { id: "graphic_eq_30", name: "Graphic Equalizer (30 Bands) (graphic_eq_30)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Graphic Equalizer (30 Bands) · tham số: b1, b2, b3, b4" },
        FilmPreset { id: "highpass", name: "Highpass (highpass)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Highpass · tham số: cutoff" },
        FilmPreset { id: "lowpass", name: "Lowpass (lowpass)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Lowpass · tham số: cutoff" },
        FilmPreset { id: "notch", name: "Notch Filter (notch)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Notch Filter · tham số: n1_on, n1_freq, n1_gain, n2_on" },
        FilmPreset { id: "parametric_eq", name: "Parametric Equalizer (parametric_eq)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Parametric Equalizer · tham số: master_gain, hp_on, hp_freq, hp_slope" },
        FilmPreset { id: "scientific_filter", name: "Scientific Filter (scientific_filter)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Scientific Filter · tham số: type, mode, order, cutoff" },
        FilmPreset { id: "simple_notch", name: "Simple Notch Filter (simple_notch)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Simple Notch Filter · tham số: center, q" },
        FilmPreset { id: "simple_eq", name: "Simple Parametric EQ (simple_eq)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Simple Parametric EQ · tham số: center, q, boost" },
        FilmPreset { id: "treble", name: "Treble (treble)", category: "Filter and EQ", description: "Hiệu ứng âm thanh Treble · tham số: boost" },
        FilmPreset { id: "chorus_flanger", name: "Chorus/Flanger (chorus_flanger)", category: "Modulation", description: "Hiệu ứng âm thanh Chorus/Flanger · tham số: mode, speed, width, intensity" },
        FilmPreset { id: "flanger", name: "Flanger (flanger)", category: "Modulation", description: "Hiệu ứng âm thanh Flanger · tham số: initial_delay, final_delay, stereo_phasing, feedback" },
        FilmPreset { id: "phaser", name: "Phaser (phaser)", category: "Modulation", description: "Hiệu ứng âm thanh Phaser · tham số: stages, intensity, depth, rate" },
        FilmPreset { id: "declicker", name: "Automatic Click Remover (declicker)", category: "Noise Reduction/Restoration", description: "Hiệu ứng âm thanh Automatic Click Remover · tham số: threshold, complexity" },
        FilmPreset { id: "dehummer", name: "DeHummer (dehummer)", category: "Noise Reduction/Restoration", description: "Hiệu ứng âm thanh DeHummer · tham số: freq, gain, harmonics, q" },
        FilmPreset { id: "denoise", name: "DeNoise (denoise)", category: "Noise Reduction/Restoration", description: "Hiệu ứng âm thanh DeNoise · tham số: amount" },
        FilmPreset { id: "dereverb", name: "DeReverb (dereverb)", category: "Noise Reduction/Restoration", description: "Hiệu ứng âm thanh DeReverb · tham số: amount, rt60" },
        FilmPreset { id: "speech_enhance", name: "Enhance Speech (speech_enhance)", category: "Noise Reduction/Restoration", description: "Hiệu ứng âm thanh Enhance Speech · tham số: mix, tone" },
        FilmPreset { id: "convolution_reverb", name: "Convolution Reverb (convolution_reverb)", category: "Reverb", description: "Hiệu ứng âm thanh Convolution Reverb · tham số: impulse, mix, room_size, damping_lf" },
        FilmPreset { id: "studio_reverb", name: "Studio Reverb (studio_reverb)", category: "Reverb", description: "Hiệu ứng âm thanh Studio Reverb · tham số: room, decay, damping, dry" },
        FilmPreset { id: "surround_reverb", name: "Surround Reverb (surround_reverb)", category: "Reverb", description: "Hiệu ứng âm thanh Surround Reverb · tham số: center_input, room_size, decay, predelay" },
        FilmPreset { id: "binauralizer", name: "Binauralizer - Ambisonics (binauralizer)", category: "Special", description: "Hiệu ứng âm thanh Binauralizer - Ambisonics · tham số: angle, head_size, mix" },
        FilmPreset { id: "distortion", name: "Distortion (distortion)", category: "Special", description: "Hiệu ứng âm thanh Distortion · tham số: drive, curve, symmetry, tone" },
        FilmPreset { id: "fill_left", name: "Fill Left with Right (fill_left)", category: "Special", description: "Hiệu ứng âm thanh Fill Left with Right" },
        FilmPreset { id: "fill_right", name: "Fill Right with Left (fill_right)", category: "Special", description: "Hiệu ứng âm thanh Fill Right with Left" },
        FilmPreset { id: "guitar_suite", name: "GuitarSuite (guitar_suite)", category: "Special", description: "Hiệu ứng âm thanh GuitarSuite · tham số: compressor, distortion, dist_type, amp" },
        FilmPreset { id: "invert_a", name: "Invert (invert_a)", category: "Special", description: "Hiệu ứng âm thanh Invert" },
        FilmPreset { id: "loudness_radar", name: "Loudness Meter (loudness_radar)", category: "Special", description: "Hiệu ứng âm thanh Loudness Meter · tham số: target" },
        FilmPreset { id: "mastering", name: "Mastering (mastering)", category: "Special", description: "Hiệu ứng âm thanh Mastering · tham số: eq_low, eq_mid_freq, eq_mid, eq_high" },
        FilmPreset { id: "panner_ambisonics", name: "Panner - Ambisonics (panner_ambisonics)", category: "Special", description: "Hiệu ứng âm thanh Panner - Ambisonics · tham số: pan, tilt, roll" },
        FilmPreset { id: "swap_channels", name: "Swap Channels (swap_channels)", category: "Special", description: "Hiệu ứng âm thanh Swap Channels" },
        FilmPreset { id: "vocal_enhancer", name: "Vocal Enhancer (vocal_enhancer)", category: "Special", description: "Hiệu ứng âm thanh Vocal Enhancer · tham số: mode" },
        FilmPreset { id: "stereo_expander", name: "Stereo Expander (stereo_expander)", category: "Stereo Imagery", description: "Hiệu ứng âm thanh Stereo Expander · tham số: center_pan, expand" },
        FilmPreset { id: "stereo_width", name: "Stereo Width (stereo_width)", category: "Stereo Imagery", description: "Hiệu ứng âm thanh Stereo Width · tham số: width" },
        FilmPreset { id: "pitch_shifter", name: "Pitch Shifter (pitch_shifter)", category: "Time and Pitch", description: "Hiệu ứng âm thanh Pitch Shifter · tham số: semitones, cents" },
        FilmPreset { id: "balance_a", name: "Balance (balance_a)", category: "Core Audio", description: "Hiệu ứng âm thanh Balance · tham số: balance" },
        FilmPreset { id: "mute", name: "Mute (mute)", category: "Core Audio", description: "Hiệu ứng âm thanh Mute · tham số: mute" },
        FilmPreset { id: "volume_a", name: "Volume (volume_a)", category: "Core Audio", description: "Hiệu ứng âm thanh Volume · tham số: bypass, level" },
    ]
}

/// Media facts from FilmCraft, or None when FilmCraft isn't built or can't read the file.
pub fn probe(p: &Path) -> Option<MediaInfo> {
    let exe = Craft::Film.locate()?;
    let path = p.to_string_lossy().to_string();
    let out = run(&exe, &["probe", &path, "--compact"], None, Duration::from_secs(30)).ok()?;
    let v: Value = serde_json::from_str(out.stdout.trim()).ok()?;
    parse_probe(&v)
}

/// FilmCraft's time base: 254 016 000 000 ticks per second (`filmcraft_time::Tick`).
pub const TICKS_PER_SEC: f64 = 254_016_000_000.0;

/// Pull duration / size / audio out of FilmCraft's probe JSON:
/// `{"duration": <ticks>, "video": {"width", "height", …} | null, "audio_streams": [...]}`.
pub fn parse_probe(v: &Value) -> Option<MediaInfo> {
    let ticks = v.get("duration").and_then(Value::as_f64).unwrap_or(0.0);
    let secs = ticks / TICKS_PER_SEC;
    // Anything past a day is a misread, not a clip.
    if !(0.0..86_400.0).contains(&secs) {
        return None;
    }
    let video = v.get("video").filter(|x| x.is_object());
    let dim = |k: &str| video.and_then(|x| x.get(k)).and_then(Value::as_u64).unwrap_or(0) as u32;
    let info = MediaInfo {
        duration_us: crate::sec_to_us(secs),
        width: dim("width"),
        height: dim("height"),
        has_audio: v.get("audio_streams").and_then(Value::as_array).is_some_and(|a| !a.is_empty()),
        source: "filmcraft",
    };
    (info.duration_us > 0 || info.width > 0).then_some(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_reads_ticks() {
        let v: Value = serde_json::from_str(r#"{"audio_streams":[{"channels":2}],"duration":45763522560000,"video":null}"#).unwrap();
        let i = parse_probe(&v).unwrap();
        assert_eq!(i.duration_us / 1000, 180_160);
        assert!(i.has_audio);
        let v: Value = serde_json::from_str(r#"{"audio_streams":[],"duration":1270080000000,"video":{"width":1920,"height":1080}}"#).unwrap();
        let i = parse_probe(&v).unwrap();
        assert_eq!((i.duration_us, i.width, i.height, i.has_audio), (5_000_000, 1920, 1080, false));
    }

    #[test]
    fn full_catalog_counts() {
        assert_eq!(transitions().len(), 112);
        assert_eq!(luts().len(), 26);
        assert_eq!(video_effects().len(), 139);
        assert_eq!(audio_effects().len(), 61);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PreviewOutput {
    pub mp4: String,
    pub project: String,
}

/// One clip of the preview sequence.
pub struct PreviewClip<'a> {
    pub path: &'a Path,
    /// "V1", "V2"… or "A1", "A2"…
    pub track: String,
    pub start_sec: f64,
    pub dur_sec: f64,
    pub src_in_sec: f64,
    /// Audio tracks take the level of their first clip (track volume in dB).
    pub volume: f64,
    pub transition: Option<String>,
    pub transition_sec: f64,
    pub lut: Option<String>,
    pub video_effect: Option<String>,
    pub zoom: Option<(f64, f64)>,
    /// Overlay position, size and opacity as the draft has them; `None` for full-frame clips.
    pub place: Option<Place>,
}

/// Where an overlay sits, in CapCut's terms: offset in half-canvas units (y up), size relative
/// to fitting the canvas, opacity 0-1; plus the media's pixel size, to work out that fit.
#[derive(Debug, Clone, Copy)]
pub struct Place {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub opacity: f64,
    pub width: u32,
    pub height: u32,
}

fn ticks(sec: f64) -> i64 {
    (sec.max(0.0) * TICKS_PER_SEC).round() as i64
}

fn track_no(t: &str) -> usize {
    t.get(1..).and_then(|n| n.parse().ok()).unwrap_or(1)
}

/// Rebuild the cut as a FilmCraft sequence (`.fcproj`) and export a half-size H.264 MP4 of it.
/// Covers placement, trims, transitions, LUTs, video effects, dynamic zoom, overlay position /
/// size / opacity and track levels.
pub fn preview(clips: &[PreviewClip], w: u32, h: u32, fps: f64, out_dir: &Path, stem: &str) -> Result<PreviewOutput> {
    let exe = Craft::Film.require()?;
    if clips.is_empty() {
        bail!("không có clip để xem trước");
    }
    std::fs::create_dir_all(out_dir)?;
    let project = out_dir.join(format!("{stem}.fcproj"));
    let mp4 = out_dir.join(format!("{stem}-preview.mp4"));
    let _ = std::fs::remove_file(&project);
    let mut paths: Vec<String> = vec![];
    for c in clips {
        let p = c.path.to_string_lossy().replace('\\', "/");
        if !paths.contains(&p) {
            paths.push(p);
        }
    }
    let nv = clips.iter().filter(|c| c.track.starts_with('V')).map(|c| track_no(&c.track)).max().unwrap_or(1);
    let na = clips.iter().filter(|c| c.track.starts_with('A')).map(|c| track_no(&c.track)).max().unwrap_or(1);

    // 1. sequence + media
    let s1 = [
        serde_json::json!({"id": "file.newSequence", "params": {"name": stem, "width": w, "height": h, "fps": fps, "video": nv, "audio": na}}),
        serde_json::json!({"id": "file.import", "params": {"paths": paths}}),
    ];
    let f1 = out_dir.join(format!("{stem}.1.jsonl"));
    std::fs::write(&f1, s1.iter().map(Value::to_string).collect::<Vec<_>>().join("\n"))?;
    let (f1a, proj_a) = (f1.to_string_lossy().to_string(), project.to_string_lossy().to_string());
    let out = run(&exe, &["run", &f1a, "--save-as", &proj_a, "--compact"], Some(out_dir), Duration::from_secs(120))?;
    let items: Vec<u64> = out
        .stdout
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .find(|v| v["id"] == "file.import")
        .and_then(|v| v["result"]["items"].as_array().map(|a| a.iter().filter_map(Value::as_u64).collect()))
        .unwrap_or_default();
    if items.len() != paths.len() {
        bail!("FilmCraft chỉ nhập được {}/{} file", items.len(), paths.len());
    }

    // 2. place every clip, set audio track levels
    let mut lines = vec![];
    let mut levels: Vec<(String, f64)> = vec![];
    for c in clips {
        let p = c.path.to_string_lossy().replace('\\', "/");
        let Some(k) = paths.iter().position(|x| *x == p) else { continue };
        let audio_track = c.track.starts_with('A');
        let mut params = serde_json::json!({
            "item": items[k], "track": c.track, "seconds": c.start_sec,
            "sourceIn": ticks(c.src_in_sec), "duration": ticks(c.dur_sec),
        });
        if !audio_track {
            params["audio"] = Value::Bool(false);
        }
        lines.push(serde_json::json!({"id": "timeline.place", "params": params}));
        if audio_track && !levels.iter().any(|(t, _)| *t == c.track) {
            levels.push((c.track.clone(), c.volume));
        }
    }
    for (t, v) in levels {
        let db = if v <= 0.001 { -60.0 } else { (20.0 * v.log10()).clamp(-60.0, 12.0) };
        lines.push(serde_json::json!({"id": "timeline.setTrack", "params": {"track": t, "volumeDb": db}}));
    }
    let f2 = out_dir.join(format!("{stem}.2.jsonl"));
    std::fs::write(&f2, lines.iter().map(Value::to_string).collect::<Vec<_>>().join("\n"))?;
    let f2a = f2.to_string_lossy().to_string();
    let placed = run(&exe, &["--project", &proj_a, "run", &f2a, "--save", "--compact"], Some(out_dir), Duration::from_secs(120))?;

    // Map each placed clip to its FilmCraft clip id
    let placed_ids: Vec<(usize, u64)> = placed
        .stdout
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|v| v["id"] == "timeline.place")
        .enumerate()
        .filter_map(|(idx, v)| v["result"]["clips"].as_array()?.first()?.as_u64().map(|cid| (idx, cid)))
        .collect();

    let visual: Vec<u64> = placed_ids
        .iter()
        .filter(|&&(idx, _)| clips.get(idx).is_some_and(|c| c.track.starts_with('V')))
        .map(|&(_, cid)| cid)
        .collect();
    if !visual.is_empty() {
        let params = serde_json::json!({ "clips": visual }).to_string();
        run(&exe, &["--project", &proj_a, "exec", "clip.fitToFrame", &params, "--save", "--compact"], Some(out_dir), Duration::from_secs(120))?;
    }

    // 2b. Apply transitions, LUTs/Lumetri presets, video effects/presets, and dynamic zoom on V1/visual clips
    let mut fx_lines = vec![];
    for &(idx, cid) in &placed_ids {
        let Some(c) = clips.get(idx) else { continue };
        if !c.track.starts_with('V') {
            continue;
        }
        if let Some(tr) = c.transition.as_deref().filter(|s| !s.trim().is_empty() && *s != "none") {
            let frames = ((c.transition_sec.clamp(0.15, 1.5)) * fps).round() as i64;
            fx_lines.push(serde_json::json!({
                "id": "sequence.applyVideoTransition",
                "params": {"clip": cid, "effect": tr, "frames": frames.max(4), "edge": "in"}
            }));
        }
        if let Some(lut) = c.lut.as_deref().filter(|s| !s.trim().is_empty() && *s != "none") {
            if let Some(preset_name) = lut.strip_prefix("lumetri:") {
                fx_lines.push(serde_json::json!({"id": "lumetri.applyPreset", "params": {"name": preset_name, "clips": [cid]}}));
            } else {
                fx_lines.push(serde_json::json!({"id": "effects.apply", "params": {"clips": [cid], "effect": "lumetri"}}));
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": "lumetri", "param": "creative_on", "value": true}}));
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": "lumetri", "param": "look_lut", "value": lut}}));
            }
        }
        if let Some(vfx) = c.video_effect.as_deref().filter(|s| !s.trim().is_empty() && *s != "none") {
            if let Some(preset_name) = vfx.strip_prefix("preset:") {
                fx_lines.push(serde_json::json!({"id": "presets.apply", "params": {"preset": preset_name, "clips": [cid]}}));
            } else {
                fx_lines.push(serde_json::json!({"id": "effects.apply", "params": {"clips": [cid], "effect": vfx}}));
            }
        }
        if let Some(pl) = c.place {
            // after clip.fitToFrame: scale is a percentage of the media, position is in sequence pixels
            let (fw, fh) = (f64::from(w), f64::from(h));
            if (pl.scale - 1.0).abs() > 1e-6 || pl.x.abs() > 1e-6 || pl.y.abs() > 1e-6 {
                let fit = if pl.width > 0 && pl.height > 0 { (fw / f64::from(pl.width)).min(fh / f64::from(pl.height)) * 100.0 } else { 100.0 };
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": "motion", "param": "scale", "value": fit * pl.scale}}));
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": "motion", "param": "position", "value": [fw / 2.0 * (1.0 + pl.x), fh / 2.0 * (1.0 - pl.y)]}}));
            }
            if pl.opacity < 0.999 {
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": "opacity", "param": "opacity", "value": pl.opacity.clamp(0.0, 1.0) * 100.0}}));
            }
        }
        if let Some((z0, z1)) = c.zoom {
            if (z1 - z0).abs() > 0.005 {
                let eff = if z1 >= z0 { "grow" } else { "shrink" };
                fx_lines.push(serde_json::json!({"id": "effects.apply", "params": {"clips": [cid], "effect": eff}}));
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": eff, "param": "from", "value": z0 * 100.0}}));
                fx_lines.push(serde_json::json!({"id": "effects.setParam", "params": {"clip": cid, "effect": eff, "param": "to", "value": z1 * 100.0}}));
            }
        }
    }
    if !fx_lines.is_empty() {
        let f3 = out_dir.join(format!("{stem}.3.jsonl"));
        std::fs::write(&f3, fx_lines.iter().map(Value::to_string).collect::<Vec<_>>().join("\n"))?;
        let f3a = f3.to_string_lossy().to_string();
        let _ = run(&exe, &["--project", &proj_a, "run", &f3a, "--save", "--compact", "--keep-going"], Some(out_dir), Duration::from_secs(120));
        let _ = std::fs::remove_file(&f3);
    }

    // 3. export
    let scale = (960.0 / w.max(h) as f64).min(1.0);
    let (mp4a, scale_a) = (mp4.to_string_lossy().to_string(), format!("{scale:.3}"));
    run(&exe, &["--project", &proj_a, "export", &mp4a, "--scale", &scale_a, "--allow-offline", "--compact"], Some(out_dir), Duration::from_secs(1800))?;
    if !mp4.is_file() {
        bail!("FilmCraft không xuất ra {}", mp4.display());
    }
    let _ = std::fs::remove_file(&f1);
    let _ = std::fs::remove_file(&f2);
    let s = |p: &Path| crate::paths::clean_path_str(&p.to_string_lossy());
    Ok(PreviewOutput { mp4: s(&mp4), project: s(&project) })
}
