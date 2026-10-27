//! EffectCraft: a motion preset becomes an After Effects-style script (comp, imported graphic,
//! keyframes, effects) run by `effectcraft-cli script`, saved as `.ecproj`, then rendered by
//! `effectcraft-cli render` to an alpha video for a CapCut overlay track. The `.ecproj` stays next
//! to the render so the animation can be opened and refined in EffectCraft.

use anyhow::{bail, Result};
use serde::Serialize;
use std::path::Path;
use std::time::Duration;

use super::{run, vector::short_hash, Craft};
use crate::paths::clean_path_str;

/// Bump when the generated script changes, so cached renders of the old script are not reused.
const SCRIPT_REV: u32 = 5;

#[derive(Debug, Clone, Serialize)]
pub struct MotionPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
}

/// 70 Motion & Animation Presets (16 CapCut Pro Motion + 8 Kinetic Text + 23 Built-in Layer Transitions + 23 Animated Backgrounds).
pub fn presets() -> Vec<MotionPreset> {
    vec![
        MotionPreset { id: "fade", name: "Mờ dần (Fade In/Out)", category: "CapCut Pro Motion", description: "Hiện dần, giữ, mờ dần" },
        MotionPreset { id: "slide_left", name: "Trượt từ trái (Slide Left)", category: "CapCut Pro Motion", description: "Trượt vào từ trái có easing, trượt ra cùng hướng" },
        MotionPreset { id: "slide_right", name: "Trượt từ phải (Slide Right)", category: "CapCut Pro Motion", description: "Trượt vào từ phải, trượt ra cùng hướng" },
        MotionPreset { id: "rise", name: "Nổi lên (Rise Up)", category: "CapCut Pro Motion", description: "Nhô lên từ dưới và hiện dần" },
        MotionPreset { id: "pop", name: "Bật ra (Elastic Pop)", category: "CapCut Pro Motion", description: "Phóng to vượt mức rồi về 100%" },
        MotionPreset { id: "blur_in", name: "Lấy nét (Gaussian Focus)", category: "CapCut Pro Motion", description: "Từ mờ nhoè (Gaussian Blur) về nét" },
        MotionPreset { id: "wipe", name: "Quét ngang (Linear Wipe)", category: "CapCut Pro Motion", description: "Linear Wipe mở từ trái sang phải" },
        MotionPreset { id: "drift", name: "Trôi chậm (Slow Drift)", category: "CapCut Pro Motion", description: "Phóng chậm 100→106% suốt thời gian, hiện/mờ ở hai đầu" },
        MotionPreset { id: "glow_pop", name: "Bật sáng Neon (Glow Pop)", category: "CapCut Pro Motion", description: "Phóng nảy đàn hồi + hào quang Glow phát sáng + bóng đổ" },
        MotionPreset { id: "light_sweep", name: "Quét tia sáng (CC Light Sweep)", category: "CapCut Pro Motion", description: "Trượt mượt + luồng sáng CC Light Sweep lướt ngang chữ" },
        MotionPreset { id: "light_wipe", name: "Quét Laser (Cyan Laser Wipe)", category: "CapCut Pro Motion", description: "Mở dần từ trái kèm tia sáng Cyan rực rỡ và hào quang" },
        MotionPreset { id: "glitch_in", name: "Nhiễu số Cyber (RGB Glitch)", category: "CapCut Pro Motion", description: "Tách kênh màu RGB + nhiễu kỹ thuật số lúc vào và ra" },
        MotionPreset { id: "zoom_punch", name: "Đập Nhịp Bass (Zoom Punch)", category: "CapCut Pro Motion", description: "Phóng đập mạnh từ 142% về 100% + Hào quang bùng sáng theo nhịp" },
        MotionPreset { id: "shake_hit", name: "Va Đập Rung Chấn (Shake Hit)", category: "CapCut Pro Motion", description: "Va đập rung giật + tách màu RGB + Glow cường độ cao" },
        MotionPreset { id: "neon_hud", name: "HUD Siêu Xe (Supercar HUD)", category: "CapCut Pro Motion", description: "Mở cổng kỹ thuật số + tia sáng Đỏ/Cyan lướt kép + Neon Glow" },
        MotionPreset { id: "flip_3d", name: "Xoắn Động Học (3D Kinetic Flip)", category: "CapCut Pro Motion", description: "Bật nảy đàn hồi kèm tia sáng vàng quét chéo tốc độ cao" },
        MotionPreset { id: "typewriter", name: "Gõ Chữ Máy Đánh Chữ (Typewriter)", category: "Text & Kinetic Animation", description: "Hiện từng ký tự sắc nét từ trái sang phải kiểu gõ phím" },
        MotionPreset { id: "fadeUpCharacters", name: "Chữ Nổi Dần Lên (Fade Up Characters)", category: "Text & Kinetic Animation", description: "Chữ hiện dần kèm nhô nhẹ từ dưới lên mượt mà" },
        MotionPreset { id: "bounceInWords", name: "Từ Bật Nảy Đàn Hồi (Bounce In Words)", category: "Text & Kinetic Animation", description: "Phóng nảy đàn hồi vượt ngưỡng rồi ổn định từng nhịp" },
        MotionPreset { id: "trackingIn", name: "Thu Hẹp Khoảng Cách Chữ (Tracking In)", category: "Text & Kinetic Animation", description: "Co giãn ngang từ rộng về chuẩn kèm hiện dần điện ảnh" },
        MotionPreset { id: "scramble", name: "Giải Mã Ký Tự Số (Cyber Scramble)", category: "Text & Kinetic Animation", description: "Nhấp nháy giải mã kỹ thuật số kèm tia sáng quét trước khi hội tụ" },
        MotionPreset { id: "blurIn", name: "Hội Tụ Tiêu Cự Chữ (Optical Blur In)", category: "Text & Kinetic Animation", description: "Lấy nét quang học từ mờ sang sắc nét kèm ánh sáng dịu" },
        MotionPreset { id: "jitter", name: "Rung Động Năng Lượng (Kinetic Jitter)", category: "Text & Kinetic Animation", description: "Dao động vị trí và góc nghiêng liên tục đầy năng lượng" },
        MotionPreset { id: "dropInLines", name: "Rơi Từng Dòng Chữ (Drop In Lines)", category: "Text & Kinetic Animation", description: "Rơi từ trên xuống kèm góc nghiêng nhẹ rồi khớp vào vị trí" },
        MotionPreset { id: "fadeIn", name: "Hiện Dần Mượt (Fade In)", category: "Layer Transition (Built-in)", description: "Hiện dần từ trong suốt lên 100% trong 0.5 giây" },
        MotionPreset { id: "fadeOut", name: "Mờ Dần Cuối (Fade Out)", category: "Layer Transition (Built-in)", description: "Mờ dần về trong suốt ở cuối thời lượng" },
        MotionPreset { id: "wipeInFromLeft", name: "Quét Mở Từ Trái (Wipe In From Left)", category: "Layer Transition (Built-in)", description: "Mép mềm quét từ trái sang phải mở dần layer" },
        MotionPreset { id: "wipeInFromRight", name: "Quét Mở Từ Phải (Wipe In From Right)", category: "Layer Transition (Built-in)", description: "Mép mềm quét từ phải sang trái mở dần layer" },
        MotionPreset { id: "wipeInFromTop", name: "Quét Mở Từ Trên (Wipe In From Top)", category: "Layer Transition (Built-in)", description: "Mở dần layer từ mép trên xuống dưới" },
        MotionPreset { id: "wipeInFromBottom", name: "Quét Mở Từ Dưới (Wipe In From Bottom)", category: "Layer Transition (Built-in)", description: "Mở dần layer từ mép dưới lên trên" },
        MotionPreset { id: "diagonalSweepIn", name: "Quét Chéo Mềm (Diagonal Sweep In)", category: "Layer Transition (Built-in)", description: "Mép quét chéo siêu mịn từ góc dưới trái" },
        MotionPreset { id: "wipeOutToRight", name: "Quét Đóng Sang Phải (Wipe Out To Right)", category: "Layer Transition (Built-in)", description: "Mép mềm lướt từ trái sang phải thu gọn layer" },
        MotionPreset { id: "clockReveal", name: "Kim Đồng Hồ Mở (Clock Reveal)", category: "Layer Transition (Built-in)", description: "Quét xoay tròn một vòng quanh tâm để mở layer" },
        MotionPreset { id: "fanOpen", name: "Mở Cánh Quạt Đôi (Fan Open)", category: "Layer Transition (Built-in)", description: "Mở đối xứng hai bên từ đỉnh xuống như cánh quạt" },
        MotionPreset { id: "clockWipeOut", name: "Kim Đồng Hồ Đóng (Clock Wipe Out)", category: "Layer Transition (Built-in)", description: "Quét xoay tròn quanh tâm để thu gọn layer" },
        MotionPreset { id: "blindsOpen", name: "Mở Rèm Dọc (Blinds Open)", category: "Layer Transition (Built-in)", description: "Các nan dọc mở rộng dần cho tới khi hiện trọn layer" },
        MotionPreset { id: "slatsDown", name: "Mở Nan Ngang (Slats Down)", category: "Layer Transition (Built-in)", description: "Các thanh ngang dày dần lên ghép thành layer hoàn chỉnh" },
        MotionPreset { id: "blindsClose", name: "Đóng Rèm Dọc (Blinds Close)", category: "Layer Transition (Built-in)", description: "Các nan dọc khép hẹp dần làm biến mất layer" },
        MotionPreset { id: "pixelDissolveIn", name: "Tan Khối Pixel Vào (Pixel Dissolve In)", category: "Layer Transition (Built-in)", description: "Các ô vuông sắc cạnh xuất hiện ngẫu nhiên ghép thành layer" },
        MotionPreset { id: "mosaicFadeIn", name: "Khảm Mờ Hiện Dần (Mosaic Fade In)", category: "Layer Transition (Built-in)", description: "Các ô gạch lớn viền mềm hiện ngẫu nhiên ghép lại" },
        MotionPreset { id: "pixelDissolveOut", name: "Tan Khối Pixel Ra (Pixel Dissolve Out)", category: "Layer Transition (Built-in)", description: "Các ô vuông rơi rụng ngẫu nhiên cho tới khi hết layer" },
        MotionPreset { id: "cardFlipIn", name: "Lật Thẻ Bài Mở (Card Flip In)", category: "Layer Transition (Built-in)", description: "Lưới thẻ 3D lật từ trái sang phải để lộ diện layer" },
        MotionPreset { id: "cardFlipOut", name: "Lật Thẻ Bài Đóng (Card Flip Out)", category: "Layer Transition (Built-in)", description: "Lưới thẻ 3D lật từ trái sang phải khép layer lại" },
        MotionPreset { id: "gridDoorsIn", name: "Cửa Lưới Mở Tâm (Grid Doors In)", category: "Layer Transition (Built-in)", description: "Các ô cửa nhỏ mở bung từ tâm ra ngoài" },
        MotionPreset { id: "lensPopIn", name: "Thấu Kính Bung Mở (Lens Pop In)", category: "Layer Transition (Built-in)", description: "Bung mở từ vòng tròn khúc xạ quang học ở tâm" },
        MotionPreset { id: "lineSweepIn", name: "Quét Sọc Chéo (Line Sweep In)", category: "Layer Transition (Built-in)", description: "Mép cắt bậc thang nghiêng quét nhanh từ góc dưới trái" },
        MotionPreset { id: "jawsCloseIn", name: "Hàm Răng Khép Mở (Jaws Close In)", category: "Layer Transition (Built-in)", description: "Hai mép răng cưa trên dưới khép lại hiện trọn layer" },
        MotionPreset { id: "softGradientDrift", name: "Soft Gradient Drift (softGradientDrift)", category: "Animated Background (Built-in)", description: "Four soft colour corners slowly drift around the frame." },
        MotionPreset { id: "auroraVeil", name: "Aurora Veil (auroraVeil)", category: "Animated Background (Built-in)", description: "Tall green curtains of light ripple slowly across a night sky." },
        MotionPreset { id: "slowFog", name: "Slow Fog (slowFog)", category: "Animated Background (Built-in)", description: "Heavy grey-blue fog banks roll gently through the frame." },
        MotionPreset { id: "cloudBank", name: "Cloud Bank (cloudBank)", category: "Animated Background (Built-in)", description: "Soft cumulus clouds drift across a blue sky." },
        MotionPreset { id: "darkSmoke", name: "Dark Smoke (darkSmoke)", category: "Animated Background (Built-in)", description: "Thick dark smoke rises slowly and curls as it goes." },
        MotionPreset { id: "emberHaze", name: "Ember Haze (emberHaze)", category: "Animated Background (Built-in)", description: "A warm glowing haze of embers, black to deep red to amber." },
        MotionPreset { id: "magmaFlow", name: "Magma Flow (magmaFlow)", category: "Animated Background (Built-in)", description: "Fire-coloured plasma that slowly cycles through its palette." },
        MotionPreset { id: "iceFlow", name: "Ice Flow (iceFlow)", category: "Animated Background (Built-in)", description: "Cold blue and white veins drift through a frozen field." },
        MotionPreset { id: "pastelPlasma", name: "Pastel Plasma (pastelPlasma)", category: "Animated Background (Built-in)", description: "Soft pastel plasma with colours that endlessly cycle." },
        MotionPreset { id: "cellDrift", name: "Cell Drift (cellDrift)", category: "Animated Background (Built-in)", description: "Bubble-like cells slowly swell and merge in teal tones." },
        MotionPreset { id: "crystalFacets", name: "Crystal Facets (crystalFacets)", category: "Animated Background (Built-in)", description: "Sharp crystal facets shimmer in warm violet light." },
        MotionPreset { id: "hexMosaic", name: "Hex Mosaic (hexMosaic)", category: "Animated Background (Built-in)", description: "A hexagon mosaic that glows as fog passes beneath it." },
        MotionPreset { id: "mirroredSilk", name: "Mirrored Silk (mirroredSilk)", category: "Animated Background (Built-in)", description: "Flowing silk ribbons mirrored down the centre." },
        MotionPreset { id: "waterCaustics", name: "Water Caustics (waterCaustics)", category: "Animated Background (Built-in)", description: "Bright caustic filaments shimmer across a turquoise pool floor." },
        MotionPreset { id: "blueprintGrid", name: "Blueprint Grid (blueprintGrid)", category: "Animated Background (Built-in)", description: "A technical grid slides across a deep blue plate." },
        MotionPreset { id: "checkerScroll", name: "Checker Scroll (checkerScroll)", category: "Animated Background (Built-in)", description: "A two-tone checkerboard scrolls diagonally." },
        MotionPreset { id: "scrollingStripes", name: "Scrolling Stripes (scrollingStripes)", category: "Animated Background (Built-in)", description: "Wide vertical stripes slide steadily sideways." },
        MotionPreset { id: "pulseRings", name: "Pulse Rings (pulseRings)", category: "Animated Background (Built-in)", description: "Soft rings expand from the centre in a steady rhythm." },
        MotionPreset { id: "bokehDrift", name: "Bokeh Drift (bokehDrift)", category: "Animated Background (Built-in)", description: "Out-of-focus lights float lazily over a dark violet backdrop." },
        MotionPreset { id: "spotlightFloor", name: "Spotlight Floor (spotlightFloor)", category: "Animated Background (Built-in)", description: "A soft pool of light on a dark floor that gently breathes." },
        MotionPreset { id: "dawnSky", name: "Dawn Sky (dawnSky)", category: "Animated Background (Built-in)", description: "A night sky warms into sunrise over six seconds." },
        MotionPreset { id: "sweepingLight", name: "Sweeping Light (sweepingLight)", category: "Animated Background (Built-in)", description: "A band of light sweeps repeatedly across a dark slate backdrop." },
        MotionPreset { id: "grainyWash", name: "Grainy Wash (grainyWash)", category: "Animated Background (Built-in)", description: "A muted diagonal wash with live film grain." },
    ]
}

/// Full catalog of 307 After Effects-style Layer Effects built into EffectCraft (`effect.list`).
/// Any `ec.*` ID can also be passed directly as `motion.preset`!
pub fn layer_effects() -> Vec<MotionPreset> {
    vec![
        MotionPreset { id: "ec.3d.channelextract", name: "3D Channel Extract (ec.3d.channelextract)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · 3D Channel Extract (3D Channel, Black Point, White Point, Clamp Output)" },
        MotionPreset { id: "ec.3d.cryptomatte", name: "Cryptomatte (ec.3d.cryptomatte)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · Cryptomatte (Layer, Selection, Display, Matte Only)" },
        MotionPreset { id: "ec.3d.depthmatte", name: "Depth Matte (ec.3d.depthmatte)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · Depth Matte (Depth, Feather, Invert)" },
        MotionPreset { id: "ec.3d.depthoffield", name: "Depth of Field (ec.3d.depthoffield)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · Depth of Field (Focal Plane, Maximum Radius, Focal Plane Thickness, Focal Bias)" },
        MotionPreset { id: "ec.3d.extractor", name: "EXtractoR (ec.3d.extractor)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · EXtractoR (Red, Green, Blue, Alpha)" },
        MotionPreset { id: "ec.3d.fog3d", name: "Fog 3D (ec.3d.fog3d)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · Fog 3D (Fog Color, Fog Start Depth, Fog End Depth, Fog Opacity)" },
        MotionPreset { id: "ec.3d.idmatte", name: "ID Matte (ec.3d.idmatte)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · ID Matte (Aux. Channel, ID Selection, Feather, Invert)" },
        MotionPreset { id: "ec.3d.identifier", name: "IDentifier (ec.3d.identifier)", category: "AE Layer FX · 3D Channel", description: "EffectCraft 3D Channel · IDentifier (Channel Type, Display, ID)" },
        MotionPreset { id: "ec.audio.backwards", name: "Backwards (ec.audio.backwards)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Backwards (Swap Channels)" },
        MotionPreset { id: "ec.audio.basstreble", name: "Bass & Treble (ec.audio.basstreble)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Bass & Treble (Bass, Treble)" },
        MotionPreset { id: "ec.audio.compressor", name: "Compressor (ec.audio.compressor)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Compressor (Threshold (dB), Ratio (x:1), Knee (dB), Attack (ms))" },
        MotionPreset { id: "ec.audio.delay", name: "Delay (ec.audio.delay)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Delay (Delay Time (milliseconds), Delay Amount, Feedback, Dry Out)" },
        MotionPreset { id: "ec.audio.distortion", name: "Distortion (ec.audio.distortion)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Distortion (Type, Drive, Gain, Mix)" },
        MotionPreset { id: "ec.audio.flangechorus", name: "Flange & Chorus (ec.audio.flangechorus)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Flange & Chorus (Voice Separation Time (ms), Voices, Modulation Rate, Modulation Depth)" },
        MotionPreset { id: "ec.audio.gate", name: "Gate (ec.audio.gate)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Gate (Threshold (dB), Attack (ms), Hold (ms), Release (ms))" },
        MotionPreset { id: "ec.audio.highlowpass", name: "High-Low Pass (ec.audio.highlowpass)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · High-Low Pass (Filter Options, Cutoff Frequency, Dry Out, Wet Out)" },
        MotionPreset { id: "ec.audio.modulator", name: "Modulator (ec.audio.modulator)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Modulator (Modulation Type, Modulation Rate, Modulation Depth, Amplitude Modulation)" },
        MotionPreset { id: "ec.audio.parametriceq", name: "Parametric EQ (ec.audio.parametriceq)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Parametric EQ (Band 1 Enabled, Band 1 Frequency, Band 1 Bandwidth, Band 1 Boost/Cut)" },
        MotionPreset { id: "ec.audio.reverb", name: "Reverb (ec.audio.reverb)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Reverb (Reverb Time (milliseconds), Diffusion, Decay, Brightness)" },
        MotionPreset { id: "ec.audio.stereomixer", name: "Stereo Mixer (ec.audio.stereomixer)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Stereo Mixer (Left Level, Right Level, Left Pan, Right Pan)" },
        MotionPreset { id: "ec.audio.tone", name: "Tone (ec.audio.tone)", category: "AE Layer FX · Audio", description: "EffectCraft Audio · Tone (Waveform Options, Frequency 1, Frequency 2, Frequency 3)" },
        MotionPreset { id: "ec.blur.bilateral", name: "Bilateral Blur (ec.blur.bilateral)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Bilateral Blur (Radius, Threshold, Colorize)" },
        MotionPreset { id: "ec.blur.cccross", name: "CC Cross Blur (ec.blur.cccross)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · CC Cross Blur (Radius X, Radius Y, Transfer Mode)" },
        MotionPreset { id: "ec.blur.ccradial", name: "CC Radial Blur (ec.blur.ccradial)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · CC Radial Blur (Type, Amount, Quality, Center)" },
        MotionPreset { id: "ec.blur.ccradialfast", name: "CC Radial Fast Blur (ec.blur.ccradialfast)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · CC Radial Fast Blur (Center, Amount, Zoom)" },
        MotionPreset { id: "ec.blur.ccvector", name: "CC Vector Blur (ec.blur.ccvector)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · CC Vector Blur (Type, Amount, Angle Offset, Map Softness)" },
        MotionPreset { id: "ec.blur.cameralens", name: "Camera Lens Blur (ec.blur.cameralens)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Camera Lens Blur (Blur Radius, Shape, Roundness, Aspect Ratio)" },
        MotionPreset { id: "ec.blur.camerashakedeblur", name: "Camera-Shake Deblur (ec.blur.camerashakedeblur)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Camera-Shake Deblur (Blur Duration, Deblur Method, Strength, Shake Sensitivity)" },
        MotionPreset { id: "ec.blur.channel", name: "Channel Blur (ec.blur.channel)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Channel Blur (Red Blurriness, Green Blurriness, Blue Blurriness, Alpha Blurriness)" },
        MotionPreset { id: "ec.blur.compound", name: "Compound Blur (ec.blur.compound)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Compound Blur (Blur Layer, Maximum Blur, Stretch Map to Fit, Invert Blur)" },
        MotionPreset { id: "ec.blur.directional", name: "Directional Blur (ec.blur.directional)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Directional Blur (Direction, Blur Length)" },
        MotionPreset { id: "ec.blur.fastbox", name: "Fast Box Blur (ec.blur.fastbox)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Fast Box Blur (Blur Radius, Iterations, Blur Dimensions, Repeat Edge Pixels)" },
        MotionPreset { id: "ec.blur.gaussian", name: "Gaussian Blur (ec.blur.gaussian)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Gaussian Blur (Blurriness, Blur Dimensions, Repeat Edge Pixels)" },
        MotionPreset { id: "ec.blur.radial", name: "Radial Blur (ec.blur.radial)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Radial Blur (Amount, Center, Type, Antialiasing (Best Quality))" },
        MotionPreset { id: "ec.blur.sharpen", name: "Sharpen (ec.blur.sharpen)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Sharpen (Sharpen Amount)" },
        MotionPreset { id: "ec.blur.smart", name: "Smart Blur (ec.blur.smart)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Smart Blur (Radius, Threshold, Mode)" },
        MotionPreset { id: "ec.blur.unsharp", name: "Unsharp Mask (ec.blur.unsharp)", category: "AE Layer FX · Blur & Sharpen", description: "EffectCraft Blur & Sharpen · Unsharp Mask (Amount, Radius, Threshold)" },
        MotionPreset { id: "ec.channel.arithmetic", name: "Arithmetic (ec.channel.arithmetic)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Arithmetic (Operator, Red Value, Green Value, Blue Value)" },
        MotionPreset { id: "ec.channel.blend", name: "Blend (ec.channel.blend)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Blend (Blend With Layer, Mode, Blend With Original, If Layer Sizes Differ)" },
        MotionPreset { id: "ec.channel.cccomposite", name: "CC Composite (ec.channel.cccomposite)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · CC Composite (Opacity, Composite Original, RGB Only)" },
        MotionPreset { id: "ec.channel.calculations", name: "Calculations (ec.channel.calculations)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Calculations (Input Channel, Invert Input, Second Layer, Second Layer Channel)" },
        MotionPreset { id: "ec.channel.combiner", name: "Channel Combiner (ec.channel.combiner)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Channel Combiner (Use 2nd Layer, Source Layer, From, To)" },
        MotionPreset { id: "ec.channel.compoundarithmetic", name: "Compound Arithmetic (ec.channel.compoundarithmetic)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Compound Arithmetic (Second Source Layer, Operator, Operate On Channels, Overflow Behavior)" },
        MotionPreset { id: "ec.channel.invert", name: "Invert (ec.channel.invert)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Invert (Channel, Blend With Original)" },
        MotionPreset { id: "ec.channel.minimax", name: "Minimax (ec.channel.minimax)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Minimax (Operation, Radius, Channel, Direction)" },
        MotionPreset { id: "ec.channel.removecolormatting", name: "Remove Color Matting (ec.channel.removecolormatting)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Remove Color Matting (Background Color, Clip HDR Values)" },
        MotionPreset { id: "ec.channel.setchannels", name: "Set Channels (ec.channel.setchannels)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Set Channels (Source Layer 1, Set Red To Source 1's, Source Layer 2, Set Green To Source 2's)" },
        MotionPreset { id: "ec.channel.setmatte", name: "Set Matte (ec.channel.setmatte)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Set Matte (Take Matte From Layer, Use For Matte, Invert Matte, Stretch Matte to Fit)" },
        MotionPreset { id: "ec.channel.shiftchannels", name: "Shift Channels (ec.channel.shiftchannels)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Shift Channels (Take Alpha From, Take Red From, Take Green From, Take Blue From)" },
        MotionPreset { id: "ec.channel.solidcomposite", name: "Solid Composite (ec.channel.solidcomposite)", category: "AE Layer FX · Channel", description: "EffectCraft Channel · Solid Composite (Source Opacity, Color, Opacity, Blending Mode)" },
        MotionPreset { id: "ec.color.autocolor", name: "Auto Color (ec.color.autocolor)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Auto Color (Temporal Smoothing (seconds), Scene Detect, Black Clip, White Clip)" },
        MotionPreset { id: "ec.color.autocontrast", name: "Auto Contrast (ec.color.autocontrast)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Auto Contrast (Temporal Smoothing (seconds), Scene Detect, Black Clip, White Clip)" },
        MotionPreset { id: "ec.color.autolevels", name: "Auto Levels (ec.color.autolevels)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Auto Levels (Temporal Smoothing (seconds), Scene Detect, Black Clip, White Clip)" },
        MotionPreset { id: "ec.color.blackwhite", name: "Black & White (ec.color.blackwhite)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Black & White (Reds, Yellows, Greens, Cyans)" },
        MotionPreset { id: "ec.color.brightnesscontrast", name: "Brightness & Contrast (ec.color.brightnesscontrast)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Brightness & Contrast (Brightness, Contrast, Use Legacy)" },
        MotionPreset { id: "ec.color.broadcast", name: "Broadcast Colors (ec.color.broadcast)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Broadcast Colors (Broadcast Locale, How To Make Color Safe, Maximum Signal Amplitude (IRE))" },
        MotionPreset { id: "ec.color.cccolorneutralizer", name: "CC Color Neutralizer (ec.color.cccolorneutralizer)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · CC Color Neutralizer (Shadows Unbalance, Shadows Balance, Midtones Unbalance, Midtones Balance)" },
        MotionPreset { id: "ec.color.cccoloroffset", name: "CC Color Offset (ec.color.cccoloroffset)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · CC Color Offset (Red Phase, Green Phase, Blue Phase, Overflow)" },
        MotionPreset { id: "ec.color.cckernel", name: "CC Kernel (ec.color.cckernel)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · CC Kernel (Top Left, Top, Top Right, Left)" },
        MotionPreset { id: "ec.color.cctoner", name: "CC Toner (ec.color.cctoner)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · CC Toner (Tones, Highlights, Brights, Midtones)" },
        MotionPreset { id: "ec.color.changecolor", name: "Change Color (ec.color.changecolor)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Change Color (View, Hue Transform, Lightness Transform, Saturation Transform)" },
        MotionPreset { id: "ec.color.changetocolor", name: "Change to Color (ec.color.changetocolor)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Change to Color (From, To, Change, Change By)" },
        MotionPreset { id: "ec.color.channelmixer", name: "Channel Mixer (ec.color.channelmixer)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Channel Mixer (Red-Red, Red-Green, Red-Blue, Red-Const)" },
        MotionPreset { id: "ec.color.colorbalance", name: "Color Balance (ec.color.colorbalance)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Color Balance (Shadow Red Balance, Shadow Green Balance, Shadow Blue Balance, Midtone Red Balance)" },
        MotionPreset { id: "ec.color.colorbalancehls", name: "Color Balance (HLS) (ec.color.colorbalancehls)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Color Balance (HLS) (Hue, Lightness, Saturation)" },
        MotionPreset { id: "ec.color.colorlink", name: "Color Link (ec.color.colorlink)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Color Link (Source Layer, Source Layer Source, Sample, Clip)" },
        MotionPreset { id: "ec.color.colorstabilizer", name: "Color Stabilizer (ec.color.colorstabilizer)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Color Stabilizer (Reference Frame (s), Stabilize, Black Point, Mid Point)" },
        MotionPreset { id: "ec.color.colorama", name: "Colorama (ec.color.colorama)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Colorama (Get Phase From, Add Phase, Add Phase From, Add Mode)" },
        MotionPreset { id: "ec.color.curves", name: "Curves (ec.color.curves)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Curves (Channel, RGB, Red, Green)" },
        MotionPreset { id: "ec.color.equalize", name: "Equalize (ec.color.equalize)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Equalize (Equalize, Amount to Equalize)" },
        MotionPreset { id: "ec.color.exposure", name: "Exposure (ec.color.exposure)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Exposure (Channels, Exposure, Offset, Gamma Correction)" },
        MotionPreset { id: "ec.color.gammapedestalgain", name: "Gamma/Pedestal/Gain (ec.color.gammapedestalgain)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Gamma/Pedestal/Gain (Black Stretch, Red Gamma, Red Pedestal, Red Gain)" },
        MotionPreset { id: "ec.color.huesaturation", name: "Hue/Saturation (ec.color.huesaturation)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Hue/Saturation (Channel Control, Master Hue, Master Saturation, Master Lightness)" },
        MotionPreset { id: "ec.color.leavecolor", name: "Leave Color (ec.color.leavecolor)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Leave Color (Amount to Decolor, Color To Leave, Tolerance, Edge Softness)" },
        MotionPreset { id: "ec.color.levels", name: "Levels (ec.color.levels)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Levels (Channel, Input Black, Input White, Gamma)" },
        MotionPreset { id: "ec.color.levelsic", name: "Levels (Individual Controls) (ec.color.levelsic)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Levels (Individual Controls) (Input Black, Input White, Gamma, Output Black)" },
        MotionPreset { id: "ec.color.lumetri", name: "Lumetri Color (ec.color.lumetri)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Lumetri Color (High Dynamic Range, Input LUT, Input LUT File, Temperature)" },
        MotionPreset { id: "ec.color.ociocdl", name: "OCIO CDL Transform (ec.color.ociocdl)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · OCIO CDL Transform (Slope Red, Slope Green, Slope Blue, Offset Red)" },
        MotionPreset { id: "ec.color.ociocolorspace", name: "OCIO Color Space Transform (ec.color.ociocolorspace)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · OCIO Color Space Transform (Configuration, Config File (.ocio), Source, Source (config name))" },
        MotionPreset { id: "ec.color.ociodisplay", name: "OCIO Display Transform (ec.color.ociodisplay)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · OCIO Display Transform (Configuration, Config File (.ocio), Input Color Space, Source (config name))" },
        MotionPreset { id: "ec.color.ociofile", name: "OCIO File Transform (ec.color.ociofile)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · OCIO File Transform (File, CCC ID, Interpolation, Direction)" },
        MotionPreset { id: "ec.color.ociolook", name: "OCIO Look Transform (ec.color.ociolook)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · OCIO Look Transform (Configuration, Source, Destination, Look)" },
        MotionPreset { id: "ec.color.psarbitrarymap", name: "PS Arbitrary Map (ec.color.psarbitrarymap)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · PS Arbitrary Map (Phase, Apply Phase Map To Alpha, Map)" },
        MotionPreset { id: "ec.color.photofilter", name: "Photo Filter (ec.color.photofilter)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Photo Filter (Filter, Color, Density, Preserve Luminosity)" },
        MotionPreset { id: "ec.color.selectivecolor", name: "Selective Color (ec.color.selectivecolor)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Selective Color (Method, Colors, Reds Cyan, Reds Magenta)" },
        MotionPreset { id: "ec.color.shadowhighlight", name: "Shadow/Highlight (ec.color.shadowhighlight)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Shadow/Highlight (Auto Amounts, Shadow Amount, Highlight Amount, Temporal Smoothing (seconds))" },
        MotionPreset { id: "ec.color.tint", name: "Tint (ec.color.tint)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Tint (Map Black To, Map White To, Amount to Tint)" },
        MotionPreset { id: "ec.color.tritone", name: "Tritone (ec.color.tritone)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Tritone (Highlights, Midtones, Shadows, Blend With Original)" },
        MotionPreset { id: "ec.color.vibrance", name: "Vibrance (ec.color.vibrance)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Vibrance (Vibrance, Saturation)" },
        MotionPreset { id: "ec.color.videolimiter", name: "Video Limiter (ec.color.videolimiter)", category: "AE Layer FX · Color Correction", description: "EffectCraft Color Correction · Video Limiter (Clip Level, Clip Method, Compression before Clipping, Gamut Warning)" },
        MotionPreset { id: "ec.distort.bezierwarp", name: "Bezier Warp (ec.distort.bezierwarp)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Bezier Warp (Top Left Vertex, Top Left Tangent, Top Right Tangent, Right Top Vertex)" },
        MotionPreset { id: "ec.distort.bulge", name: "Bulge (ec.distort.bulge)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Bulge (Horizontal Radius, Vertical Radius, Bulge Center, Bulge Height)" },
        MotionPreset { id: "ec.distort.ccbendit", name: "CC Bend It (ec.distort.ccbendit)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Bend It (Bend, Start, End, Render Prestart)" },
        MotionPreset { id: "ec.distort.ccbender", name: "CC Bender (ec.distort.ccbender)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Bender (Amount, Style, Adjust To Distance, Top)" },
        MotionPreset { id: "ec.distort.ccblobbylize", name: "CC Blobbylize (ec.distort.ccblobbylize)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Blobbylize (Blob Layer, Property, Softness, Cut Away)" },
        MotionPreset { id: "ec.distort.ccflomotion", name: "CC Flo Motion (ec.distort.ccflomotion)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Flo Motion (Knot 1, Amount 1, Knot 2, Amount 2)" },
        MotionPreset { id: "ec.distort.ccgriddler", name: "CC Griddler (ec.distort.ccgriddler)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Griddler (Horizontal Scale, Vertical Scale, Tile Size, Rotation)" },
        MotionPreset { id: "ec.distort.cclens", name: "CC Lens (ec.distort.cclens)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Lens (Center, Size, Convergence)" },
        MotionPreset { id: "ec.distort.ccpageturn", name: "CC Page Turn (ec.distort.ccpageturn)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Page Turn (Fold Position, Fold Direction, Fold Radius, Light Direction)" },
        MotionPreset { id: "ec.distort.ccpowerpin", name: "CC Power Pin (ec.distort.ccpowerpin)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Power Pin (Top Left, Top Right, Bottom Left, Bottom Right)" },
        MotionPreset { id: "ec.distort.ccripplepulse", name: "CC Ripple Pulse (ec.distort.ccripplepulse)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Ripple Pulse (Center, Pulse Level (animate), Time Span (sec), Amplitude)" },
        MotionPreset { id: "ec.distort.ccslant", name: "CC Slant (ec.distort.ccslant)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Slant (Slant, Stretching, Height, Floor)" },
        MotionPreset { id: "ec.distort.ccsmear", name: "CC Smear (ec.distort.ccsmear)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Smear (From, To, Reach, Radius)" },
        MotionPreset { id: "ec.distort.ccsplit", name: "CC Split (ec.distort.ccsplit)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Split (Point A, Point B, Split)" },
        MotionPreset { id: "ec.distort.ccsplit2", name: "CC Split 2 (ec.distort.ccsplit2)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Split 2 (Point A, Point B, Split 1, Split 2)" },
        MotionPreset { id: "ec.distort.cctiler", name: "CC Tiler (ec.distort.cctiler)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · CC Tiler (Scale, Center, Blend w. Original)" },
        MotionPreset { id: "ec.distort.cornerpin", name: "Corner Pin (ec.distort.cornerpin)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Corner Pin (Upper Left, Upper Right, Lower Left, Lower Right)" },
        MotionPreset { id: "ec.distort.upscale", name: "Detail-preserving Upscale (ec.distort.upscale)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Detail-preserving Upscale (Scale, Reduce Noise, Detail, Alpha)" },
        MotionPreset { id: "ec.distort.displacementmap", name: "Displacement Map (ec.distort.displacementmap)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Displacement Map (Displacement Map Layer, Displacement Map Layer Source, Use For Horizontal Displacement, Max Horizontal Displacement)" },
        MotionPreset { id: "ec.distort.liquify", name: "Liquify (ec.distort.liquify)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Liquify (Tool, Brush Size, Brush Pressure, Freeze Area Mask)" },
        MotionPreset { id: "ec.distort.magnify", name: "Magnify (ec.distort.magnify)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Magnify (Shape, Center, Magnification, Link)" },
        MotionPreset { id: "ec.distort.meshwarp", name: "Mesh Warp (ec.distort.meshwarp)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Mesh Warp (Rows, Columns, Quality, Distortion Mesh)" },
        MotionPreset { id: "ec.distort.mirror", name: "Mirror (ec.distort.mirror)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Mirror (Reflection Center, Reflection Angle)" },
        MotionPreset { id: "ec.distort.offset", name: "Offset (ec.distort.offset)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Offset (Shift Center To, Blend With Original)" },
        MotionPreset { id: "ec.distort.opticscompensation", name: "Optics Compensation (ec.distort.opticscompensation)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Optics Compensation (Field Of View (FOV), Reverse Lens Distortion, FOV Orientation, View Center)" },
        MotionPreset { id: "ec.distort.polar", name: "Polar Coordinates (ec.distort.polar)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Polar Coordinates (Interpolation, Type of Conversion)" },
        MotionPreset { id: "ec.distort.puppet", name: "Puppet (ec.distort.puppet)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Puppet (Mesh Rotation Refinement)" },
        MotionPreset { id: "ec.distort.reshape", name: "Reshape (ec.distort.reshape)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Reshape (Source Mask, Destination Mask, Boundary Mask, Percent)" },
        MotionPreset { id: "ec.distort.ripple", name: "Ripple (ec.distort.ripple)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Ripple (Radius, Center of Ripple, Type of Conversion, Wave Speed)" },
        MotionPreset { id: "ec.distort.rollingshutterrepair", name: "Rolling Shutter Repair (ec.distort.rollingshutterrepair)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Rolling Shutter Repair (Rolling Shutter Rate, Scan Direction, Method, Detailed Analysis)" },
        MotionPreset { id: "ec.distort.smear", name: "Smear (ec.distort.smear)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Smear (Source Mask, Boundary Mask, Mask Offset, Mask Rotation)" },
        MotionPreset { id: "ec.distort.spherize", name: "Spherize (ec.distort.spherize)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Spherize (Radius, Center of Sphere)" },
        MotionPreset { id: "ec.distort.transform", name: "Transform (ec.distort.transform)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Transform (Anchor Point, Position, Uniform Scale, Scale Height)" },
        MotionPreset { id: "ec.distort.turbulentdisplace", name: "Turbulent Displace (ec.distort.turbulentdisplace)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Turbulent Displace (Displacement, Amount, Size, Offset (Turbulence))" },
        MotionPreset { id: "ec.distort.twirl", name: "Twirl (ec.distort.twirl)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Twirl (Angle, Twirl Radius, Twirl Center)" },
        MotionPreset { id: "ec.distort.twirllegacy", name: "Twirl (Legacy) (ec.distort.twirllegacy)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Twirl (Legacy) (Angle, Twirl Radius, Twirl Center)" },
        MotionPreset { id: "ec.distort.warp", name: "Warp (ec.distort.warp)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Warp (Warp Style, Warp Axis, Bend, Horizontal Distortion)" },
        MotionPreset { id: "ec.distort.warpstabilizer", name: "Warp Stabilizer (ec.distort.warpstabilizer)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Warp Stabilizer (Result, Smoothness, Method, Preserve Scale)" },
        MotionPreset { id: "ec.distort.wavewarp", name: "Wave Warp (ec.distort.wavewarp)", category: "AE Layer FX · Distort", description: "EffectCraft Distort · Wave Warp (Wave Type, Wave Height, Wave Width, Direction)" },
        MotionPreset { id: "ec.control.point3d", name: "3D Point Control (ec.control.point3d)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · 3D Point Control (3D Point)" },
        MotionPreset { id: "ec.control.angle", name: "Angle Control (ec.control.angle)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Angle Control (Angle)" },
        MotionPreset { id: "ec.control.checkbox", name: "Checkbox Control (ec.control.checkbox)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Checkbox Control (Checkbox)" },
        MotionPreset { id: "ec.control.color", name: "Color Control (ec.control.color)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Color Control (Color)" },
        MotionPreset { id: "ec.control.dropdown", name: "Dropdown Menu Control (ec.control.dropdown)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Dropdown Menu Control (Menu)" },
        MotionPreset { id: "ec.control.layer", name: "Layer Control (ec.control.layer)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Layer Control (Layer)" },
        MotionPreset { id: "ec.control.point", name: "Point Control (ec.control.point)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Point Control (Point)" },
        MotionPreset { id: "ec.control.slider", name: "Slider Control (ec.control.slider)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Slider Control (Slider)" },
        MotionPreset { id: "ec.control.stereo3d", name: "Stereo 3D Controls (ec.control.stereo3d)", category: "AE Layer FX · Expression Controls", description: "EffectCraft Expression Controls · Stereo 3D Controls (Configuration, Stereo Scene Depth, Enable Convergence, Convergence Of)" },
        MotionPreset { id: "ec.generate.fourcolor", name: "4-Color Gradient (ec.generate.fourcolor)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · 4-Color Gradient (Point 1, Color 1, Point 2, Color 2)" },
        MotionPreset { id: "ec.generate.advancedlightning", name: "Advanced Lightning (ec.generate.advancedlightning)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Advanced Lightning (Lightning Type, Origin, Direction, Conductivity State)" },
        MotionPreset { id: "ec.generate.audiospectrum", name: "Audio Spectrum (ec.generate.audiospectrum)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Audio Spectrum (Audio Layer, Start Point, End Point, Path)" },
        MotionPreset { id: "ec.generate.audiowaveform", name: "Audio Waveform (ec.generate.audiowaveform)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Audio Waveform (Audio Layer, Start Point, End Point, Path)" },
        MotionPreset { id: "ec.generate.beam", name: "Beam (ec.generate.beam)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Beam (Starting Point, Ending Point, Length, Time)" },
        MotionPreset { id: "ec.generate.ccgluegun", name: "CC Glue Gun (ec.generate.ccgluegun)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · CC Glue Gun (Brush Position, Stroke Width, Density, Time Span (sec))" },
        MotionPreset { id: "ec.generate.cclightburst", name: "CC Light Burst 2.5 (ec.generate.cclightburst)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · CC Light Burst 2.5 (Center, Intensity, Ray Length, Burst)" },
        MotionPreset { id: "ec.generate.cclightrays", name: "CC Light Rays (ec.generate.cclightrays)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · CC Light Rays (Intensity, Center, Radius, Warp Softness)" },
        MotionPreset { id: "ec.generate.cclightsweep", name: "CC Light Sweep (ec.generate.cclightsweep)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · CC Light Sweep (Center, Direction, Shape, Width)" },
        MotionPreset { id: "ec.generate.ccthreads", name: "CC Threads (ec.generate.ccthreads)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · CC Threads (Width, Height, Overlaps, Direction)" },
        MotionPreset { id: "ec.generate.cellpattern", name: "Cell Pattern (ec.generate.cellpattern)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Cell Pattern (Cell Pattern, Invert, Contrast, Overflow)" },
        MotionPreset { id: "ec.generate.checkerboard", name: "Checkerboard (ec.generate.checkerboard)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Checkerboard (Anchor, Size From, Corner, Width)" },
        MotionPreset { id: "ec.generate.circle", name: "Circle (ec.generate.circle)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Circle (Center, Radius, Edge, Edge Radius)" },
        MotionPreset { id: "ec.generate.ellipse", name: "Ellipse (ec.generate.ellipse)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Ellipse (Center, Width, Height, Thickness)" },
        MotionPreset { id: "ec.generate.eyedropperfill", name: "Eyedropper Fill (ec.generate.eyedropperfill)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Eyedropper Fill (Sample Point, Sample Radius, Average Pixel Colors, Maintain Original Alpha)" },
        MotionPreset { id: "ec.generate.fill", name: "Fill (ec.generate.fill)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Fill (Fill Mask, All Masks, Color, Invert)" },
        MotionPreset { id: "ec.generate.fractal", name: "Fractal (ec.generate.fractal)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Fractal (Set Choice, Equation, X (Real), Y (Imaginary))" },
        MotionPreset { id: "ec.generate.gradientramp", name: "Gradient Ramp (ec.generate.gradientramp)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Gradient Ramp (Start of Ramp, Start Color, End of Ramp, End Color)" },
        MotionPreset { id: "ec.generate.grid", name: "Grid (ec.generate.grid)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Grid (Anchor, Size From, Corner, Width)" },
        MotionPreset { id: "ec.generate.lensflare", name: "Lens Flare (ec.generate.lensflare)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Lens Flare (Flare Center, Flare Brightness, Lens Type, Blend With Original)" },
        MotionPreset { id: "ec.generate.paintbucket", name: "Paint Bucket (ec.generate.paintbucket)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Paint Bucket (Fill Point, Fill Selector, Tolerance, View Threshold)" },
        MotionPreset { id: "ec.generate.radiowaves", name: "Radio Waves (ec.generate.radiowaves)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Radio Waves (Producer Point, Parameters Are Set At, Render Quality, Wave Type)" },
        MotionPreset { id: "ec.generate.scribble", name: "Scribble (ec.generate.scribble)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Scribble (Scribble, Mask, Fill Type, Edge Width)" },
        MotionPreset { id: "ec.generate.stroke", name: "Stroke (ec.generate.stroke)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Stroke (Path, All Masks, Stroke Sequentially, Color)" },
        MotionPreset { id: "ec.generate.vegas", name: "Vegas (ec.generate.vegas)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Vegas (Stroke, Image Contours: Input Layer, Image Contours: Invert Input, Image Contours: If Layer Sizes Differ)" },
        MotionPreset { id: "ec.generate.writeon", name: "Write-on (ec.generate.writeon)", category: "AE Layer FX · Generate", description: "EffectCraft Generate · Write-on (Brush Position, Color, Brush Size, Brush Hardness)" },
        MotionPreset { id: "ec.vr.blur", name: "VR Blur (ec.vr.blur)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Blur (Frame Layout, Blurriness)" },
        MotionPreset { id: "ec.vr.chromaticaberrations", name: "VR Chromatic Aberrations (ec.vr.chromaticaberrations)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Chromatic Aberrations (Frame Layout, Center Tilt, Center Pan, Aberration (Red))" },
        MotionPreset { id: "ec.vr.colorgradients", name: "VR Color Gradients (ec.vr.colorgradients)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Color Gradients (Frame Layout, Enable Point 1, Point 1, Color 1)" },
        MotionPreset { id: "ec.vr.converter", name: "VR Converter (ec.vr.converter)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Converter (Frame Layout, Source Projection, Source Horizontal FOV, Target Projection)" },
        MotionPreset { id: "ec.vr.denoise", name: "VR De-Noise (ec.vr.denoise)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR De-Noise (Frame Layout, Noise Type, Noise Level, Detail)" },
        MotionPreset { id: "ec.vr.digitalglitch", name: "VR Digital Glitch (ec.vr.digitalglitch)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Digital Glitch (Frame Layout, Master Amplitude, Distortion Rate, Distortion Complexity)" },
        MotionPreset { id: "ec.vr.fractalnoise", name: "VR Fractal Noise (ec.vr.fractalnoise)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Fractal Noise (Frame Layout, Fractal Type, Contrast, Brightness)" },
        MotionPreset { id: "ec.vr.glow", name: "VR Glow (ec.vr.glow)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Glow (Frame Layout, Luma Threshold, Glow Radius, Glow Brightness)" },
        MotionPreset { id: "ec.vr.planetosphere", name: "VR Plane to Sphere (ec.vr.planetosphere)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Plane to Sphere (Frame Layout, Scale (Degrees), Feather, Tilt (X Axis))" },
        MotionPreset { id: "ec.vr.rotatesphere", name: "VR Rotate Sphere (ec.vr.rotatesphere)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Rotate Sphere (Frame Layout, Invert Rotation, Tilt (X Axis), Pan (Y Axis))" },
        MotionPreset { id: "ec.vr.sharpen", name: "VR Sharpen (ec.vr.sharpen)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Sharpen (Frame Layout, Sharpen Amount)" },
        MotionPreset { id: "ec.vr.spheretoplane", name: "VR Sphere to Plane (ec.vr.spheretoplane)", category: "AE Layer FX · Immersive Video", description: "EffectCraft Immersive Video · VR Sphere to Plane (Frame Layout, Horizontal FOV, Tilt (X Axis), Pan (Y Axis))" },
        MotionPreset { id: "ec.key.advancedspill", name: "Advanced Spill Suppressor (ec.key.advancedspill)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Advanced Spill Suppressor (Method, Suppression, Key Color, Tolerance)" },
        MotionPreset { id: "ec.key.ccsimplewireremoval", name: "CC Simple Wire Removal (ec.key.ccsimplewireremoval)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · CC Simple Wire Removal (Point A, Point B, Removal Style, Thickness)" },
        MotionPreset { id: "ec.key.colordifference", name: "Color Difference Key (ec.key.colordifference)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Color Difference Key (View, Key Color, Color Matching Accuracy, Partial A In Black)" },
        MotionPreset { id: "ec.key.colorrange", name: "Color Range (ec.key.colorrange)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Color Range (Fuzziness, Color Space, Min (L, Y, R), Max (L, Y, R))" },
        MotionPreset { id: "ec.key.differencematte", name: "Difference Matte (ec.key.differencematte)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Difference Matte (View, Difference Layer, If Layer Sizes Differ, Matching Tolerance)" },
        MotionPreset { id: "ec.key.extract", name: "Extract (ec.key.extract)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Extract (Channel, Black Point, White Point, Black Softness)" },
        MotionPreset { id: "ec.key.innerouter", name: "Inner/Outer Key (ec.key.innerouter)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Inner/Outer Key (Foreground (Inside), Foreground 1, Foreground 2, Foreground 3)" },
        MotionPreset { id: "ec.key.keycleaner", name: "Key Cleaner (ec.key.keycleaner)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Key Cleaner (Additional Edge Radius, Reduce Chatter, Alpha Contrast, Strength)" },
        MotionPreset { id: "ec.keying.keylight", name: "Key Light (ec.keying.keylight)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Key Light (View, Unpremultiply Result, Screen Colour, Screen Gain)" },
        MotionPreset { id: "ec.key.linearcolor", name: "Linear Color Key (ec.key.linearcolor)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Linear Color Key (View, Key Color, Match colors, Matching Tolerance)" },
        MotionPreset { id: "ec.key.screen", name: "Screen Key (ec.key.screen)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Screen Key (View, Screen Colour, Screen Gain, Screen Balance)" },
        MotionPreset { id: "ec.key.unmult", name: "Unmult (ec.key.unmult)", category: "AE Layer FX · Keying", description: "EffectCraft Keying · Unmult (Background Color, Black Level, White Level, Softness)" },
        MotionPreset { id: "ec.matte.mattechoker", name: "Matte Choker (ec.matte.mattechoker)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Matte Choker (Geometric Softness 1, Choke 1, Gray Level Softness 1, Geometric Softness 2)" },
        MotionPreset { id: "ec.obsolete.mochashape", name: "Mocha shape (ec.obsolete.mochashape)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Mocha shape (Blend mode, Invert, Render edge width, Render type)" },
        MotionPreset { id: "ec.matte.objectmatte", name: "Object Matte (ec.matte.objectmatte)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Object Matte (Object Point, Use Box, Box Top Left, Box Bottom Right)" },
        MotionPreset { id: "ec.matte.refinehard", name: "Refine Hard Matte (ec.matte.refinehard)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Refine Hard Matte (Smooth, Feather, Choke, Reduce Chatter)" },
        MotionPreset { id: "ec.matte.refinesoft", name: "Refine Soft Matte (ec.matte.refinesoft)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Refine Soft Matte (Calculate Edge Details, Additional Edge Radius, View Edge Region, Smooth)" },
        MotionPreset { id: "ec.matte.rotobrush", name: "Roto Brush & Refine Edge (ec.matte.rotobrush)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Roto Brush & Refine Edge (Version, Quality, Search Radius, Motion Threshold)" },
        MotionPreset { id: "ec.matte.simplechoker", name: "Simple Choker (ec.matte.simplechoker)", category: "AE Layer FX · Matte", description: "EffectCraft Matte · Simple Choker (View, Choke Matte)" },
        MotionPreset { id: "ec.noise.addgrain", name: "Add Grain (ec.noise.addgrain)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Add Grain (Viewing Mode, Preset, Center, Width)" },
        MotionPreset { id: "ec.noise.curlnoise", name: "Curl Noise (ec.noise.curlnoise)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Curl Noise (View, Displacement Amount, Scale, Rotation)" },
        MotionPreset { id: "ec.noise.dustscratches", name: "Dust & Scratches (ec.noise.dustscratches)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Dust & Scratches (Radius, Threshold, Operate On Alpha Channel)" },
        MotionPreset { id: "ec.noise.fractal", name: "Fractal Noise (ec.noise.fractal)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Fractal Noise (Fractal Type, Noise Type, Invert, Contrast)" },
        MotionPreset { id: "ec.noise.matchgrain", name: "Match Grain (ec.noise.matchgrain)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Match Grain (Viewing Mode, Noise Source Layer, Noise Source Layer Source, Compensate for Existing Noise)" },
        MotionPreset { id: "ec.noise.median", name: "Median (ec.noise.median)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Median (Radius, Operate On Alpha Channel)" },
        MotionPreset { id: "ec.noise.medianlegacy", name: "Median (Legacy) (ec.noise.medianlegacy)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Median (Legacy) (Radius, Operate On Alpha Channel)" },
        MotionPreset { id: "ec.noise.noise", name: "Noise (ec.noise.noise)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Noise (Amount of Noise, Use Color Noise, Clip Result Values)" },
        MotionPreset { id: "ec.noise.noisealpha", name: "Noise Alpha (ec.noise.noisealpha)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Noise Alpha (Noise, Amount, Original Alpha, Overflow)" },
        MotionPreset { id: "ec.noise.noisehls", name: "Noise HLS (ec.noise.noisehls)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Noise HLS (Noise, Hue, Lightness, Saturation)" },
        MotionPreset { id: "ec.noise.noisehlsauto", name: "Noise HLS Auto (ec.noise.noisehlsauto)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Noise HLS Auto (Noise, Hue, Lightness, Saturation)" },
        MotionPreset { id: "ec.noise.removegrain", name: "Remove Grain (ec.noise.removegrain)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Remove Grain (Viewing Mode, Center, Width, Height)" },
        MotionPreset { id: "ec.noise.turbulent", name: "Turbulent Noise (ec.noise.turbulent)", category: "AE Layer FX · Noise & Grain", description: "EffectCraft Noise & Grain · Turbulent Noise (Fractal Type, Noise Type, Invert, Contrast)" },
        MotionPreset { id: "ec.obsolete.basic3d", name: "Basic 3D (ec.obsolete.basic3d)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Basic 3D (Swivel, Tilt, Distance to Image, Specular Highlight)" },
        MotionPreset { id: "ec.obsolete.basictext", name: "Basic Text (ec.obsolete.basictext)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Basic Text (Text, Alignment, Position, Display Options)" },
        MotionPreset { id: "ec.key.colorkey", name: "Color Key (ec.key.colorkey)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Color Key (Key Color, Color Tolerance, Edge Thin, Edge Feather)" },
        MotionPreset { id: "ec.obsolete.gaussianlegacy", name: "Gaussian Blur (Legacy) (ec.obsolete.gaussianlegacy)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Gaussian Blur (Legacy) (Blurriness, Blur Dimensions)" },
        MotionPreset { id: "ec.obsolete.lightning", name: "Lightning (ec.obsolete.lightning)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Lightning (Start Point, End Point, Segments, Amplitude)" },
        MotionPreset { id: "ec.key.luma", name: "Luma Key (ec.key.luma)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Luma Key (Key Type, Threshold, Tolerance, Edge Thin)" },
        MotionPreset { id: "ec.obsolete.pathtext", name: "Path Text (ec.obsolete.pathtext)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Path Text (Text, Shape Type, Tangent 1/Circle Point, Vertex 1/Circle Center)" },
        MotionPreset { id: "ec.blur.reduceflicker", name: "Reduce Interlace Flicker (ec.blur.reduceflicker)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Reduce Interlace Flicker (Softness)" },
        MotionPreset { id: "ec.key.spill", name: "Spill Suppressor (ec.key.spill)", category: "AE Layer FX · Obsolete", description: "EffectCraft Obsolete · Spill Suppressor (Color To Suppress, Color Accuracy, Suppression)" },
        MotionPreset { id: "ec.paint.paint", name: "Paint (ec.paint.paint)", category: "AE Layer FX · Paint", description: "EffectCraft Paint · Paint (Paint on Transparent)" },
        MotionPreset { id: "ec.perspective.cameratracker", name: "3D Camera Tracker (ec.perspective.cameratracker)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · 3D Camera Tracker (Shot Type, Horizontal Angle of View, Show Track Points, Render Track Points)" },
        MotionPreset { id: "ec.perspective.3dglasses", name: "3D Glasses (ec.perspective.3dglasses)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · 3D Glasses (Left View, Right View, Scene Convergence, Vertical Alignment)" },
        MotionPreset { id: "ec.perspective.bevelalpha", name: "Bevel Alpha (ec.perspective.bevelalpha)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · Bevel Alpha (Edge Thickness, Light Angle, Light Color, Light Intensity)" },
        MotionPreset { id: "ec.perspective.beveledges", name: "Bevel Edges (ec.perspective.beveledges)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · Bevel Edges (Edge Thickness, Light Angle, Light Color, Light Intensity)" },
        MotionPreset { id: "ec.perspective.cccylinder", name: "CC Cylinder (ec.perspective.cccylinder)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · CC Cylinder (Radius, Position, Rotation, Rotation X)" },
        MotionPreset { id: "ec.perspective.ccenvironment", name: "CC Environment (ec.perspective.ccenvironment)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · CC Environment (Environment, Mapping, Filter Environment, Height)" },
        MotionPreset { id: "ec.perspective.ccsphere", name: "CC Sphere (ec.perspective.ccsphere)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · CC Sphere (Radius, Offset, Rotation X, Rotation Y)" },
        MotionPreset { id: "ec.perspective.ccspotlight", name: "CC Spotlight (ec.perspective.ccspotlight)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · CC Spotlight (From, To, Height, Cone Angle)" },
        MotionPreset { id: "ec.perspective.dropshadow", name: "Drop Shadow (ec.perspective.dropshadow)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · Drop Shadow (Shadow Color, Opacity, Direction, Distance)" },
        MotionPreset { id: "ec.perspective.radialshadow", name: "Radial Shadow (ec.perspective.radialshadow)", category: "AE Layer FX · Perspective", description: "EffectCraft Perspective · Radial Shadow (Shadow Color, Opacity, Light Source, Projection Distance)" },
        MotionPreset { id: "ec.sim.ccballaction", name: "CC Ball Action (ec.sim.ccballaction)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Ball Action (Scatter, Rotation Axis, Rotation, Twist Property)" },
        MotionPreset { id: "ec.sim.ccbubbles", name: "CC Bubbles (ec.sim.ccbubbles)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Bubbles (Bubble Amount, Bubble Speed, Wobble Amplitude, Wobble Frequency)" },
        MotionPreset { id: "ec.sim.ccdrizzle", name: "CC Drizzle (ec.sim.ccdrizzle)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Drizzle (Drip Rate, Longevity (sec), Rippling, Displacement)" },
        MotionPreset { id: "ec.sim.cchair", name: "CC Hair (ec.sim.cchair)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Hair (Length, Thickness, Weight, Constant Mass)" },
        MotionPreset { id: "ec.sim.ccmrmercury", name: "CC Mr. Mercury (ec.sim.ccmrmercury)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Mr. Mercury (Radius X, Radius Y, Producer, Direction)" },
        MotionPreset { id: "ec.sim.ccparticlesystems2", name: "CC Particle Systems II (ec.sim.ccparticlesystems2)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Particle Systems II (Birth Rate, Longevity (sec), Position, Radius X)" },
        MotionPreset { id: "ec.sim.ccparticleworld", name: "CC Particle World (ec.sim.ccparticleworld)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Particle World (Birth Rate, Longevity (sec), Position X, Position Y)" },
        MotionPreset { id: "ec.sim.ccpixelpolly", name: "CC Pixel Polly (ec.sim.ccpixelpolly)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Pixel Polly (Force, Gravity, Spinning, Force Center)" },
        MotionPreset { id: "ec.sim.ccrainfall", name: "CC Rainfall (ec.sim.ccrainfall)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Rainfall (Drops, Size, Scene Depth, Speed)" },
        MotionPreset { id: "ec.sim.ccscatterize", name: "CC Scatterize (ec.sim.ccscatterize)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Scatterize (Scatter, Right Twist, Left Twist, Transfer Mode)" },
        MotionPreset { id: "ec.sim.ccsnowfall", name: "CC Snowfall (ec.sim.ccsnowfall)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Snowfall (Flakes, Size, Variation % (Size), Scene Depth)" },
        MotionPreset { id: "ec.sim.ccstarburst", name: "CC Star Burst (ec.sim.ccstarburst)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · CC Star Burst (Scatter, Speed, Phase, Grid Spacing)" },
        MotionPreset { id: "ec.sim.carddance", name: "Card Dance (ec.sim.carddance)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · Card Dance (Rows & Columns, Rows, Columns, Back Layer)" },
        MotionPreset { id: "ec.sim.caustics", name: "Caustics (ec.sim.caustics)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · Caustics (Bottom, Scaling, Repeat Mode, If Layer Size Differs)" },
        MotionPreset { id: "ec.sim.foam", name: "Foam (ec.sim.foam)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · Foam (View, Producer Point, Producer X Size, Producer Y Size)" },
        MotionPreset { id: "ec.sim.particleplayground", name: "Particle Playground (ec.sim.particleplayground)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · Particle Playground (Cannon Text, Grid Text, Auto-Orient Rotation, Cannon Enabled)" },
        MotionPreset { id: "ec.sim.shatter", name: "Shatter (ec.sim.shatter)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · Shatter (View, Render, Pattern, Repetitions)" },
        MotionPreset { id: "ec.sim.waveworld", name: "Wave World (ec.sim.waveworld)", category: "AE Layer FX · Simulation", description: "EffectCraft Simulation · Wave World (View, Horizontal Rotation, Vertical Rotation, Vertical Scale)" },
        MotionPreset { id: "ec.stylize.brushstrokes", name: "Brush Strokes (ec.stylize.brushstrokes)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Brush Strokes (Stroke Angle, Brush Size, Stroke Length, Stroke Density)" },
        MotionPreset { id: "ec.stylize.ccblockload", name: "CC Block Load (ec.stylize.ccblockload)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Block Load (Completion, Scanlines, Smoothing, Start Cleared)" },
        MotionPreset { id: "ec.stylize.ccburnfilm", name: "CC Burn Film (ec.stylize.ccburnfilm)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Burn Film (Burn, Center, Random Seed)" },
        MotionPreset { id: "ec.stylize.ccglass", name: "CC Glass (ec.stylize.ccglass)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Glass (Bump Map, Property, Softness, Height)" },
        MotionPreset { id: "ec.stylize.cchextile", name: "CC HexTile (ec.stylize.cchextile)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC HexTile (Render, Radius, Center, Rotate)" },
        MotionPreset { id: "ec.stylize.cckaleida", name: "CC Kaleida (ec.stylize.cckaleida)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Kaleida (Center, Size, Mirroring, Rotation)" },
        MotionPreset { id: "ec.stylize.ccmrsmoothie", name: "CC Mr. Smoothie (ec.stylize.ccmrsmoothie)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Mr. Smoothie (Flow Layer, Property, Smoothness, Sample A)" },
        MotionPreset { id: "ec.stylize.ccplastic", name: "CC Plastic (ec.stylize.ccplastic)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Plastic (Bump Layer, Property, Softness, Height)" },
        MotionPreset { id: "ec.stylize.ccrepetile", name: "CC RepeTile (ec.stylize.ccrepetile)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC RepeTile (Expand Right, Expand Left, Expand Down, Expand Up)" },
        MotionPreset { id: "ec.stylize.ccthreshold", name: "CC Threshold (ec.stylize.ccthreshold)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Threshold (Threshold, Channel, Invert, Blend w. Original)" },
        MotionPreset { id: "ec.stylize.ccthresholdrgb", name: "CC Threshold RGB (ec.stylize.ccthresholdrgb)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Threshold RGB (Red Threshold, Green Threshold, Blue Threshold, Invert Red Channel)" },
        MotionPreset { id: "ec.stylize.ccvignette", name: "CC Vignette (ec.stylize.ccvignette)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · CC Vignette (Amount, Angle of View, Center, Pin Highlights)" },
        MotionPreset { id: "ec.stylize.cartoon", name: "Cartoon (ec.stylize.cartoon)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Cartoon (Render, Detail Radius, Detail Threshold, Shading Steps)" },
        MotionPreset { id: "ec.stylize.coloremboss", name: "Color Emboss (ec.stylize.coloremboss)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Color Emboss (Direction, Relief, Contrast, Blend With Original)" },
        MotionPreset { id: "ec.stylize.emboss", name: "Emboss (ec.stylize.emboss)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Emboss (Direction, Relief, Contrast, Blend With Original)" },
        MotionPreset { id: "ec.stylize.findedges", name: "Find Edges (ec.stylize.findedges)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Find Edges (Invert, Blend With Original)" },
        MotionPreset { id: "ec.stylize.glow", name: "Glow (ec.stylize.glow)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Glow (Glow Based On, Glow Threshold, Glow Radius, Glow Intensity)" },
        MotionPreset { id: "ec.stylize.mosaic", name: "Mosaic (ec.stylize.mosaic)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Mosaic (Horizontal Blocks, Vertical Blocks, Sharp Colors)" },
        MotionPreset { id: "ec.stylize.motiontile", name: "Motion Tile (ec.stylize.motiontile)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Motion Tile (Tile Center, Tile Width, Tile Height, Output Width)" },
        MotionPreset { id: "ec.stylize.posterize", name: "Posterize (ec.stylize.posterize)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Posterize (Level)" },
        MotionPreset { id: "ec.stylize.roughenedges", name: "Roughen Edges (ec.stylize.roughenedges)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Roughen Edges (Edge Type, Edge Color, Border, Edge Sharpness)" },
        MotionPreset { id: "ec.stylize.scatter", name: "Scatter (ec.stylize.scatter)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Scatter (Scatter Amount, Grain, Scatter Randomness)" },
        MotionPreset { id: "ec.stylize.strobe", name: "Strobe Light (ec.stylize.strobe)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Strobe Light (Strobe Color, Blend With Original, Strobe Duration (secs), Strobe Period (secs))" },
        MotionPreset { id: "ec.stylize.texturize", name: "Texturize (ec.stylize.texturize)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Texturize (Texture Layer, Light Direction, Texture Contrast, Texture Placement)" },
        MotionPreset { id: "ec.stylize.threshold", name: "Threshold (ec.stylize.threshold)", category: "AE Layer FX · Stylize", description: "EffectCraft Stylize · Threshold (Level)" },
        MotionPreset { id: "ec.text.numbers", name: "Numbers (ec.text.numbers)", category: "AE Layer FX · Text", description: "EffectCraft Text · Numbers (Type, Random Values, Value/Offset/Random Max, Decimal Places)" },
        MotionPreset { id: "ec.text.timecode", name: "Timecode (ec.text.timecode)", category: "AE Layer FX · Text", description: "EffectCraft Text · Timecode (Display Format, Time Source, Time Units, Drop Frame)" },
        MotionPreset { id: "ec.time.ccforcemotionblur", name: "CC Force Motion Blur (ec.time.ccforcemotionblur)", category: "AE Layer FX · Time", description: "EffectCraft Time · CC Force Motion Blur (Motion Blur Levels, Override Shutter Angle, Shutter Angle, Native Motion Blur)" },
        MotionPreset { id: "ec.time.ccwidetime", name: "CC Wide Time (ec.time.ccwidetime)", category: "AE Layer FX · Time", description: "EffectCraft Time · CC Wide Time (Forward Steps, Backward Steps, Native Motion Blur)" },
        MotionPreset { id: "ec.time.echo", name: "Echo (ec.time.echo)", category: "AE Layer FX · Time", description: "EffectCraft Time · Echo (Echo Time (seconds), Number Of Echoes, Starting Intensity, Decay)" },
        MotionPreset { id: "ec.time.pixelmotionblur", name: "Pixel Motion Blur (ec.time.pixelmotionblur)", category: "AE Layer FX · Time", description: "EffectCraft Time · Pixel Motion Blur (Shutter Control, Shutter Angle, Shutter Samples, Vector Detail)" },
        MotionPreset { id: "ec.time.posterizetime", name: "Posterize Time (ec.time.posterizetime)", category: "AE Layer FX · Time", description: "EffectCraft Time · Posterize Time (Frame Rate)" },
        MotionPreset { id: "ec.time.timedifference", name: "Time Difference (ec.time.timedifference)", category: "AE Layer FX · Time", description: "EffectCraft Time · Time Difference (Target, Time Offset (sec), Contrast, Absolute Difference)" },
        MotionPreset { id: "ec.time.timedisplacement", name: "Time Displacement (ec.time.timedisplacement)", category: "AE Layer FX · Time", description: "EffectCraft Time · Time Displacement (Time Displacement Layer, Max Displacement Time (sec), Time Resolution (fps), Stretch Map To Fit)" },
        MotionPreset { id: "ec.time.timewarp", name: "Timewarp (ec.time.timewarp)", category: "AE Layer FX · Time", description: "EffectCraft Time · Timewarp (Method, Adjust Time By, Speed, Source Frame)" },
        MotionPreset { id: "ec.transition.blockdissolve", name: "Block Dissolve (ec.transition.blockdissolve)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Block Dissolve (Transition Completion, Block Width, Block Height, Feather)" },
        MotionPreset { id: "ec.transition.ccglasswipe", name: "CC Glass Wipe (ec.transition.ccglasswipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Glass Wipe (Completion, Layer to Reveal, Gradient Layer, Softness)" },
        MotionPreset { id: "ec.transition.ccgridwipe", name: "CC Grid Wipe (ec.transition.ccgridwipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Grid Wipe (Completion, Center, Rotation, Border)" },
        MotionPreset { id: "ec.transition.ccimagewipe", name: "CC Image Wipe (ec.transition.ccimagewipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Image Wipe (Completion, Border Softness, Auto Softness, Layer)" },
        MotionPreset { id: "ec.transition.ccjaws", name: "CC Jaws (ec.transition.ccjaws)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Jaws (Completion, Center, Direction, Height)" },
        MotionPreset { id: "ec.transition.cclightwipe", name: "CC Light Wipe (ec.transition.cclightwipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Light Wipe (Completion, Center, Intensity, Shape)" },
        MotionPreset { id: "ec.transition.cclinesweep", name: "CC Line Sweep (ec.transition.cclinesweep)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Line Sweep (Completion, Direction, Thickness, Slant)" },
        MotionPreset { id: "ec.transition.ccradialscalewipe", name: "CC Radial ScaleWipe (ec.transition.ccradialscalewipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Radial ScaleWipe (Completion, Center, Reverse Transition)" },
        MotionPreset { id: "ec.transition.ccscalewipe", name: "CC Scale Wipe (ec.transition.ccscalewipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Scale Wipe (Stretch, Center, Direction)" },
        MotionPreset { id: "ec.transition.cctwister", name: "CC Twister (ec.transition.cctwister)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC Twister (Completion, Backside, Shading, Center)" },
        MotionPreset { id: "ec.transition.ccwarpomatic", name: "CC WarpoMatic (ec.transition.ccwarpomatic)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · CC WarpoMatic (Completion, Layer to Reveal, Reactor Layer, Reactor)" },
        MotionPreset { id: "ec.transition.cardwipe", name: "Card Wipe (ec.transition.cardwipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Card Wipe (Transition Completion, Transition Width, Back Layer, Back Layer Is Self)" },
        MotionPreset { id: "ec.transition.gradientwipe", name: "Gradient Wipe (ec.transition.gradientwipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Gradient Wipe (Transition Completion, Transition Softness, Gradient Layer, Gradient Placement)" },
        MotionPreset { id: "ec.transition.iriswipe", name: "Iris Wipe (ec.transition.iriswipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Iris Wipe (Iris Center, Iris Points, Outer Radius, Use Inner Radius)" },
        MotionPreset { id: "ec.transition.linearwipe", name: "Linear Wipe (ec.transition.linearwipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Linear Wipe (Transition Completion, Wipe Angle, Feather)" },
        MotionPreset { id: "ec.transition.radialwipe", name: "Radial Wipe (ec.transition.radialwipe)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Radial Wipe (Transition Completion, Start Angle, Wipe Center, Wipe)" },
        MotionPreset { id: "ec.transition.venetian", name: "Venetian Blinds (ec.transition.venetian)", category: "AE Layer FX · Transition", description: "EffectCraft Transition · Venetian Blinds (Transition Completion, Direction, Width, Feather)" },
        MotionPreset { id: "ec.utility.applylut", name: "Apply Color LUT (ec.utility.applylut)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · Apply Color LUT (LUT)" },
        MotionPreset { id: "ec.utility.ccoverbrights", name: "CC Overbrights (ec.utility.ccoverbrights)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · CC Overbrights (Channel, Highlight Color)" },
        MotionPreset { id: "ec.utility.cineon", name: "Cineon Converter (ec.utility.cineon)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · Cineon Converter (Conversion Type, 10 Bit Black Point, Internal Black Point, 10 Bit White Point)" },
        MotionPreset { id: "ec.utility.colorprofileconverter", name: "Color Profile Converter (ec.utility.colorprofileconverter)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · Color Profile Converter (Input Profile, Linearize Input Profile, Output Profile, Linearize Output Profile)" },
        MotionPreset { id: "ec.utility.facemeasurements", name: "Face Measurements (ec.utility.facemeasurements)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · Face Measurements (Head Position X, Head Position Y, Head Scale, Head Orientation X)" },
        MotionPreset { id: "ec.utility.facetrackpoints", name: "Face Track Points (ec.utility.facetrackpoints)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · Face Track Points (Left Eyebrow Inner, Left Eyebrow Middle, Left Eyebrow Outer, Right Eyebrow Inner)" },
        MotionPreset { id: "ec.utility.growbounds", name: "Grow Bounds (ec.utility.growbounds)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · Grow Bounds (Pixels)" },
        MotionPreset { id: "ec.utility.hdrcompander", name: "HDR Compander (ec.utility.hdrcompander)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · HDR Compander (Mode, Gain, Gamma)" },
        MotionPreset { id: "ec.utility.hdrcompression", name: "HDR Highlight Compression (ec.utility.hdrcompression)", category: "AE Layer FX · Utility", description: "EffectCraft Utility · HDR Highlight Compression (Amount)" },
    ]
}

pub fn is_builtin_anim_preset(preset: &str) -> bool {
    matches!(preset, "fadeIn" | "fadeOut" | "wipeInFromLeft" | "wipeInFromRight" | "wipeInFromTop" | "wipeInFromBottom" | "diagonalSweepIn" | "wipeOutToRight" | "clockReveal" | "fanOpen" | "clockWipeOut" | "blindsOpen" | "slatsDown" | "blindsClose" | "pixelDissolveIn" | "mosaicFadeIn" | "pixelDissolveOut" | "cardFlipIn" | "cardFlipOut" | "gridDoorsIn" | "lensPopIn" | "lineSweepIn" | "jawsCloseIn" | "softGradientDrift" | "auroraVeil" | "slowFog" | "cloudBank" | "darkSmoke" | "emberHaze" | "magmaFlow" | "iceFlow" | "pastelPlasma" | "cellDrift" | "crystalFacets" | "hexMosaic" | "mirroredSilk" | "waterCaustics" | "blueprintGrid" | "checkerScroll" | "scrollingStripes" | "pulseRings" | "bokehDrift" | "spotlightFloor" | "dawnSky" | "sweepingLight" | "grainyWash")
}

pub fn is_supported_preset(preset: &str) -> bool {
    presets().iter().any(|p| p.id == preset) || layer_effects().iter().any(|p| p.id == preset)
}

#[derive(Debug, Clone, Serialize)]
pub struct MotionOutput {
    pub video: String,
    pub project: String,
    pub cached: bool,
}

fn js_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".into())
}

/// The script that builds the animated comp and saves it to `project`.
pub fn script(preset: &str, image: &str, project: &str, w: u32, h: u32, fps: f64, dur: f64) -> Result<String> {
    if !is_supported_preset(preset) {
        bail!("không có preset chuyển động '{preset}'");
    }
    let t_in = (dur / 3.0).min(0.6);
    let t_out = (dur / 3.0).min(0.5);
    let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
    let u = w.min(h) as f64 / 1080.0;
    let end = dur - 1.0 / fps;
    let body = match preset {
        "fade" => format!("fadeInOut({t_in}, {t_out});"),
        "slide_left" | "slide_right" => {
            let d = if preset == "slide_left" { -(w as f64) * 0.18 } else { w as f64 * 0.18 };
            format!(
                "keys(pos, [[0,[{a},{cy}]],[{t_in},[{cx},{cy}]],[{o},[{cx},{cy}]],[{end},[{a},{cy}]]]); straight(pos); ease(pos); fadeInOut({fi}, {fo}); addShadow({u:.2});",
                a = cx + d,
                o = end - t_out,
                fi = t_in * 0.7,
                fo = t_out * 0.8
            )
        }
        "rise" => format!(
            "keys(pos, [[0,[{cx},{b}]],[{t_in},[{cx},{cy}]]]); straight(pos); ease(pos); fadeInOut({t_in}, {t_out}); addShadow({u:.2});",
            b = cy + h as f64 * 0.06
        ),
        "pop" => format!(
            "keys(scl, [[0,[60,60]],[{a},[108,108]],[{t_in},[100,100]],[{o},[100,100]],[{end},[85,85]]]); ease(scl); fadeInOut({fi}, {t_out}); addShadow({u:.2});",
            a = t_in * 0.65,
            o = end - t_out,
            fi = t_in * 0.5
        ),
        "blur_in" | "blurIn" => format!(
            "var fx = L.property(\"ADBE Effect Parade\").addProperty(\"ADBE Gaussian Blur 2\"); var br = fx.property(\"ADBE Gaussian Blur 2-0001\"); keys(br, [[0,{b}],[{t2},0]]); ease(br); fadeInOut({t_in}, {t_out}); addShadow({u:.2});",
            b = 40.0 * u,
            t2 = t_in * 1.4
        ),
        "wipe" | "typewriter" => format!(
            "var fx = L.property(\"ADBE Effect Parade\").addProperty(\"ADBE Linear Wipe\"); fx.property(\"ADBE Linear Wipe-0002\").setValue(270); fx.property(\"ADBE Linear Wipe-0003\").setValue({feather}); var tc = fx.property(\"ADBE Linear Wipe-0001\"); keys(tc, [[0,100],[{t2},0],[{o},0],[{end},100]]); ease(tc); addShadow({u:.2});",
            feather = if preset == "typewriter" { 8.0 * u } else { 120.0 * u },
            t2 = t_in * 1.35,
            o = end - t_out
        ),
        "drift" => format!("keys(scl, [[0,[100,100]],[{end},[106,106]]]); fadeInOut({t_in}, {t_out}); addShadow({u:.2});"),
        "glow_pop" | "bounceInWords" => format!(
            "keys(scl, [[0,[52,52]],[{a},[112,112]],[{t_in},[100,100]],[{o},[100,100]],[{end},[82,82]]]); ease(scl); fadeInOut({fi}, {t_out}); var gl = addGlow({u:.2}, 52, 32); var gi = gl.property(\"ADBE Glo2-0004\"); keys(gi, [[0,3.2],[{t_in},1.1],[{o},1.1],[{end},2.4]]); ease(gi); addShadow({u:.2});",
            a = t_in * 0.62,
            o = end - t_out,
            fi = t_in * 0.45
        ),
        "light_sweep" | "fadeUpCharacters" => format!(
            "keys(pos, [[0,[{cx},{b}]],[{t_in},[{cx},{cy}]],[{o},[{cx},{cy}]],[{end},[{cx},{b}]]]); straight(pos); ease(pos); fadeInOut({fi}, {t_out}); addSweep({x0:.1}, {x1:.1}, {cy:.1}, {u:.2}, [1.0, 0.90, 0.45, 1.0], {s_end:.2}); addGlow({u:.2}, 58, 22); addShadow({u:.2});",
            b = cy + h as f64 * 0.05,
            o = end - t_out,
            fi = t_in * 0.6,
            x0 = w as f64 * 0.05,
            x1 = w as f64 * 0.95,
            s_end = (dur * 0.78).min(end)
        ),
        "light_wipe" | "trackingIn" => format!(
            "var fx = L.property(\"ADBE Effect Parade\").addProperty(\"ADBE Linear Wipe\"); fx.property(\"ADBE Linear Wipe-0002\").setValue(270); fx.property(\"ADBE Linear Wipe-0003\").setValue({feather}); var tc = fx.property(\"ADBE Linear Wipe-0001\"); keys(tc, [[0,100],[{t2},0],[{o},0],[{end},100]]); ease(tc); addSweep({x0:.1}, {x1:.1}, {cy:.1}, {u:.2}, [0.0, 0.92, 1.0, 1.0], {s_end:.2}); addGlow({u:.2}, 54, 26); addShadow({u:.2});",
            feather = 110.0 * u,
            t2 = t_in * 1.25,
            o = end - t_out,
            x0 = w as f64 * 0.04,
            x1 = w as f64 * 0.96,
            s_end = (dur * 0.72).min(end)
        ),
        "glitch_in" | "scramble" => format!(
            "keys(scl, [[0,[92,92]],[{t_in},[100,100]],[{o},[100,100]],[{end},[94,94]]]); ease(scl); fadeInOut({fi}, {fo}); var fx = L.property(\"ADBE Effect Parade\"); var dg = fx.addProperty(\"ec.vr.digitalglitch\"); var amp = dg.property(\"ec.vr.digitalglitch-0002\"); keys(amp, [[0,130],[{g_in},0],[{g_out},0],[{end},110]]); var ev = dg.property(\"ec.vr.digitalglitch-0005\"); keys(ev, [[0,0],[{end},180]]); var ca = fx.addProperty(\"ec.vr.chromaticaberrations\"); var cr = ca.property(\"ec.vr.chromaticaberrations-0004\"); var cb = ca.property(\"ec.vr.chromaticaberrations-0006\"); keys(cr, [[0,24],[{g_in},0],[{g_out},0],[{end},20]]); keys(cb, [[0,-24],[{g_in},0],[{g_out},0],[{end},-20]]); addGlow({u:.2}, 55, 24); addShadow({u:.2});",
            o = end - t_out,
            fi = t_in * 0.35,
            fo = t_out * 0.45,
            g_in = (t_in * 1.15).min(dur * 0.4),
            g_out = (end - t_out * 0.9).max(dur * 0.6)
        ),
        "zoom_punch" | "dropInLines" => format!(
            "keys(scl, [[0,[145,145]],[{a},[94,94]],[{t_in},[100,100]],[{o},[100,100]],[{end},[125,125]]]); ease(scl); fadeInOut({fi}, {fo}); var gl = addGlow({u:.2}, 45, 38); var gi = gl.property(\"ADBE Glo2-0004\"); keys(gi, [[0,3.8],[{t_in},1.2],[{o},1.2],[{end},3.0]]); ease(gi); addShadow({u:.2});",
            a = t_in * 0.55,
            o = end - t_out,
            fi = t_in * 0.35,
            fo = t_out * 0.45
        ),
        "shake_hit" | "jitter" => format!(
            "keys(pos, [[0,[{cx},{cy}]],[0.06,[{x_l},{y_u}]],[0.12,[{x_r},{y_d}]],[0.18,[{x_l2},{y_d}]],[{t_in},[{cx},{cy}]],[{o},[{cx},{cy}]]]); straight(pos); keys(scl, [[0,[128,128]],[0.12,[96,96]],[{t_in},[100,100]],[{o},[100,100]],[{end},[90,90]]]); ease(scl); fadeInOut({fi}, {fo}); var fx = L.property(\"ADBE Effect Parade\"); var ca = fx.addProperty(\"ec.vr.chromaticaberrations\"); var cr = ca.property(\"ec.vr.chromaticaberrations-0004\"); var cb = ca.property(\"ec.vr.chromaticaberrations-0006\"); keys(cr, [[0,32],[{t_in},0],[{o},0],[{end},24]]); keys(cb, [[0,-32],[{t_in},0],[{o},0],[{end},-24]]); addGlow({u:.2}, 48, 30); addShadow({u:.2});",
            x_l = cx - 22.0 * u,
            x_r = cx + 20.0 * u,
            x_l2 = cx - 10.0 * u,
            y_u = cy - 14.0 * u,
            y_d = cy + 12.0 * u,
            o = end - t_out,
            fi = t_in * 0.3,
            fo = t_out * 0.4
        ),
        "neon_hud" => format!(
            "var fx = L.property(\"ADBE Effect Parade\").addProperty(\"ADBE Linear Wipe\"); fx.property(\"ADBE Linear Wipe-0002\").setValue(270); fx.property(\"ADBE Linear Wipe-0003\").setValue({feather}); var tc = fx.property(\"ADBE Linear Wipe-0001\"); keys(tc, [[0,100],[{t2},0],[{o},0],[{end},100]]); ease(tc); addSweep({x0:.1}, {x1:.1}, {cy:.1}, {u:.2}, [1.0, 0.18, 0.28, 1.0], {s_end:.2}); var gl = addGlow({u:.2}, 46, 34); var gi = gl.property(\"ADBE Glo2-0004\"); keys(gi, [[0,3.4],[{t_in},1.35],[{o},1.35],[{end},2.8]]); ease(gi); addShadow({u:.2});",
            feather = 90.0 * u,
            t2 = t_in * 1.1,
            o = end - t_out,
            x0 = w as f64 * 0.04,
            x1 = w as f64 * 0.96,
            s_end = (dur * 0.70).min(end)
        ),
        "flip_3d" => format!(
            "keys(scl, [[0,[118,18]],[{a},[96,108]],[{t_in},[100,100]],[{o},[100,100]],[{end},[110,15]]]); ease(scl); fadeInOut({fi}, {fo}); addSweep({x0:.1}, {x1:.1}, {cy:.1}, {u:.2}, [1.0, 0.82, 0.25, 1.0], {s_end:.2}); addGlow({u:.2}, 50, 28); addShadow({u:.2});",
            a = t_in * 0.6,
            o = end - t_out,
            fi = t_in * 0.35,
            fo = t_out * 0.4,
            x0 = w as f64 * 0.08,
            x1 = w as f64 * 0.92,
            s_end = (dur * 0.75).min(end)
        ),
        _ if is_builtin_anim_preset(preset) => {
            // Built-in animation preset (`anim.applyBuiltinPreset` is executed right after the script saves)
            format!("addShadow({u:.2});")
        }
        _ if preset.starts_with("ec.") => {
            // Direct EffectCraft Layer Effect (`effect.apply` + optional transition completion animation)
            if preset.starts_with("ec.transition.") {
                format!(
                    "try {{ var fx = L.property(\"ADBE Effect Parade\").addProperty({eff}); var c = fx.property({eff} + \"-0001\"); if (c) {{ keys(c, [[0,100],[{t_in},0],[{o},0],[{end},100]]); ease(c); }} }} catch (e) {{}} addShadow({u:.2});",
                    eff = js_str(preset),
                    o = end - t_out
                )
            } else {
                format!(
                    "fadeInOut({t_in}, {t_out}); try {{ L.property(\"ADBE Effect Parade\").addProperty({eff}); }} catch (e) {{}} addShadow({u:.2});",
                    eff = js_str(preset)
                )
            }
        }
        _ => unreachable!(),
    };
    Ok(format!(
        r#"// Draft Studio → EffectCraft motion preset "{preset}"
var comp = app.project.items.addComp("Motion", {w}, {h}, 1, {dur}, {fps});
var item = app.project.importFile(new ImportOptions(new File({image})));
var L = comp.layers.add(item);
L.startTime = 0;
L.outPoint = {dur};
var T = L.property("ADBE Transform Group");
var pos = T.property("ADBE Position");
var scl = T.property("ADBE Scale");
var opa = T.property("ADBE Opacity");
function keys(p, list) {{ for (var i = 0; i < list.length; i++) p.setValueAtTime(list[i][0], list[i][1]); }}
function ease(p) {{
  for (var k = 1; k <= p.numKeys; k++) {{
    var done = false;
    for (var n = 1; n <= 3 && !done; n++) {{
      try {{
        var e = []; for (var j = 0; j < n; j++) e.push(new KeyframeEase(0, 70));
        p.setTemporalEaseAtKey(k, e, e); done = true;
      }} catch (err) {{}}
    }}
  }}
}}
// A straight spatial path: auto-bezier tangents bulge between two equal position keys.
function straight(p) {{
  for (var k = 1; k <= p.numKeys; k++) {{
    try {{ p.setSpatialAutoBezierAtKey(k, false); }} catch (err) {{}}
    try {{ p.setSpatialTangentsAtKey(k, [0, 0, 0], [0, 0, 0]); }} catch (err) {{
      try {{ p.setSpatialTangentsAtKey(k, [0, 0], [0, 0]); }} catch (err2) {{}}
    }}
  }}
}}
function fadeInOut(a, b) {{ keys(opa, [[0,0],[a,100],[{end} - b,100],[{end},0]]); ease(opa); }}
function addShadow(u) {{
  try {{
    var ds = L.property("ADBE Effect Parade").addProperty("ec.perspective.dropshadow");
    ds.property("ADBE Drop Shadow-0002").setValue(72);
    ds.property("ADBE Drop Shadow-0004").setValue(8 * u);
    ds.property("ADBE Drop Shadow-0005").setValue(22 * u);
  }} catch (err) {{}}
}}
function addGlow(u, thr, rad) {{
  var gl = L.property("ADBE Effect Parade").addProperty("ec.stylize.glow");
  gl.property("ADBE Glo2-0002").setValue(thr);
  gl.property("ADBE Glo2-0003").setValue(rad * u);
  gl.property("ADBE Glo2-0004").setValue(1.25);
  return gl;
}}
function addSweep(x0, x1, cy, u, col, tEnd) {{
  var sw = L.property("ADBE Effect Parade").addProperty("ec.generate.cclightsweep");
  var c = sw.property("ec.generate.cclightsweep-0001");
  keys(c, [[0, [x0, cy]], [tEnd, [x1, cy]]]);
  straight(c); ease(c);
  sw.property("ec.generate.cclightsweep-0002").setValue(-25);
  sw.property("ec.generate.cclightsweep-0004").setValue(85 * u);
  sw.property("ec.generate.cclightsweep-0005").setValue(115);
  sw.property("ec.generate.cclightsweep-0006").setValue(140);
  sw.property("ec.generate.cclightsweep-0008").setValue(col);
  sw.property("ec.generate.cclightsweep-0009").setValue(1);
  return sw;
}}
{body}
app.project.save(new File({project}));
"motion ok";
"#,
        image = js_str(image),
        project = js_str(project),
    ))
}

/// Animate `image` (a canvas-sized PNG) with `preset` into an alpha video in `out_dir`.
pub fn render(image: &Path, preset: &str, format: &str, w: u32, h: u32, fps: f64, dur: f64, out_dir: &Path) -> Result<MotionOutput> {
    let img = clean_path_str(&image.to_string_lossy());
    let (ext, fmt_args): (&str, Vec<&str>) = match format {
        "webm" => ("webm", vec!["--format", "webm", "--webm-codec", "vp9", "--channels", "rgba"]),
        _ => ("mov", vec!["--format", "prores", "--prores", "4444", "--channels", "rgba"]),
    };
    std::fs::create_dir_all(out_dir)?;
    let stamp = std::fs::metadata(image).and_then(|m| m.modified()).map(|t| format!("{t:?}")).unwrap_or_default();
    let safe_preset = preset.replace('.', "_");
    let key = short_hash(&format!("{SCRIPT_REV}|{img}|{stamp}|{preset}|{ext}|{w}x{h}|{fps}|{dur:.3}"));
    let stem = format!("motion-{safe_preset}-{key}");
    let video = out_dir.join(format!("{stem}.{ext}"));
    let project = out_dir.join(format!("{stem}.ecproj"));
    let s = |p: &Path| clean_path_str(&p.to_string_lossy());
    if video.is_file() && project.is_file() {
        return Ok(MotionOutput { video: s(&video), project: s(&project), cached: true });
    }
    let exe = Craft::Effect.require()?;
    let js_path = out_dir.join(format!("{stem}.jsx"));
    std::fs::write(&js_path, script(preset, &img, &s(&project), w, h, fps, dur)?)?;
    let js_a = js_path.to_string_lossy().to_string();
    run(&exe, &["script", &js_a, "--empty"], Some(out_dir), Duration::from_secs(120))?;
    if !project.is_file() {
        bail!("EffectCraft không lưu được {}", project.display());
    }
    let proj_a = project.to_string_lossy().to_string();
    if is_builtin_anim_preset(preset) {
        let p_json = serde_json::json!({"preset": preset, "layers": [1]}).to_string();
        let _ = run(&exe, &["--project", &proj_a, "exec", "anim.applyBuiltinPreset", "--params", &p_json, "--save"], Some(out_dir), Duration::from_secs(60));
    } else if preset.starts_with("ec.") {
        let p_json = serde_json::json!({"effect": preset, "layers": [1]}).to_string();
        let _ = run(&exe, &["--project", &proj_a, "exec", "effect.apply", "--params", &p_json, "--save"], Some(out_dir), Duration::from_secs(60));
    }
    let video_a = video.to_string_lossy().to_string();
    let mut args = vec!["render", "--project", proj_a.as_str(), "--comp", "Motion", "--out", video_a.as_str()];
    args.extend(fmt_args);
    run(&exe, &args, Some(out_dir), Duration::from_secs(600))?;
    if !video.is_file() {
        bail!("EffectCraft không render ra {}", video.display());
    }
    Ok(MotionOutput { video: s(&video), project: s(&project), cached: false })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_scripts() {
        assert_eq!(presets().len(), 70);
        assert_eq!(layer_effects().len(), 307);
        for p in presets() {
            let js = script(p.id, "E:/a b/x.png", "E:/o.ecproj", 1920, 1080, 30.0, 4.0).unwrap();
            assert!(js.contains("app.project.save"), "{}", p.id);
            assert!(js.contains("\"E:/a b/x.png\""));
        }
        for fx in layer_effects().iter().take(25) {
            let js = script(fx.id, "E:/a b/x.png", "E:/o.ecproj", 1920, 1080, 30.0, 4.0).unwrap();
            assert!(js.contains("app.project.save"), "{}", fx.id);
        }
        assert!(script("nope", "a", "b", 10, 10, 30.0, 1.0).is_err());
    }
}
