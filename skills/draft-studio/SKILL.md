---
name: draft-studio
description: Chuyên gia điều khiển Draft Studio (VectorCraft + EffectCraft + FilmCraft → CapCut PC) với ĐẦY ĐỦ 735 hiệu ứng, chuyển cảnh, màu LUT, kỹ xảo video/audio, preset chuyển động và mẫu đồ họa. Kích hoạt NGAY LẬP TỨC khi người dùng hoặc agent cần dựng video kiểu CapCut, thêm hiệu ứng chuyển cảnh (112 Transitions), chỉnh màu điện ảnh (26 LUTs & Lumetri), kỹ xảo hình ảnh (139 Video FX), kỹ xảo âm thanh (61 Audio FX), vẽ đồ họa VectorCraft (20 Templates), render chữ động alpha EffectCraft (70 Motion Presets + 307 AE Layer Effects), hoặc biểu diễn quá trình edit video trực tiếp từng bước trên màn hình (Live IDE Mode).
---

# Draft Studio — CapCut AI Video Editing & Craft Engine Skill (Full 735-Item Catalog)

Skill này cung cấp **toàn bộ 735 công cụ, hiệu ứng, chuyển cảnh, LUT màu, preset chuyển động, kỹ xảo After Effects và mẫu đồ họa** có sẵn trong **Draft Studio (`127.0.0.1:8797`)** để Agent nạp ngữ cảnh tức thì mà không cần đọc mã nguồn Rust hay dò tìm thủ công.

## Tổng quan Kho Vũ Khí 735 Hiệu ứng & Công cụ (`735 Items`)
1. **VectorCraft Graphic Templates (`20 mẫu`)**: Đồ họa Vector/SVG xuất ra PNG trong suốt + file `.vectorcraft` chỉnh sửa được (hỗ trợ 16:9, 9:16, 1:1).
2. **FilmCraft Video Transitions (`112 chuyển cảnh`)**: Đầy đủ 112 hiệu ứng chuyển cảnh thuộc 11 nhóm (`Animation`, `Dissolve`, `Grunge & Distort`, `Immersive Video`, `Lights & Blurs`, `Slide`, `Smart Tools`, `Text`, `Transformers`, `Wipe`, `Classic`).
3. **FilmCraft Look LUTs & Lumetri Presets (`26 bộ màu`)**: 8 Creative Look LUTs (`builtin:look-*`) + 18 Lumetri Color Presets (`lumetri:*`).
4. **FilmCraft Video Effects & Animated Presets (`139 kỹ xảo`)**: 130 kỹ xảo Video Effects + 9 Built-in Animated/Mask Presets (`preset:*`).
5. **FilmCraft Audio Effects & Transitions (`61 hiệu ứng âm thanh`)**: 58 bộ lọc/xử lý âm thanh chuyên nghiệp + 3 chuyển cảnh âm thanh.
6. **EffectCraft Motion & Animation Presets (`70 preset chuyển động`)**: 16 CapCut Pro Motion Presets + 8 Kinetic Text Presets + 23 Built-in Layer Transitions + 23 Animated Backgrounds.
7. **EffectCraft After Effects Layer FX (`307 kỹ xảo layer ec.*`)**: Toàn bộ 307 hiệu ứng tầng sâu chuẩn After Effects thuộc 21 danh mục (xem bảng đầy đủ trong `references/full-catalog.md`). Bất kỳ mã `ec.*` nào cũng có thể truyền trực tiếp vào `motion.preset`!

---

## 1. Quy trình "Live IDE Mode" (Hiển thị từng bước trực tiếp trên màn hình)

Khi người dùng yêu cầu dựng hoặc chỉnh sửa video, **LUÔN thực hiện theo từng bước trực quan** bằng công cụ MCP `capcut_live_edit` (hoặc CLI `capcut_draft_app.exe live <spec.json> --step "<mô tả bước>"`) trước khi gọi `capcut_build_project`:

1. **Bước 1 — Dàn dựng phân cảnh thô (Rough Cut)**:
   - Gọi `capcut_live_edit` với danh sách `scenes` (đã cắt `trim_start_sec`, `duration_sec`, `ken_burns`) và `step_note: "Bước 1/4: Dàn dựng phân cảnh trên Timeline"`.
2. **Bước 2 — Đắp Chuyển cảnh, Màu LUT & Kỹ xảo cảnh (FilmCraft FX)**:
   - Cập nhật `transition`, `transition_sec`, `lut`, `video_effect` cho từng cảnh trong `scenes`.
   - Gọi `capcut_live_edit` với `step_note: "Bước 2/4: Áp hiệu ứng chuyển cảnh, LUT màu & kỹ xảo FilmCraft"`.
3. **Bước 3 — Thiết kế Đồ họa VectorCraft & Chữ động EffectCraft (Overlays & BGM)**:
   - Thêm các lớp `overlays` (`kind: "graphic"` hoặc `kind: "motion"`) và `bgm` / `voiceovers`.
   - Gọi `capcut_live_edit` với `step_note: "Bước 3/4: Vẽ đồ họa VectorCraft & xếp lớp Overlay"`.
4. **Bước 4 — Render Chuyển động Alpha, Xuất MP4 Preview & Đăng ký CapCut PC**:
   - Gọi `capcut_build_project` với `film_preview: true`, `register_to_capcut: true`.

---

## 2. Danh mục 20 Mẫu Đồ họa VectorCraft (`graphic.template`)

Mọi mẫu đồ họa tự co giãn theo khổ khung hình (`1920×1080`, `1080×1920`, `1080×1080`) và nhận `accent` (mã màu hex, mặc định `#E8B04B`), `font` (mặc định `Segoe UI`).

| Mã `template` | Tên mẫu | Các khóa trong `fields` | Thời lượng gợi ý | Vị trí mặc định trên Canvas |
|---|---|---|---|---|
| `lower_third` | Lower third (Tên + Chức danh) | `name`, `role` | `5.0s` | Góc dưới trái (`y ≈ 800..950`) |
| `chapter_title` | Tiêu đề chương lớn | `kicker`, `title`, `subtitle` | `4.0s` | Chính giữa màn hình (`y ≈ 400..680`) |
| `quote_card` | Khung trích dẫn | `quote` (nhiều dòng), `author` | `6.0s` | Chính giữa màn hình |
| `location_tag` | Nhãn Địa điểm · Thời gian | `place`, `date` | `4.0s` | Góc trên trái (`y ≈ 84..148`) |
| `headline_bar` | Thanh tin tức (News Bar) | `tag`, `headline` | `6.0s` | Ngang đáy màn hình (`y ≈ 844..960`) |
| `subtitle` | Phụ đề nổi tiếng Việt | `tag` (nhãn từ khóa, để `""` nếu ẩn), `text` | `4.5s` | Giữa đáy màn hình (`y ≈ 862..990`) |
| `fact_box` | Hộp con số thống kê | `value`, `label` | `5.0s` | Giữa bên phải (`x ≈ 1244..1824`) |
| `subscribe` | Nút đăng ký kênh | `text`, `handle` | `4.0s` | Góc dưới phải |
| `end_card` | Màn kết (Outro kênh) | `channel`, `tagline`, `next_label`, `next_title`, `cta` | `9.0s` | Toàn màn hình |
| `car_spec_hud` | HUD Thông số Siêu xe / Tech | `model`, `hp`, `speed`, `note` | `5.0s` | Góc dưới phải (`x ≈ 1274..1834`) |
| `vs_compare` | So sánh Đối đầu (VS Card) | `left_title`, `left_val`, `right_title`, `right_val` | `5.5s` | Chính giữa màn hình |
| `breaking_banner` | Cảnh báo Nóng (Breaking Alert) | `badge`, `title`, `sub` | `5.0s` | Ngang dưới màn hình |
| `price_tag` | Nhãn Giá & Đánh giá (Price Badge) | `item`, `price`, `rating` | `4.5s` | Góc trên phải (`y ≈ 78..246`) |
| `neon_title` | Tiêu đề Cyberpunk Neon | `top`, `main`, `bottom` | `4.0s` | Chính giữa màn hình |
| `step_callout` | Bước Quy trình (Step Badge) | `step`, `title`, `detail` | `5.0s` | Góc trên trái (`y ≈ 78..226`) |
| `stat_bar` | Thanh Chỉ số Phần trăm (Stat Bar) | `label`, `percent`, `note` | `5.0s` | Góc dưới trái |
| `social_handle` | Tag Mạng xã hội (Social Pill) | `platform`, `handle` | `4.5s` | Góc trên phải (`y ≈ 72..164`) |
| `minimal_corner` | Nhãn Góc Tối giản (Watermark) | `title`, `sub` | `6.0s` | Góc trên trái (`y ≈ 72..134`) |
| `top_list_badge` | Huy hiệu Xếp hạng (Top Rank #1) | `rank`, `title`, `tag` | `4.5s` | Góc trên trái (`y ≈ 84..220`) |
| `cinematic_letterbox` | Khung Điện ảnh 2.39:1 & Phụ đề | `caption`, `credit` | `5.5s` | Hai dải đen trên/dưới màn hình |

> **Quy tắc chống đè chữ (Anti-Collision Rule)**:
> - Tuyệt đối **KHÔNG đặt 2 overlay có cùng vùng toạ độ mặc định xuất hiện cùng một thời điểm** trừ khi dịch chuyển toạ độ `x`, `y` (`-1.0` đến `+1.0`, đơn vị nửa khung hình CapCut).
> - Ví dụ: Nếu `subtitle` đang hiển thị ở đáy màn hình (`y: 0`), thì `lower_third` hoặc `headline_bar` xuất hiện cùng lúc phải đẩy lên cao (`"y": 0.25`) hoặc xếp lệch thời gian `start_sec`.

---

## 3. Danh mục 70 Preset Chuyển động EffectCraft (`motion.preset`)

Đặt `"kind": "motion"` và `"motion": {"preset": "<mã>", "format": "prores"}` (hoặc `"webm"`). Ngoài 70 preset dưới đây, bạn có thể truyền **bất kỳ mã nào trong 307 Layer Effects (`ec.*`)** ở Mục 7 vào `motion.preset`!

| Mã `preset` | Tên chuyển động | Nhóm | Mô tả kỹ xảo |
|---|---|---|---|
| `fade` | Mờ dần (Fade In/Out) | CapCut Pro Motion | Hiện dần, giữ, mờ dần |
| `slide_left` | Trượt từ trái (Slide Left) | CapCut Pro Motion | Trượt vào từ trái có easing, trượt ra cùng hướng |
| `slide_right` | Trượt từ phải (Slide Right) | CapCut Pro Motion | Trượt vào từ phải, trượt ra cùng hướng |
| `rise` | Nổi lên (Rise Up) | CapCut Pro Motion | Nhô lên từ dưới và hiện dần |
| `pop` | Bật ra (Elastic Pop) | CapCut Pro Motion | Phóng to vượt mức rồi về 100% |
| `blur_in` | Lấy nét (Gaussian Focus) | CapCut Pro Motion | Từ mờ nhoè (Gaussian Blur) về nét |
| `wipe` | Quét ngang (Linear Wipe) | CapCut Pro Motion | Linear Wipe mở từ trái sang phải |
| `drift` | Trôi chậm (Slow Drift) | CapCut Pro Motion | Phóng chậm 100→106% suốt thời gian, hiện/mờ ở hai đầu |
| `glow_pop` | Bật sáng Neon (Glow Pop) | CapCut Pro Motion | Phóng nảy đàn hồi + hào quang Glow phát sáng + bóng đổ |
| `light_sweep` | Quét tia sáng (CC Light Sweep) | CapCut Pro Motion | Trượt mượt + luồng sáng CC Light Sweep lướt ngang chữ |
| `light_wipe` | Quét Laser (Cyan Laser Wipe) | CapCut Pro Motion | Mở dần từ trái kèm tia sáng Cyan rực rỡ và hào quang |
| `glitch_in` | Nhiễu số Cyber (RGB Glitch) | CapCut Pro Motion | Tách kênh màu RGB + nhiễu kỹ thuật số lúc vào và ra |
| `zoom_punch` | Đập Nhịp Bass (Zoom Punch) | CapCut Pro Motion | Phóng đập mạnh từ 142% về 100% + Hào quang bùng sáng theo nhịp |
| `shake_hit` | Va Đập Rung Chấn (Shake Hit) | CapCut Pro Motion | Va đập rung giật + tách màu RGB + Glow cường độ cao |
| `neon_hud` | HUD Siêu Xe (Supercar HUD) | CapCut Pro Motion | Mở cổng kỹ thuật số + tia sáng Đỏ/Cyan lướt kép + Neon Glow |
| `flip_3d` | Xoắn Động Học (3D Kinetic Flip) | CapCut Pro Motion | Bật nảy đàn hồi kèm tia sáng vàng quét chéo tốc độ cao |
| `typewriter` | Gõ Chữ Máy Đánh Chữ (Typewriter) | Text & Kinetic Animation | Hiện từng ký tự sắc nét từ trái sang phải kiểu gõ phím |
| `fadeUpCharacters` | Chữ Nổi Dần Lên (Fade Up Characters) | Text & Kinetic Animation | Chữ hiện dần kèm nhô nhẹ từ dưới lên mượt mà |
| `bounceInWords` | Từ Bật Nảy Đàn Hồi (Bounce In Words) | Text & Kinetic Animation | Phóng nảy đàn hồi vượt ngưỡng rồi ổn định từng nhịp |
| `trackingIn` | Thu Hẹp Khoảng Cách Chữ (Tracking In) | Text & Kinetic Animation | Co giãn ngang từ rộng về chuẩn kèm hiện dần điện ảnh |
| `scramble` | Giải Mã Ký Tự Số (Cyber Scramble) | Text & Kinetic Animation | Nhấp nháy giải mã kỹ thuật số kèm tia sáng quét trước khi hội tụ |
| `blurIn` | Hội Tụ Tiêu Cự Chữ (Optical Blur In) | Text & Kinetic Animation | Lấy nét quang học từ mờ sang sắc nét kèm ánh sáng dịu |
| `jitter` | Rung Động Năng Lượng (Kinetic Jitter) | Text & Kinetic Animation | Dao động vị trí và góc nghiêng liên tục đầy năng lượng |
| `dropInLines` | Rơi Từng Dòng Chữ (Drop In Lines) | Text & Kinetic Animation | Rơi từ trên xuống kèm góc nghiêng nhẹ rồi khớp vào vị trí |
| `fadeIn` | Hiện Dần Mượt (Fade In) | Layer Transition (Built-in) | Hiện dần từ trong suốt lên 100% trong 0.5 giây |
| `fadeOut` | Mờ Dần Cuối (Fade Out) | Layer Transition (Built-in) | Mờ dần về trong suốt ở cuối thời lượng |
| `wipeInFromLeft` | Quét Mở Từ Trái (Wipe In From Left) | Layer Transition (Built-in) | Mép mềm quét từ trái sang phải mở dần layer |
| `wipeInFromRight` | Quét Mở Từ Phải (Wipe In From Right) | Layer Transition (Built-in) | Mép mềm quét từ phải sang trái mở dần layer |
| `wipeInFromTop` | Quét Mở Từ Trên (Wipe In From Top) | Layer Transition (Built-in) | Mở dần layer từ mép trên xuống dưới |
| `wipeInFromBottom` | Quét Mở Từ Dưới (Wipe In From Bottom) | Layer Transition (Built-in) | Mở dần layer từ mép dưới lên trên |
| `diagonalSweepIn` | Quét Chéo Mềm (Diagonal Sweep In) | Layer Transition (Built-in) | Mép quét chéo siêu mịn từ góc dưới trái |
| `wipeOutToRight` | Quét Đóng Sang Phải (Wipe Out To Right) | Layer Transition (Built-in) | Mép mềm lướt từ trái sang phải thu gọn layer |
| `clockReveal` | Kim Đồng Hồ Mở (Clock Reveal) | Layer Transition (Built-in) | Quét xoay tròn một vòng quanh tâm để mở layer |
| `fanOpen` | Mở Cánh Quạt Đôi (Fan Open) | Layer Transition (Built-in) | Mở đối xứng hai bên từ đỉnh xuống như cánh quạt |
| `clockWipeOut` | Kim Đồng Hồ Đóng (Clock Wipe Out) | Layer Transition (Built-in) | Quét xoay tròn quanh tâm để thu gọn layer |
| `blindsOpen` | Mở Rèm Dọc (Blinds Open) | Layer Transition (Built-in) | Các nan dọc mở rộng dần cho tới khi hiện trọn layer |
| `slatsDown` | Mở Nan Ngang (Slats Down) | Layer Transition (Built-in) | Các thanh ngang dày dần lên ghép thành layer hoàn chỉnh |
| `blindsClose` | Đóng Rèm Dọc (Blinds Close) | Layer Transition (Built-in) | Các nan dọc khép hẹp dần làm biến mất layer |
| `pixelDissolveIn` | Tan Khối Pixel Vào (Pixel Dissolve In) | Layer Transition (Built-in) | Các ô vuông sắc cạnh xuất hiện ngẫu nhiên ghép thành layer |
| `mosaicFadeIn` | Khảm Mờ Hiện Dần (Mosaic Fade In) | Layer Transition (Built-in) | Các ô gạch lớn viền mềm hiện ngẫu nhiên ghép lại |
| `pixelDissolveOut` | Tan Khối Pixel Ra (Pixel Dissolve Out) | Layer Transition (Built-in) | Các ô vuông rơi rụng ngẫu nhiên cho tới khi hết layer |
| `cardFlipIn` | Lật Thẻ Bài Mở (Card Flip In) | Layer Transition (Built-in) | Lưới thẻ 3D lật từ trái sang phải để lộ diện layer |
| `cardFlipOut` | Lật Thẻ Bài Đóng (Card Flip Out) | Layer Transition (Built-in) | Lưới thẻ 3D lật từ trái sang phải khép layer lại |
| `gridDoorsIn` | Cửa Lưới Mở Tâm (Grid Doors In) | Layer Transition (Built-in) | Các ô cửa nhỏ mở bung từ tâm ra ngoài |
| `lensPopIn` | Thấu Kính Bung Mở (Lens Pop In) | Layer Transition (Built-in) | Bung mở từ vòng tròn khúc xạ quang học ở tâm |
| `lineSweepIn` | Quét Sọc Chéo (Line Sweep In) | Layer Transition (Built-in) | Mép cắt bậc thang nghiêng quét nhanh từ góc dưới trái |
| `jawsCloseIn` | Hàm Răng Khép Mở (Jaws Close In) | Layer Transition (Built-in) | Hai mép răng cưa trên dưới khép lại hiện trọn layer |
| `softGradientDrift` | Soft Gradient Drift (softGradientDrift) | Animated Background (Built-in) | Four soft colour corners slowly drift around the frame. |
| `auroraVeil` | Aurora Veil (auroraVeil) | Animated Background (Built-in) | Tall green curtains of light ripple slowly across a night sky. |
| `slowFog` | Slow Fog (slowFog) | Animated Background (Built-in) | Heavy grey-blue fog banks roll gently through the frame. |
| `cloudBank` | Cloud Bank (cloudBank) | Animated Background (Built-in) | Soft cumulus clouds drift across a blue sky. |
| `darkSmoke` | Dark Smoke (darkSmoke) | Animated Background (Built-in) | Thick dark smoke rises slowly and curls as it goes. |
| `emberHaze` | Ember Haze (emberHaze) | Animated Background (Built-in) | A warm glowing haze of embers, black to deep red to amber. |
| `magmaFlow` | Magma Flow (magmaFlow) | Animated Background (Built-in) | Fire-coloured plasma that slowly cycles through its palette. |
| `iceFlow` | Ice Flow (iceFlow) | Animated Background (Built-in) | Cold blue and white veins drift through a frozen field. |
| `pastelPlasma` | Pastel Plasma (pastelPlasma) | Animated Background (Built-in) | Soft pastel plasma with colours that endlessly cycle. |
| `cellDrift` | Cell Drift (cellDrift) | Animated Background (Built-in) | Bubble-like cells slowly swell and merge in teal tones. |
| `crystalFacets` | Crystal Facets (crystalFacets) | Animated Background (Built-in) | Sharp crystal facets shimmer in warm violet light. |
| `hexMosaic` | Hex Mosaic (hexMosaic) | Animated Background (Built-in) | A hexagon mosaic that glows as fog passes beneath it. |
| `mirroredSilk` | Mirrored Silk (mirroredSilk) | Animated Background (Built-in) | Flowing silk ribbons mirrored down the centre. |
| `waterCaustics` | Water Caustics (waterCaustics) | Animated Background (Built-in) | Bright caustic filaments shimmer across a turquoise pool floor. |
| `blueprintGrid` | Blueprint Grid (blueprintGrid) | Animated Background (Built-in) | A technical grid slides across a deep blue plate. |
| `checkerScroll` | Checker Scroll (checkerScroll) | Animated Background (Built-in) | A two-tone checkerboard scrolls diagonally. |
| `scrollingStripes` | Scrolling Stripes (scrollingStripes) | Animated Background (Built-in) | Wide vertical stripes slide steadily sideways. |
| `pulseRings` | Pulse Rings (pulseRings) | Animated Background (Built-in) | Soft rings expand from the centre in a steady rhythm. |
| `bokehDrift` | Bokeh Drift (bokehDrift) | Animated Background (Built-in) | Out-of-focus lights float lazily over a dark violet backdrop. |
| `spotlightFloor` | Spotlight Floor (spotlightFloor) | Animated Background (Built-in) | A soft pool of light on a dark floor that gently breathes. |
| `dawnSky` | Dawn Sky (dawnSky) | Animated Background (Built-in) | A night sky warms into sunrise over six seconds. |
| `sweepingLight` | Sweeping Light (sweepingLight) | Animated Background (Built-in) | A band of light sweeps repeatedly across a dark slate backdrop. |
| `grainyWash` | Grainy Wash (grainyWash) | Animated Background (Built-in) | A muted diagonal wash with live film grain. |

---

## 4. Danh mục 112 Hiệu ứng Chuyển cảnh FilmCraft (`scene.transition`)

Gán trực tiếp vào từng cảnh trong `scenes` (`"transition": "<mã>", "transition_sec": 0.45`).

| Mã `transition` | Tên hiệu ứng | Nhóm | Mô tả & Tham số |
|---|---|---|---|
| `cross_zoom` | Phóng Xuyên Cảnh (Cross Zoom) | Tốc độ & Lia máy (Speed) | Phóng vút qua điểm cắt kiểu CapCut Car Edit |
| `whip` | Lia Máy Vút (Whip Pan) | Tốc độ & Lia máy (Speed) | Lia ngang tốc độ cao kèm nhoè chuyển động |
| `zoom_blur` | Hút Tâm Tốc Độ (Zoom Blur) | Tốc độ & Lia máy (Speed) | Nhoè hướng tâm lao thẳng vào cảnh kế tiếp |
| `flash` | Chớp Trắng (Camera Flash) | Ánh sáng & Tia chớp (Light) | Chớp sáng mạnh đúng nhịp bass |
| `light_leak` | Lọt Sáng (Light Leak) | Ánh sáng & Tia chớp (Light) | Vệt nắng ấm tràn qua khung hình |
| `light_sweep` | Quét Tia Sáng (Light Sweep) | Ánh sáng & Tia chớp (Light) | Luồng sáng sắc nét quét ngang |
| `flare` | Loá Ống Kính (Lens Flare) | Ánh sáng & Tia chớp (Light) | Quầng sáng quang học lướt qua điểm cắt |
| `burn_chroma` | Cháy Sắc Độ (Burn Chroma) | Ánh sáng & Tia chớp (Light) | Cháy màu rực rỡ ở mép chuyển cảnh |
| `glitch` | Nhiễu Số (Digital Glitch) | Glitch & Rung chấn | Giật hình RGB kỹ thuật số |
| `earthquake` | Động Đất (Earthquake) | Glitch & Rung chấn | Va đập rung lắc mạnh khi sang cảnh mới |
| `vhs_damage` | Băng Từ VHS (VHS Damage) | Glitch & Rung chấn | Nhiễu sọc ngang hoài cổ |
| `liquid_distortion` | Sóng Chất Lỏng (Liquid Distortion) | Glitch & Rung chấn | Biến dạng uốn lượn mượt mà |
| `neon_wipe` | Quét Viền Neon (Neon Wipe) | 3D & Hình học | Đường cắt phát sáng Neon hiện đại |
| `spin_3d` | Xoay Lật 3D (3D Spin) | 3D & Hình học | Xoay lật không gian 3 chiều |
| `pop_motion` | Bật Nảy (Pop Motion) | 3D & Hình học | Bật nảy đàn hồi năng động |
| `spring_motion` | Lò Xo (Spring Motion) | 3D & Hình học | Co giãn nhịp nhàng |
| `luma_fade` | Tan Theo Vùng Sáng (Luma Fade) | Điện ảnh (Cinematic) | Chuyển mượt từ vùng sáng sang vùng tối |
| `film_dissolve` | Hoà Tan Phim Nhựa (Film Dissolve) | Điện ảnh (Cinematic) | Hoà trộn mượt chuẩn màu điện ảnh |
| `blur_dissolve` | Mờ Nhoè Chuyển Cảnh (Blur Dissolve) | Điện ảnh (Cinematic) | Mờ bokeh rồi lấy nét sang cảnh sau |
| `dip_to_black` | Chìm Vào Đen (Dip to Black) | Điện ảnh (Cinematic) | Tối dần rồi mở sang cảnh mới |
| `block_motion` | Block Motion (block_motion) | Chuyển động đàn hồi (Animation) | Chuyển cảnh Block Motion (Animation) · tham số: direction, blocks, motion_blur |
| `flip_motion` | Flip Motion (flip_motion) | Chuyển động đàn hồi (Animation) | Chuyển cảnh Flip Motion (Animation) · tham số: direction, perspective, motion_blur |
| `fold_motion` | Fold Motion (fold_motion) | Chuyển động đàn hồi (Animation) | Chuyển cảnh Fold Motion (Animation) · tham số: direction, perspective |
| `pull_motion` | Pull Motion (pull_motion) | Chuyển động đàn hồi (Animation) | Chuyển cảnh Pull Motion (Animation) · tham số: direction, motion_blur |
| `spin_motion` | Spin Motion (spin_motion) | Chuyển động đàn hồi (Animation) | Chuyển cảnh Spin Motion (Animation) · tham số: rotation, spin, motion_blur |
| `travel_motion` | Travel Motion (travel_motion) | Chuyển động đàn hồi (Animation) | Chuyển cảnh Travel Motion (Animation) · tham số: direction, zoom, motion_blur |
| `additive_dissolve` | Additive Dissolve (additive_dissolve) | Hoà tan (Dissolve) | Chuyển cảnh Additive Dissolve (Dissolve) |
| `burn_alpha` | Burn Alpha (burn_alpha) | Hoà tan (Dissolve) | Chuyển cảnh Burn Alpha (Dissolve) · tham số: burn_color, softness, seed |
| `cross_dissolve` | Cross Dissolve (cross_dissolve) | Hoà tan (Dissolve) | Chuyển cảnh Cross Dissolve (Dissolve) |
| `dip_to_color` | Dip to Color (dip_to_color) | Hoà tan (Dissolve) | Chuyển cảnh Dip to Color (Dissolve) · tham số: color, hold |
| `dip_to_white` | Dip to White (dip_to_white) | Hoà tan (Dissolve) | Chuyển cảnh Dip to White (Dissolve) |
| `morph_cut` | Morph Cut (morph_cut) | Hoà tan (Dissolve) | Chuyển cảnh Morph Cut (Dissolve) |
| `mosaic_transition` | Mosaic (mosaic_transition) | Hoà tan (Dissolve) | Chuyển cảnh Mosaic (Dissolve) · tham số: block_size, dissolve |
| `chaos` | Chaos (chaos) | Glitch & Biến dạng (Grunge & Distort) | Chuyển cảnh Chaos (Grunge & Distort) · tham số: amount, seed |
| `flicker` | Flicker (flicker) | Glitch & Biến dạng (Grunge & Distort) | Chuyển cảnh Flicker (Grunge & Distort) · tham số: flickers, seed |
| `glass` | Glass (glass) | Glitch & Biến dạng (Grunge & Distort) | Chuyển cảnh Glass (Grunge & Distort) · tham số: cell_size, refraction, seed |
| `grunge` | Grunge (grunge) | Glitch & Biến dạng (Grunge & Distort) | Chuyển cảnh Grunge (Grunge & Distort) · tham số: scale, edge_color, edge_width, seed |
| `kaleidoscope` | Kaleidoscope (kaleidoscope) | Glitch & Biến dạng (Grunge & Distort) | Chuyển cảnh Kaleidoscope (Grunge & Distort) · tham số: segments, zoom |
| `tv_power` | TV Power (tv_power) | Glitch & Biến dạng (Grunge & Distort) | Chuyển cảnh TV Power (Grunge & Distort) · tham số: glow_color, line |
| `vr_chroma_leaks` | VR Chroma Leaks (vr_chroma_leaks) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Chroma Leaks (Immersive Video) · tham số: amount, seed |
| `vr_gradient_wipe` | VR Gradient Wipe (vr_gradient_wipe) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Gradient Wipe (Immersive Video) · tham số: softness, invert |
| `vr_iris_wipe` | VR Iris Wipe (vr_iris_wipe) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Iris Wipe (Immersive Video) · tham số: center, feather, border_width, border_color |
| `vr_light_leaks` | VR Light Leaks (vr_light_leaks) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Light Leaks (Immersive Video) · tham số: amount, seed |
| `vr_light_rays` | VR Light Rays (vr_light_rays) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Light Rays (Immersive Video) · tham số: center, amount |
| `vr_mobius_zoom` | VR Mobius Zoom (vr_mobius_zoom) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Mobius Zoom (Immersive Video) · tham số: zoom, twist |
| `vr_random_blocks` | VR Random Blocks (vr_random_blocks) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Random Blocks (Immersive Video) · tham số: block_size, softness, seed |
| `vr_spherical_blur` | VR Spherical Blur (vr_spherical_blur) | Không gian VR 360 (Immersive) | Chuyển cảnh VR Spherical Blur (Immersive Video) · tham số: blur |
| `chroma_leak` | Chroma Leak (chroma_leak) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Chroma Leak (Lights & Blurs) · tham số: amount, seed |
| `directional_blur_transition` | Directional Blur (directional_blur_transition) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Directional Blur (Lights & Blurs) · tham số: angle, blur |
| `glow` | Glow (glow) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Glow (Lights & Blurs) · tham số: radius, intensity |
| `lens_blur` | Lens Blur (lens_blur) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Lens Blur (Lights & Blurs) · tham số: radius |
| `phosphor` | Phosphor (phosphor) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Phosphor (Lights & Blurs) · tham số: color, decay |
| `radial_blur` | Radial Blur (radial_blur) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Radial Blur (Lights & Blurs) · tham số: center, angle |
| `ray` | Ray (ray) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Ray (Lights & Blurs) · tham số: center, amount, length |
| `solarize` | Solarize (solarize) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Solarize (Lights & Blurs) · tham số: amount |
| `stripe` | Stripe (stripe) | Ánh sáng & Nhoè (Lights & Blurs) | Chuyển cảnh Stripe (Lights & Blurs) · tham số: stripes, angle, color |
| `roll_3d` | 3D Roll (roll_3d) | Trượt & Cuộn (Slide) | Chuyển cảnh 3D Roll (Slide) · tham số: direction, perspective |
| `film_roll` | Film Roll (film_roll) | Trượt & Cuộn (Slide) | Chuyển cảnh Film Roll (Slide) · tham số: direction, gap, gap_color, motion_blur |
| `push` | Push (push) | Trượt & Cuộn (Slide) | Chuyển cảnh Push (Slide) · tham số: direction, motion_blur |
| `roll` | Roll (roll) | Trượt & Cuộn (Slide) | Chuyển cảnh Roll (Slide) · tham số: direction, motion_blur |
| `slide` | Slide (slide) | Trượt & Cuộn (Slide) | Chuyển cảnh Slide (Slide) · tham số: direction, motion_blur |
| `split` | Split (split) | Trượt & Cuộn (Slide) | Chuyển cảnh Split (Slide) · tham số: orientation, motion_blur |
| `stretch` | Stretch (stretch) | Trượt & Cuộn (Slide) | Chuyển cảnh Stretch (Slide) · tham số: direction |
| `motion_camera` | Motion Camera (motion_camera) | Thông minh (Smart Tools) | Chuyển cảnh Motion Camera (Smart Tools) · tham số: zoom, rotation, motion_blur |
| `motion_tween` | Motion Tween (motion_tween) | Thông minh (Smart Tools) | Chuyển cảnh Motion Tween (Smart Tools) · tham số: scale |
| `shape_dissolve` | Shape Dissolve (shape_dissolve) | Thông minh (Smart Tools) | Chuyển cảnh Shape Dissolve (Smart Tools) · tham số: shape, size, softness, rotation |
| `shape_flow` | Shape Flow (shape_flow) | Thông minh (Smart Tools) | Chuyển cảnh Shape Flow (Smart Tools) · tham số: shape, direction, softness |
| `text_animator` | Text Animator (text_animator) | Chữ động (Text) | Chuyển cảnh Text Animator (Text) · tham số: columns, rows |
| `typewriter` | Typewriter (typewriter) | Chữ động (Text) | Chuyển cảnh Typewriter (Text) · tham số: columns, rows, cursor |
| `spinback_3d` | 3D Spinback (spinback_3d) | Biến hình 3D (Transformers) | Chuyển cảnh 3D Spinback (Transformers) · tham số: direction, depth |
| `frame` | Frame (frame) | Biến hình 3D (Transformers) | Chuyển cảnh Frame (Transformers) · tham số: frame_width, frame_color, scale |
| `louver` | Louver (louver) | Biến hình 3D (Transformers) | Chuyển cảnh Louver (Transformers) · tham số: slats, orientation |
| `mirror_transition` | Mirror (mirror_transition) | Biến hình 3D (Transformers) | Chuyển cảnh Mirror (Transformers) · tham số: direction |
| `page_peel` | Page Peel (page_peel) | Biến hình 3D (Transformers) | Chuyển cảnh Page Peel (Transformers) · tham số: corner, radius, shadow |
| `slice` | Slice (slice) | Biến hình 3D (Transformers) | Chuyển cảnh Slice (Transformers) · tham số: slices, angle, motion_blur |
| `wave` | Wave (wave) | Biến hình 3D (Transformers) | Chuyển cảnh Wave (Transformers) · tham số: direction, amplitude, wavelength |
| `clock_wipe` | Clock Wipe (clock_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Clock Wipe (Wipe) · tham số: center, start, feather, border_width |
| `linear_wipe` | Linear Wipe (linear_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Linear Wipe (Wipe) · tham số: angle, feather, border_width, border_color |
| `panel_wipe` | Panel Wipe (panel_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Panel Wipe (Wipe) · tham số: direction, panels, feather, border_width |
| `plateau_wipe` | Plateau Wipe (plateau_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Plateau Wipe (Wipe) · tham số: angle, plateaus, feather, border_width |
| `radial_wipe` | Radial Wipe (radial_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Radial Wipe (Wipe) · tham số: corner, feather, border_width, border_color |
| `soft_wipe` | Soft Wipe (soft_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Soft Wipe (Wipe) · tham số: angle, feather |
| `star_wipe` | Star Wipe (star_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Star Wipe (Wipe) · tham số: center, points, inner, rotation |
| `stretch_wipe` | Stretch Wipe (stretch_wipe) | Quét màn hình (Wipe) | Chuyển cảnh Stretch Wipe (Wipe) · tham số: direction, stretch |
| `additive_dissolve_legacy` | Additive Dissolve (Legacy) (additive_dissolve_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Additive Dissolve (Legacy) (Video Transitions) |
| `barn_doors` | Barn Doors (barn_doors) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Barn Doors (Video Transitions) · tham số: orientation, border_width, border_color, antialias |
| `center_split` | Center Split (center_split) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Center Split (Video Transitions) |
| `clock_wipe_legacy` | Clock Wipe (Legacy) (clock_wipe_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Clock Wipe (Legacy) (Video Transitions) |
| `cross_dissolve_legacy` | Cross Dissolve (Legacy) (cross_dissolve_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Cross Dissolve (Legacy) (Video Transitions) |
| `cross_zoom_legacy` | Cross Zoom (Legacy) (cross_zoom_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Cross Zoom (Legacy) (Video Transitions) |
| `dip_to_black_legacy` | Dip to Black (Legacy) (dip_to_black_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Dip to Black (Legacy) (Video Transitions) |
| `dip_to_white_legacy` | Dip to White (Legacy) (dip_to_white_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Dip to White (Legacy) (Video Transitions) |
| `film_dissolve_legacy` | Film Dissolve (Legacy) (film_dissolve_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Film Dissolve (Legacy) (Video Transitions) |
| `inset` | Inset (inset) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Inset (Video Transitions) · tham số: corner, border_width, border_color, antialias |
| `iris_box` | Iris Box (iris_box) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Iris Box (Video Transitions) · tham số: center, border_width, border_color, antialias |
| `iris_cross` | Iris Cross (iris_cross) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Iris Cross (Video Transitions) · tham số: center, border_width, border_color, antialias |
| `iris_diamond` | Iris Diamond (iris_diamond) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Iris Diamond (Video Transitions) · tham số: center, border_width, border_color, antialias |
| `iris_round` | Iris Round (iris_round) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Iris Round (Video Transitions) · tham số: center, border_width, border_color, antialias |
| `non_additive_dissolve` | Non-Additive Dissolve (non_additive_dissolve) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Non-Additive Dissolve (Video Transitions) |
| `push_legacy` | Push (Legacy) (push_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Push (Legacy) (Video Transitions) · tham số: direction |
| `radial_wipe_legacy` | Radial Wipe (Legacy) (radial_wipe_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Radial Wipe (Legacy) (Video Transitions) |
| `slide_legacy` | Slide (Legacy) (slide_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Slide (Legacy) (Video Transitions) · tham số: direction |
| `split_legacy` | Split (Legacy) (split_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Split (Legacy) (Video Transitions) |
| `whip_legacy` | Whip (Legacy) (whip_legacy) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Whip (Legacy) (Video Transitions) · tham số: direction |
| `wipe` | Wipe (wipe) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Wipe (Video Transitions) · tham số: direction |
| `band_slide` | Band Slide (band_slide) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Band Slide (Video Transitions) |
| `checker_wipe` | Checker Wipe (checker_wipe) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Checker Wipe (Video Transitions) |
| `cube_spin` | Cube Spin (cube_spin) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Cube Spin (Video Transitions) |
| `flip_over` | Flip Over (flip_over) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Flip Over (Video Transitions) |
| `gradient_wipe` | Gradient Wipe (gradient_wipe) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Gradient Wipe (Video Transitions) · tham số: softness |
| `page_turn` | Page Turn (page_turn) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Page Turn (Video Transitions) |
| `venetian_blinds` | Venetian Blinds (venetian_blinds) | Chuyển cảnh cổ điển (Classic) | Chuyển cảnh Venetian Blinds (Video Transitions) |

---

## 5. Danh mục 26 Màu Phim LUTs & Lumetri Presets (`scene.lut`)

Gán vào `"lut"` của từng cảnh trong `scenes`. Hỗ trợ cả 8 Look LUT (`builtin:look-*`) lẫn 18 Lumetri Preset (`lumetri:*`).

| Mã `lut` | Tên màu | Nhóm | Phong cách màu |
|---|---|---|---|
| `builtin:look-teal-orange` | Teal & Orange (Bom tấn) | Creative Look LUT | Tương phản xanh lạnh & cam ấm rực rỡ |
| `builtin:look-golden-hour` | Golden Hour (Hoàng hôn vàng) | Creative Look LUT | Ánh vàng rực ấm áp, sang trọng |
| `builtin:look-night` | Cyber Night (Đêm điện ảnh) | Creative Look LUT | Tông xanh đêm sâu và sắc lạnh |
| `builtin:look-cool-blue` | Cool Blue (Xanh kim khí) | Creative Look LUT | Tông lạnh sắc sảo cho siêu xe & công nghệ |
| `builtin:look-warm-film` | Warm Film (Phim nhựa ấm) | Creative Look LUT | Màu phim Kodak cổ điển |
| `builtin:look-bleach-bypass` | Bleach Bypass (Gai góc) | Creative Look LUT | Tương phản gắt kiểu phim hành động tốc độ |
| `builtin:look-faded-matte` | Faded Matte (Mờ sương) | Creative Look LUT | Nâng nhẹ vùng tối kiểu điện ảnh nghệ thuật |
| `builtin:look-monochrome` | Monochrome (Đen trắng) | Creative Look LUT | Đen trắng tương phản cao |
| `lumetri:Teal Shadows, Warm Skin` | Lumetri · Teal Shadows, Warm Skin | Lumetri Cinematic | Cyan-teal vùng tối và giữ ấm tông da với tương phản điện ảnh |
| `lumetri:Night Exterior` | Lumetri · Night Exterior | Lumetri Cinematic | Hiệu ứng Day-for-Night: tối sâu, sắc lạnh và giảm bão hoà |
| `lumetri:Golden Dusk` | Lumetri · Golden Dusk | Lumetri Cinematic | Ánh nắng chiều tà ấm dịu và mềm mại |
| `lumetri:Steel Blue` | Lumetri · Steel Blue | Lumetri Cinematic | Tông thép xanh lạnh, giảm bão hoà và sắc nét |
| `lumetri:Hard Sun` | Lumetri · Hard Sun | Lumetri Cinematic | Tương phản mạnh, giữ chi tiết vùng cháy sáng và bóng đổ sâu |
| `lumetri:Faded Print` | Lumetri · Faded Print | Lumetri Film Emulation | Màu phim in cổ điển: nâng vùng đen, tương phản êm, màu trầm |
| `lumetri:Silver Retention` | Lumetri · Silver Retention | Lumetri Film Emulation | Giữ bạc kiểu Bleach-Bypass: giảm bão hoà kèm tương phản cứng |
| `lumetri:Cross Processed` | Lumetri · Cross Processed | Lumetri Film Emulation | Tráng chéo kênh màu: vùng sáng vàng-lục, vùng tối xanh lam |
| `lumetri:Warm Negative` | Lumetri · Warm Negative | Lumetri Film Emulation | Phim âm bản ấm áp với đường cong chuyển sáng mượt |
| `lumetri:Neutral Mono` | Lumetri · Neutral Mono | Lumetri Monochrome | Đen trắng trung tính chuẩn điện ảnh |
| `lumetri:Hard Mono` | Lumetri · Hard Mono | Lumetri Monochrome | Đen trắng tương phản gắt, vùng tối sâu |
| `lumetri:Sepia Tone` | Lumetri · Sepia Tone | Lumetri Monochrome | Đơn sắc nhuộm tông nâu ấm hoài cổ |
| `lumetri:Cold Mono` | Lumetri · Cold Mono | Lumetri Monochrome | Đơn sắc nhuộm tông xanh băng giá |
| `lumetri:Lift Shadows` | Lumetri · Lift Shadows | Lumetri Technical | Mở sáng chi tiết vùng tối (Shadows) |
| `lumetri:Protect Highlights` | Lumetri · Protect Highlights | Lumetri Technical | Hạ dịu và cứu chi tiết vùng cháy sáng (Highlights) |
| `lumetri:Half Saturation` | Lumetri · Half Saturation | Lumetri Technical | Giảm 50% độ bão hoà màu toàn khung |
| `lumetri:Boost Vibrance` | Lumetri · Boost Vibrance | Lumetri Technical | Tăng độ tươi màu thông minh cho vùng nhạt mà không làm cháy da |
| `lumetri:Legal Range Squeeze` | Lumetri · Legal Range Squeeze | Lumetri Technical | Nén dải sáng chuẩn truyền hình 16–235 bằng đường cong Luma |

---

## 6. Danh mục 139 Kỹ xảo Hình ảnh & Preset Động FilmCraft (`scene.video_effect`)

Gán vào `"video_effect"` của từng cảnh trong `scenes`. Hỗ trợ toàn bộ 130 Video Effects và 9 Preset động/mặt nạ (`preset:*`).

| Mã `video_effect` | Tên kỹ xảo | Nhóm | Mô tả & Tham số |
|---|---|---|---|
| `wonder_glow` | Hào Quang Rực Rỡ (Wonder Glow) | Ánh sáng & Glow (Lights & Glows) | Phát sáng mịn vùng đèn pha & kim loại bóng |
| `glint` | Tia Sao Lấp Lánh (Star Glint) | Ánh sáng & Glow (Lights & Glows) | Tia sao lấp lánh trên thân xe và mâm hợp kim |
| `light_leaks` | Vệt Sáng Trôi (Light Leaks) | Ánh sáng & Glow (Lights & Glows) | Luồng sáng màu điện ảnh trôi trên khung hình |
| `volumetric_rays` | Tia Nắng Khối (God Rays) | Ánh sáng & Glow (Lights & Glows) | Luồng tia sáng xuyên không gian |
| `edge_glow` | Viền Phát Sáng (Edge Glow) | Ánh sáng & Glow (Lights & Glows) | Tôn đường gân khí động học phát sáng |
| `echo_glow` | Dư Ảnh Phát Sáng (Echo Glow) | Ánh sáng & Glow (Lights & Glows) | Hào quang đa tầng ngân dài |
| `lens_flare` | Loá Đèn Pha (Lens Flare) | Ánh sáng & Glow (Lights & Glows) | Điểm lóa quang học rực rỡ |
| `rgb_split` | Tách Màu RGB (Chromatic Split) | Hành động & Glitch | Lệch kênh màu Đỏ-Xanh kiểu CapCut Pro |
| `camera_shake` | Rung Máy Tốc Độ (Camera Shake) | Hành động & Glitch | Rung lắc hành động kèm nhoè chuyển động |
| `vr_digital_glitch` | Nhiễu Sóng Cyber (Digital Glitch) | Hành động & Glitch | Sọc nhiễu kỹ thuật số hiện đại |
| `vignette` | Tối Bốn Góc (Vignette) | Ống kính & Màu | Tập trung ánh nhìn vào chủ thể trung tâm |
| `bokeh_blur` | Xoá Phông Bokeh (Bokeh Blur) | Ống kính & Màu | Làm mờ lung linh kiểu ống kính khẩu lớn |
| `motion` | Motion (motion) | Cơ bản (Core) | Kỹ xảo Motion · tham số: position, scale, scale_width, uniform_scale |
| `opacity` | Opacity (opacity) | Cơ bản (Core) | Kỹ xảo Opacity · tham số: opacity, blend |
| `time_remap` | Time Remapping (time_remap) | Cơ bản (Core) | Kỹ xảo Time Remapping · tham số: speed |
| `brightness_contrast` | Brightness & Contrast (brightness_contrast) | Chỉnh màu (Color Correction) | Kỹ xảo Brightness & Contrast · tham số: brightness, contrast |
| `tint` | Tint (tint) | Chỉnh màu (Color Correction) | Kỹ xảo Tint · tham số: black, white, amount |
| `color_balance` | Color Balance (color_balance) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Color Balance · tham số: shadow_r, shadow_g, shadow_b, mid_r |
| `leave_color` | Leave Color (leave_color) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Leave Color · tham số: amount, color, tolerance, softness |
| `change_to_color` | Change to Color (change_to_color) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Change to Color · tham số: from, to, hue_tol, softness |
| `lumetri` | Lumetri Color (lumetri) | Chỉnh màu (Color Correction) | Kỹ xảo Lumetri Color · tham số: basic_on, input_lut, temperature, tint |
| `gaussian_blur` | Gaussian Blur (gaussian_blur) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Gaussian Blur · tham số: blurriness, dimensions, repeat_edge |
| `directional_blur` | Directional Blur (directional_blur) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Directional Blur · tham số: direction, length |
| `sharpen` | Sharpen (sharpen) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Sharpen · tham số: amount |
| `unsharp_mask` | Unsharp Mask (unsharp_mask) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Unsharp Mask · tham số: amount, radius, threshold |
| `camera_blur` | Camera Blur (camera_blur) | Kỹ xảo video (Video Effects) | Kỹ xảo Camera Blur · tham số: percent |
| `black_white` | Black & White (black_white) | Kiểm soát ảnh (Image Control) | Kỹ xảo Black & White |
| `color_pass` | Color Pass (color_pass) | Kiểm soát ảnh (Image Control) | Kỹ xảo Color Pass · tham số: color, similarity, reverse |
| `gamma_correction` | Gamma Correction (gamma_correction) | Kiểm soát ảnh (Image Control) | Kỹ xảo Gamma Correction · tham số: gamma |
| `invert` | Invert (invert) | Kiểm soát ảnh (Image Control) | Kỹ xảo Invert · tham số: channel, blend |
| `levels` | Levels (levels) | Điều chỉnh ánh sáng (Adjust) | Kỹ xảo Levels · tham số: in_black, in_white, out_black, out_white |
| `proc_amp` | ProcAmp (proc_amp) | Điều chỉnh ánh sáng (Adjust) | Kỹ xảo ProcAmp · tham số: brightness, contrast, hue, saturation |
| `extract` | Extract (extract) | Điều chỉnh ánh sáng (Adjust) | Kỹ xảo Extract · tham số: black, white, softness, invert |
| `posterize` | Posterize (posterize) | Phong cách hoá (Stylize) | Kỹ xảo Posterize · tham số: levels |
| `mosaic` | Mosaic (mosaic) | Phong cách hoá (Stylize) | Kỹ xảo Mosaic · tham số: horizontal, vertical, sharp, softness |
| `find_edges` | Find Edges (find_edges) | Phong cách hoá (Stylize) | Kỹ xảo Find Edges · tham số: invert, blend |
| `emboss` | Emboss (emboss) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Emboss · tham số: direction, relief, contrast, blend |
| `replicate` | Replicate (replicate) | Kỹ xảo video (Video Effects) | Kỹ xảo Replicate · tham số: count |
| `strobe` | Strobe Light (strobe) | Phong cách hoá (Stylize) | Kỹ xảo Strobe Light · tham số: color, blend, duration, period |
| `noise` | Noise (noise) | Noise & Grain | Kỹ xảo Noise · tham số: amount, color, clip, grain_size |
| `median` | Median (median) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Median · tham số: radius |
| `crop` | Crop (crop) | Kỹ xảo video (Video Effects) | Kỹ xảo Crop · tham số: left, top, right, bottom |
| `horizontal_flip` | Horizontal Flip (horizontal_flip) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Horizontal Flip |
| `vertical_flip` | Vertical Flip (vertical_flip) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Vertical Flip |
| `edge_feather` | Edge Feather (edge_feather) | Kỹ xảo video (Video Effects) | Kỹ xảo Edge Feather · tham số: amount |
| `transform` | Transform (transform) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Transform · tham số: anchor, position, uniform_scale, scale_height |
| `mirror` | Mirror (mirror) | Biến dạng quang học (Distort) | Kỹ xảo Mirror · tham số: center, angle |
| `offset` | Offset (offset) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Offset · tham số: shift, blend |
| `lens_distortion` | Lens Distortion (lens_distortion) | Biến dạng quang học (Distort) | Kỹ xảo Lens Distortion · tham số: curvature, v_decentering, h_decentering |
| `twirl` | Twirl (twirl) | Biến dạng quang học (Distort) | Kỹ xảo Twirl · tham số: angle, radius, center |
| `wave_warp` | Wave Warp (wave_warp) | Biến dạng quang học (Distort) | Kỹ xảo Wave Warp · tham số: height, width, direction, speed |
| `basic_3d` | Basic 3D (basic_3d) | Phối cảnh 3D & Bóng đổ (Perspective) | Kỹ xảo Basic 3D · tham số: swivel, tilt, distance |
| `drop_shadow` | Drop Shadow (drop_shadow) | Phối cảnh 3D & Bóng đổ (Perspective) | Kỹ xảo Drop Shadow · tham số: color, opacity, direction, distance |
| `bevel_alpha` | Bevel Alpha (bevel_alpha) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Bevel Alpha · tham số: thickness, angle, color, intensity |
| `ultra_key` | Ultra Key (ultra_key) | Tách nền & Alpha (Keying) | Kỹ xảo Ultra Key · tham số: key_color, output, setting, transparency |
| `track_matte` | Track Matte Key (track_matte) | Tách nền & Alpha (Keying) | Kỹ xảo Track Matte Key · tham số: matte, composite, reverse |
| `color_key` | Color Key (color_key) | Tách nền & Alpha (Keying) | Kỹ xảo Color Key · tham số: color, tolerance, thin, feather |
| `luma_key` | Luma Key (luma_key) | Tách nền & Alpha (Keying) | Kỹ xảo Luma Key · tham số: threshold, cutoff |
| `four_color_gradient` | 4-Color Gradient (four_color_gradient) | Tạo dải màu (Generate) | Kỹ xảo 4-Color Gradient · tham số: c1, c2, c3, c4 |
| `ramp` | Ramp (ramp) | Kỹ xảo video (Video Effects) | Kỹ xảo Ramp · tham số: start, start_color, end, end_color |
| `circle` | Circle (circle) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Circle · tham số: center, radius, color, opacity |
| `grid` | Grid (grid) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Grid · tham số: size, border, color, opacity |
| `timecode` | Timecode (timecode) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Timecode · tham số: position, size, opacity |
| `clip_name` | Clip Name (clip_name) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Clip Name · tham số: position, size |
| `simple_text` | Simple Text (simple_text) | Tiện ích (Utility) | Kỹ xảo Simple Text · tham số: text, position, size, font |
| `lighting_effects` | Lighting Effects (lighting_effects) | Điều chỉnh ánh sáng (Adjust) | Kỹ xảo Lighting Effects · tham số: l1_type, l1_color, l1_center, l1_major |
| `channel_blur` | Channel Blur (channel_blur) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Channel Blur · tham số: red, green, blue, alpha |
| `compound_blur` | Compound Blur (compound_blur) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Compound Blur · tham số: layer, max, stretch, invert |
| `focus_blur` | Focus Blur (focus_blur) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Focus Blur · tham số: shape, center, angle, size |
| `reduce_interlace_flicker` | Reduce Interlace Flicker (reduce_interlace_flicker) | Làm mờ & Sắc nét (Blur & Sharpen) | Kỹ xảo Reduce Interlace Flicker · tham số: softness |
| `asc_cdl` | ASC CDL (asc_cdl) | Chỉnh màu (Color Correction) | Kỹ xảo ASC CDL · tham số: r_slope, r_offset, r_power, g_slope |
| `video_limiter` | Video Limiter (video_limiter) | Chỉnh màu (Color Correction) | Kỹ xảo Video Limiter · tham số: clip_level, compression, axis, gamut_warning |
| `corner_pin` | Corner Pin (corner_pin) | Biến dạng quang học (Distort) | Kỹ xảo Corner Pin · tham số: upper_left, upper_right, lower_left, lower_right |
| `magnify` | Magnify (magnify) | Biến dạng quang học (Distort) | Kỹ xảo Magnify · tham số: shape, center, magnification, size |
| `spherize` | Spherize (spherize) | Biến dạng quang học (Distort) | Kỹ xảo Spherize · tham số: radius, center |
| `turbulent_displace` | Turbulent Displace (turbulent_displace) | Biến dạng quang học (Distort) | Kỹ xảo Turbulent Displace · tham số: displacement, amount, size, offset |
| `warp_stabilizer` | Warp Stabilizer (warp_stabilizer) | Biến dạng quang học (Distort) | Kỹ xảo Warp Stabilizer · tham số: result, smoothness, method, preserve_scale |
| `gradient` | Gradient (gradient) | Tạo dải màu (Generate) | Kỹ xảo Gradient · tham số: start, start_color, end, end_color |
| `channel_mix` | Channel Mix (channel_mix) | Kiểm soát ảnh (Image Control) | Kỹ xảo Channel Mix · tham số: rr, rg, rb, rc |
| `color_replace` | Color Replace (color_replace) | Kiểm soát ảnh (Image Control) | Kỹ xảo Color Replace · tham số: similarity, solid, target, replace |
| `rounded_crop` | Rounded Crop (rounded_crop) | Kiểm soát ảnh (Image Control) | Kỹ xảo Rounded Crop · tham số: left, top, right, bottom |
| `alpha_adjust` | Alpha Adjust (alpha_adjust) | Tách nền & Alpha (Keying) | Kỹ xảo Alpha Adjust · tham số: opacity, ignore, invert, mask_only |
| `logo_cutout` | Logo Cutout (logo_cutout) | Tách nền & Alpha (Keying) | Kỹ xảo Logo Cutout · tham số: background, color, threshold, softness |
| `long_shadow` | Long Shadow (long_shadow) | Phối cảnh 3D & Bóng đổ (Perspective) | Kỹ xảo Long Shadow · tham số: angle, length, color, opacity |
| `brush_strokes` | Brush Strokes (brush_strokes) | Phong cách hoá (Stylize) | Kỹ xảo Brush Strokes · tham số: angle, size, length, density |
| `color_emboss` | Color Emboss (color_emboss) | Phong cách hoá (Stylize) | Kỹ xảo Color Emboss · tham số: direction, relief, contrast, blend |
| `roughen_edges` | Roughen Edges (roughen_edges) | Phong cách hoá (Stylize) | Kỹ xảo Roughen Edges · tham số: edge_type, edge_color, border, sharpness |
| `posterize_time` | Posterize Time (posterize_time) | Thời gian (Time) | Kỹ xảo Posterize Time · tham số: rate |
| `rotate_3d` | 3D Rotate (rotate_3d) | Biến đổi & Chuyển động (Transform) | Kỹ xảo 3D Rotate · tham số: rot_x, rot_y, rot_z, perspective |
| `auto_reframe` | Auto Reframe (auto_reframe) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Auto Reframe · tham số: preset, offset, zoom, aspect |
| `grow` | Grow (grow) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Grow · tham số: from, to, center, easing |
| `shrink` | Shrink (shrink) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Shrink · tham số: from, to, center, easing |
| `move` | Move (move) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Move · tham số: from, to, easing, motion_blur |
| `spin` | Spin (spin) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Spin · tham số: amount, center, easing, scale |
| `spacer` | Spacer (spacer) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Spacer · tham số: left, top, right, bottom |
| `wiggle` | Wiggle (wiggle) | Biến đổi & Chuyển động (Transform) | Kỹ xảo Wiggle · tham số: frequency, amount, rotation, scale |
| `auto_align` | Auto Align (auto_align) | Tiện ích (Utility) | Kỹ xảo Auto Align · tham số: horizontal, vertical, margin_x, margin_y |
| `cineon_converter` | Cineon Converter (cineon_converter) | Tiện ích (Utility) | Kỹ xảo Cineon Converter · tham số: conversion, black10, black_internal, white10 |
| `clone` | Clone (clone) | Tiện ích (Utility) | Kỹ xảo Clone · tham số: columns, rows, gap, mirror |
| `stroke` | Stroke (stroke) | Tiện ích (Utility) | Kỹ xảo Stroke · tham số: color, width, position, opacity |
| `metadata_burnin` | Metadata & Timecode Burn-in (metadata_burnin) | Hiển thị (Video) | Kỹ xảo Metadata & Timecode Burn-in · tham số: source, position, alignment, size |
| `echo` | Echo (echo) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Echo · tham số: time, count, start, decay |
| `lightning` | Lightning (lightning) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Lightning · tham số: start, end, segments, amplitude |
| `cell_pattern` | Cell Pattern (cell_pattern) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Cell Pattern · tham số: pattern, invert, contrast, overflow |
| `checkerboard` | Checkerboard (checkerboard) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Checkerboard · tham số: anchor, width, height, square |
| `ellipse` | Ellipse (ellipse) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Ellipse · tham số: center, width, height, thickness |
| `paint_bucket` | Paint Bucket (paint_bucket) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Paint Bucket · tham số: point, selector, tolerance, invert |
| `write_on` | Write-on (write_on) | Kỹ xảo cổ điển (Classic) | Kỹ xảo Write-on · tham số: brush, color, size, hardness |
| `alpha_glow` | Alpha Glow (alpha_glow) | Kỹ xảo video (Video Effects) | Kỹ xảo Alpha Glow · tham số: glow, brightness, start_color, end_color |
| `block_dissolve` | Block Dissolve (block_dissolve) | Kỹ xảo video (Video Effects) | Kỹ xảo Block Dissolve · tham số: completion, block_w, block_h, feather |
| `directional_blur_legacy` | Directional Blur (Legacy) (directional_blur_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Directional Blur (Legacy) · tham số: direction, length |
| `gaussian_blur_legacy` | Gaussian Blur (Legacy) (gaussian_blur_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Gaussian Blur (Legacy) · tham số: blurriness, dimensions |
| `gradient_wipe_legacy` | Gradient Wipe (Legacy) (gradient_wipe_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Gradient Wipe (Legacy) · tham số: completion, softness, layer, placement |
| `linear_wipe_legacy` | Linear Wipe (Legacy) (linear_wipe_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Linear Wipe (Legacy) · tham số: completion, angle, feather |
| `magnify_legacy` | Magnify (Legacy) (magnify_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Magnify (Legacy) · tham số: shape, center, magnification, size |
| `mosaic_legacy` | Mosaic (Legacy) (mosaic_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Mosaic (Legacy) · tham số: horizontal, vertical, sharp |
| `noise_legacy` | Noise (Legacy) (noise_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Noise (Legacy) · tham số: amount, color, clip |
| `twirl_legacy` | Twirl (Legacy) (twirl_legacy) | Kỹ xảo video (Video Effects) | Kỹ xảo Twirl (Legacy) · tham số: angle, radius, center |
| `vr_blur` | VR Blur (vr_blur) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Blur · tham số: frame_layout, blurriness |
| `vr_chromatic_aberrations` | VR Chromatic Aberrations (vr_chromatic_aberrations) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Chromatic Aberrations · tham số: frame_layout, red, green, blue |
| `vr_color_gradients` | VR Color Gradients (vr_color_gradients) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Color Gradients · tham số: frame_layout, c1, p1_lon, p1_lat |
| `vr_denoise` | VR De-Noise (vr_denoise) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR De-Noise · tham số: frame_layout, amount, noise_type, show_noise |
| `vr_fractal_noise` | VR Fractal Noise (vr_fractal_noise) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Fractal Noise · tham số: frame_layout, fractal_type, invert, contrast |
| `vr_glow` | VR Glow (vr_glow) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Glow · tham số: frame_layout, threshold, radius, brightness |
| `vr_plane_to_sphere` | VR Plane to Sphere (vr_plane_to_sphere) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Plane to Sphere · tham số: frame_layout, scale, pan, tilt |
| `vr_projection` | VR Projection (vr_projection) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Projection · tham số: frame_layout, fov, tilt, pan |
| `vr_rotate_sphere` | VR Rotate Sphere (vr_rotate_sphere) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Rotate Sphere · tham số: frame_layout, tilt, pan, roll |
| `vr_sharpen` | VR Sharpen (vr_sharpen) | Không gian Cyber & VR (Immersive) | Kỹ xảo VR Sharpen · tham số: frame_layout, amount |
| `graphic_text` | Text (graphic_text) | Cơ bản (Core) | Kỹ xảo Text · tham số: name, text, font, font_style |
| `graphic_shape` | Shape (graphic_shape) | Cơ bản (Core) | Kỹ xảo Shape · tham số: name, shape, size, sides |
| `preset:Blur In` | Preset · Lấy Nét Đầu Cảnh (Blur In) | Preset Động & Mặt Nạ (Animated Presets) | Gaussian Blur từ 60 về 0 trong 1 giây đầu |
| `preset:Blur Out` | Preset · Mờ Dần Cuối Cảnh (Blur Out) | Preset Động & Mặt Nạ (Animated Presets) | Gaussian Blur từ 0 lên 60 trong 1 giây cuối |
| `preset:Fade In` | Preset · Hiện Dần Đầu Cảnh (Fade In) | Preset Động & Mặt Nạ (Animated Presets) | Độ mờ Opacity 0% → 100% trong 1 giây đầu |
| `preset:Fade Out` | Preset · Tối Dần Cuối Cảnh (Fade Out) | Preset Động & Mặt Nạ (Animated Presets) | Độ mờ Opacity 100% → 0% trong 1 giây cuối |
| `preset:Mosaic In` | Preset · Giải Mã Ô Vuông (Mosaic In) | Preset Động & Mặt Nạ (Animated Presets) | Khối pixel lớn hội tụ về ảnh nét trong 1 giây đầu |
| `preset:Soft Vignette` | Preset · Mặt Nạ Tối Viền (Soft Vignette) | Preset Động & Mặt Nạ (Animated Presets) | Làm tối bên ngoài mặt nạ bầu dục viền mềm |
| `preset:Spotlight` | Preset · Đèn Rọi Tâm (Spotlight) | Preset Động & Mặt Nạ (Animated Presets) | Làm sáng vùng trung tâm bằng mặt nạ tròn viền mềm |
| `preset:Warm Tint` | Preset · Nhuộm Ấm Hoài Cổ (Warm Tint) | Preset Động & Mặt Nạ (Animated Presets) | Nhuộm vùng sáng ấm và vùng tối nâu cổ điển |
| `preset:PiP 35% Upper Right` | Preset · Hình Trong Hình 35% (PiP Upper Right) | Preset Động & Mặt Nạ (Animated Presets) | Thu nhỏ 35% đặt ở góc trên bên phải khung hình |

---

## 7. Danh mục 61 Hiệu ứng Âm thanh FilmCraft (`audio_effects`) & 307 Kỹ xảo Layer After Effects (`ec.*`)

- Toàn bộ **61 hiệu ứng âm thanh FilmCraft** và **307 kỹ xảo Layer After Effects (`ec.*`)** của EffectCraft được liệt kê chi tiết từng mã kèm tham số tại file tham chiếu:
  👉 Xem chi tiết: [`references/full-catalog.md`](references/full-catalog.md)
- **Mẹo dùng nhanh 307 Layer FX (`ec.*`) trong `overlays`**:
  Bạn có thể gán bất kỳ mã `ec.*` nào làm `motion.preset` của overlay! Ví dụ:
  - Chuyển cảnh kính vỡ 3D: `"motion": {"preset": "ec.transition.ccglasswipe", "format": "prores"}`
  - Chuyển cảnh lật thẻ 3D: `"motion": {"preset": "ec.transition.cardwipe", "format": "prores"}`
  - Xoáy lốc Twister: `"motion": {"preset": "ec.transition.cctwister", "format": "prores"}`
  - Tia sáng quét Light Wipe: `"motion": {"preset": "ec.transition.cclightwipe", "format": "prores"}`
  - Kính vạn hoa Kaleida: `"motion": {"preset": "ec.stylize.cckaleida", "format": "prores"}`
