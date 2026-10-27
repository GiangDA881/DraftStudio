# Draft Studio

Dựng sẵn project CapCut PC từ clip, ảnh, giọng đọc và nhạc của bạn.

[English](README.md) · Tiếng Việt

> Công cụ không chính thức, không liên kết với CapCut hay ByteDance. Định dạng draft của CapCut được tìm hiểu từ file CapCut tự ghi (không có tài liệu công khai), nên một bản cập nhật CapCut có thể làm draft hỏng. Giấy phép MIT — xem [LICENSE](LICENSE) và [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Dựng project **CapCut PC** từ ảnh, video, giọng đọc và nhạc, với đồ họa và chuyển động do ba engine craft làm thật:

| Engine | Việc trong studio | Gọi bằng |
|---|---|---|
| **VectorCraft** | Vẽ title, lower-third, quote card, nhãn địa điểm, thanh tin, con số, nút đăng ký → PNG trong suốt, kèm file `.vectorcraft` để sửa tiếp | `vectorcraft-cli convert x.svg x.png` |
| **EffectCraft** | Làm chuyển động cho đồ họa (trượt, nổi lên, bật, lấy nét, quét, trôi) → video alpha ProRes 4444 `.mov` hoặc WebM, kèm `.ecproj` | `effectcraft-cli script` + `render` |
| **FilmCraft** | Đo thời lượng/kích thước media bằng decoder thuần Rust; dựng lại cả timeline thành `.fcproj` và xuất MP4 xem trước | `filmcraft-cli probe` / `run` / `export` |

Không engine nào bị giả lập: thiếu engine thì lệnh báo lỗi và chỉ chỗ đặt file. Không có engine vẫn dựng được draft (ảnh, video, giọng, nhạc, Ken Burns); chỉ overlay đồ họa / chuyển động và bản xem trước MP4 là cần engine.

## Ba file chạy

| File | Dùng để |
|---|---|
| `DraftStudio.exe` | App desktop (cửa sổ Tauri, giao diện phòng dựng). `DraftStudio.exe serve` mở trong trình duyệt ở http://127.0.0.1:8797 |
| `WebView2Loader.dll` | Phải nằm cạnh `DraftStudio.exe` (bản build GNU của Tauri nạp WebView2 qua DLL này) |
| `capcut_draft_app.exe` | CLI + MCP server (`capcut_draft_app.exe mcp`) cho Claude Desktop / Cursor / Antigravity |
| craft CLI | `{vectorcraft,effectcraft,filmcraft}-cli.exe`. Bản zip phát hành đã kèm sẵn trong `crafts/` (giấy phép ở `crafts/licenses/`). Tự dò theo thứ tự: đường dẫn trong Cài đặt → biến môi trường `VECTORCRAFT_CLI`… → thư mục `crafts/` cạnh app → cạnh app → `PATH` → thư mục craft_engines (Cài đặt hoặc biến `CRAFT_ENGINES_DIR`) |

## Draft CapCut được tạo ra

- **Track hình chính**: ảnh/video nối tiếp; Ken Burns (zoom) và lia trái/phải/lên/xuống bằng keyframe `KFTypeScale*` / `KFTypePosition*`.
- **Track overlay** (`flag 2`, `render_index` tăng dần, giống CapCut tự ghi): đồ họa và chuyển động; tự chia nhiều track khi chồng thời gian; fade bằng keyframe `KFTypeAlpha`.
- **Giọng đọc**: nối tiếp hoặc theo giờ bắt đầu riêng; tuỳ chọn cho ảnh tự khớp độ dài lời. Có nhiều file giọng thì dùng **đoạn** (xem dưới): mỗi giọng một nhóm cảnh, độ dài cảnh tự khớp.
- **Nhạc nền**: lặp tới hết video, fade đầu/cuối (`audio_fades`), tự hạ khi có giọng (keyframe `KFTypeVolume`, mặc định 30% khi lặng → 12% dưới giọng).
- Đăng ký vào `root_meta_info.json` (dự án nằm đầu tab Projects). Dựng lại cùng tên: bản cũ được **dời** sang `CapCut/User Data/Projects/capcut_studio_backups/` (giữ 3 bản), vì CapCut giữ timeline sống trong `Timelines/` và sẽ bỏ qua file ghi đè.
- Khử tiền tố `\\?\` gây crash / "media not found".

> CapCut ghi đè danh sách dự án khi thoát. Nếu CapCut đang mở lúc xuất, tắt hẳn rồi mở lại.

## CLI

```bash
capcut_draft_app status                     # CapCut + 3 engine + ffmpeg
capcut_draft_app list | inspect "Tên draft" | repair "Tên draft"
capcut_draft_app create --spec examples/narrated_sections.json
capcut_draft_app slideshow --name "Thử" --images C:/media/anh --voice C:/media/giong.mp3 --bgm C:/media/nhac.m4a
capcut_draft_app templates                  # mẫu đồ họa + preset chuyển động
capcut_draft_app graphic --template lower_third --field "name=Seneca" --field "role=Triết gia"
capcut_draft_app motion --preset slide_left --template lower_third --duration 5
capcut_draft_app probe C:/media/clip.mp4
```

### Đoạn (`sections`): cảnh tự khớp theo từng file giọng

Mỗi đoạn gắn một file giọng với nhóm cảnh của nó; không cần tính `duration_sec` cho từng clip:

```json
"sections": [
  {"voice": {"path": "C:/media/voice/01.mp3"}, "scenes": [{"path": "C:/media/clips/a.mp4"}, {"path": "C:/media/clips/b.mp4"}]},
  {"voice": {"path": "C:/media/voice/02.mp3"}, "gap_after_sec": 0.5,
   "scenes": [{"path": "C:/media/photos/adler.jpg", "duration_sec": 6, "ken_burns": true}, {"path": "C:/media/clips/c.mp4"}]}
],
"overlays": [{"kind": "motion", "section": 1, "start_sec": 0.6, "graphic": {"template": "lower_third", "fields": {"name": "Alfred Adler"}}, "motion": {"preset": "slide_left"}}]
```

- Các đoạn xếp từ 0 giây, nối tiếp: đoạn N+1 bắt đầu **đúng** lúc giọng N hết (+ `gap_after_sec`, khoảng lặng do hình của đoạn N phủ). Mỗi ranh giới đoạn là một điểm cắt trên track hình, trùng tới micro giây với điểm bắt đầu giọng.
- Trong đoạn, cảnh có `duration_sec` giữ nguyên (video dài hơn file thì rút về độ dài file). Cảnh không đặt thời lượng chia nhau phần còn lại: **ảnh giãn lấp chỗ trống**; **video bị cắt bớt đuôi theo tỉ lệ**, không bao giờ kéo dài quá độ dài file (sau `trim_start_sec`). Khi phải cắt video, mỗi ảnh tự do vẫn giữ ít nhất 2 giây.
- Hình không đủ phủ giọng (chỉ có video, hoặc mọi cảnh đều đặt sẵn thời lượng) → lỗi nói rõ đoạn nào thiếu bao nhiêu giây, thay vì để đen. Cách sửa: thêm cảnh, thêm một ảnh (ảnh tự giãn), hoặc bỏ `duration_sec` ở một ảnh.
- Giọng trong đoạn không dùng `start_sec`. Overlay có `section` (đếm từ 0) thì `start_sec` tính từ đầu đoạn đó, nên đồ họa đi theo câu thoại dù file giọng đổi độ dài.
- `scenes` / `voiceovers` ở cấp ngoài vẫn dùng như cũ và nằm sau đoạn cuối (`fit_scenes_to_voice` chia phần giọng còn lại sau các đoạn).
- Studio: tab Cảnh → **Thêm đoạn theo giọng đọc** (mỗi file giọng một đoạn), nút + trên từng đoạn để thêm cảnh; timeline tính đúng như bản dựng (`compute()` / `fitSection` trong `ui/app.js` là bản sao của `build::fit_section`).

Ví dụ: [examples/narrated_sections.json](examples/narrated_sections.json). Thử trên một tập thật (8 đoạn giọng, 35 cảnh, 8 chuyển động EffectCraft): timeline ra đúng tổng độ dài 8 file giọng (117,65 s), mọi ranh giới đoạn trùng điểm bắt đầu giọng, không clip nào chạy quá file.

Spec v1 (`video_clips`, `bgm_tracks`) vẫn đọc được. Overlay có `kind` = `file` | `graphic` (VectorCraft) | `motion` (EffectCraft); overlay `file` (cảnh chèn B-roll, ảnh) luôn nằm dưới đồ họa và chuyển động, nên chữ không bị cảnh chèn che. Cảnh video có âm lượng > 0 giữ tiếng gốc cả trong MP4 xem trước của FilmCraft. Ví dụ trình chiếu ảnh dọc 9:16 khớp theo giọng: [examples/slideshow.json](examples/slideshow.json). Đường dẫn trong ví dụ là giả — thay bằng file của bạn.

## MCP (cho agent)

```json
{ "mcpServers": { "draft-studio": { "command": "C:\\path\\to\\capcut_draft_app.exe", "args": ["mcp"] } } }
```

Tool: `capcut_get_status`, `capcut_list_drafts`, `capcut_get_draft`, `capcut_live_edit`, `capcut_build_project`, `capcut_create_draft` (v1), `capcut_quick_slideshow`, `capcut_repair_draft`, `craft_list_templates`, `vectorcraft_render_graphic`, `effectcraft_render_motion`, `filmcraft_probe`. Giao diện và agent gọi cùng một hàm (`capcut_core::mcp::call_tool`).

### Xem agent dựng trực tiếp

Để Draft Studio mở trong lúc agent làm việc: mỗi lệnh MCP hiện ra ngay khi nó chạy.

- Chỉ báo **Agent** trên thanh trên nhấp nháy khi có lệnh đang chạy. Tab **Agent** bên trái liệt kê các lệnh gần đây, mỗi lệnh có nhật ký chạy trực tiếp kiểu terminal.
- Lệnh dựng (`capcut_build_project`, `capcut_create_draft`, `capcut_quick_slideshow`) đưa spec của agent lên khung xem và timeline, chỉ để xem. Overlay hiện dần khi VectorCraft / EffectCraft làm xong (chuyển động WebM phát được; `.mov` ProRes thì hiện ảnh đồ họa vì trình duyệt không phát ProRes), và MP4 FilmCraft tự phát ngay khi render xong.
- Dự án bạn đang mở được lưu trước và để nguyên: **Quay lại dự án** để trở về, **Mở thành dự án** lưu spec của agent thành một dự án mới trong studio để sửa tiếp.
- Studio tự chuyển sang bản dựng khi agent bắt đầu, trừ khi bạn đang gõ hoặc đang mở hộp thoại (khi đó chỉ có thông báo kèm nút **Xem**). Cài đặt → *Xem agent làm việc* để tắt việc tự chuyển, và có tuỳ chọn (mặc định tắt) **tự mở Draft Studio khi agent bắt đầu dựng**: MCP server mở `DraftStudio.exe` cùng thư mục nếu chưa có studio nào chạy, còn studio đang mở thì cửa sổ được đưa lên trước.
- Mở studio giữa chừng vẫn thấy toàn bộ lệnh từ đầu. Studio tắt thì agent không bị ảnh hưởng gì: sự kiện gửi kiểu bắn-rồi-quên trên một luồng nền (HTTP nội bộ, timeout ngắn, hàng đợi có giới hạn), nhật ký dựng vẫn ra stderr như cũ.

Bên trong: MCP server gửi sự kiện tới `POST /api/activity` trên cổng studio (8797, hoặc `CAPCUT_STUDIO_PORT`), mọi giao diện đang mở theo dõi `GET /api/activity/stream` (Server-Sent Events); xem `crates/core/src/activity.rs` và `apps/studio/src/server.rs`.

## Mã nguồn

```
crates/core     draft engine, spec, thư viện draft, cầu nối craft, mẫu đồ họa, MCP, kênh hoạt động trực tiếp tới studio (activity.rs)
apps/cli        capcut_draft_app.exe
apps/studio     DraftStudio.exe — axum (API + UI nhúng) trong cửa sổ Tauri; ui/ là HTML/CSS/JS thuần
```

Build — máy có Visual Studio Build Tools (MSVC):

```bash
cargo test -p capcut-core
cargo build --release -p capcut-studio -p capcut-draft-cli
```

Máy không có MSVC: dùng GNU toolchain + MinGW (`winget install BrechtSanders.WinLibs.POSIX.MSVCRT`, `rustup toolchain install stable-x86_64-pc-windows-gnu`), rồi `source env.sh` trong Git Bash trước hai lệnh trên. Bản GNU ra ở `target/x86_64-pc-windows-gnu/release/` và cần `WebView2Loader.dll` cạnh `CapCutDraftStudio.exe`.

Craft engines: clone và build ba repo [vectorcraft](https://github.com/storytold/vectorcraft), [effectcraft](https://github.com/storytold/effectcraft), [filmcraft](https://github.com/storytold/filmcraft) (`cargo build --release -p <tên>-cli`), rồi đặt ba file `-cli.exe` vào `crafts/` cạnh app hoặc chỉ thư mục chứa chúng trong Cài đặt.
