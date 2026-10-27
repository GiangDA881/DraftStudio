//! Broadcast graphic templates, written as canvas-sized SVG documents that VectorCraft opens and
//! exports to transparent PNG (and to an editable `.vectorcraft` file). Sizes scale with the
//! canvas, so the same template works for 16:9, 9:16 and 1:1.

use anyhow::{bail, Result};
use serde::Serialize;

use crate::spec::GraphicSpec;

#[derive(Debug, Clone, Serialize)]
pub struct FieldInfo {
    pub key: &'static str,
    pub label: &'static str,
    pub default: &'static str,
    pub multiline: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub fields: Vec<FieldInfo>,
    /// Suggested on-screen time in seconds.
    pub suggested_sec: f64,
}

fn f(key: &'static str, label: &'static str, default: &'static str) -> FieldInfo {
    FieldInfo { key, label, default, multiline: false }
}

fn fm(key: &'static str, label: &'static str, default: &'static str) -> FieldInfo {
    FieldInfo { key, label, default, multiline: true }
}

pub const DEFAULT_ACCENT: &str = "#E8B04B";
pub const DEFAULT_FONT: &str = "Segoe UI";

pub fn templates() -> Vec<TemplateInfo> {
    vec![
        TemplateInfo {
            id: "lower_third",
            name: "Lower third",
            description: "Tên + chức danh góc dưới trái, thanh nhấn màu",
            fields: vec![f("name", "Tên", "Lucius Annaeus Seneca"), f("role", "Chức danh", "Triết gia Khắc kỷ · 4 TCN – 65 SCN")],
            suggested_sec: 5.0,
        },
        TemplateInfo {
            id: "chapter_title",
            name: "Tiêu đề chương",
            description: "Tiêu đề lớn ở giữa, dòng kicker và phụ đề",
            fields: vec![f("kicker", "Dòng nhỏ phía trên", "CHƯƠNG I"), f("title", "Tiêu đề", "Cái giá của quyền lực"), f("subtitle", "Phụ đề", "Rome, năm 54 sau Công nguyên")],
            suggested_sec: 4.0,
        },
        TemplateInfo {
            id: "quote_card",
            name: "Trích dẫn",
            description: "Câu trích dẫn trong khung, dấu ngoặc kép màu nhấn",
            fields: vec![fm("quote", "Trích dẫn", "Chúng ta khổ trong tưởng tượng\nnhiều hơn trong thực tế."), f("author", "Tác giả", "Seneca, Thư gửi Lucilius")],
            suggested_sec: 6.0,
        },
        TemplateInfo {
            id: "location_tag",
            name: "Địa điểm · thời gian",
            description: "Nhãn nhỏ góc trên trái: nơi chốn và năm",
            fields: vec![f("place", "Địa điểm", "ROMA"), f("date", "Thời gian", "65 SCN")],
            suggested_sec: 4.0,
        },
        TemplateInfo {
            id: "headline_bar",
            name: "Thanh tin",
            description: "Thanh tiêu đề ngang đáy màn hình kiểu bản tin",
            fields: vec![f("tag", "Nhãn", "LỊCH SỬ"), f("headline", "Tiêu đề", "Nero ra lệnh cho thầy mình tự kết liễu")],
            suggested_sec: 6.0,
        },
        TemplateInfo {
            id: "subtitle",
            name: "Phụ đề nổi",
            description: "Sub Việt nổi bật đáy khung hình với nhãn từ khóa phát sáng",
            fields: vec![f("tag", "Từ khóa", "ĐIỂM NHẤN"), f("text", "Câu phụ đề", "Chỉ một câu lệnh tiếng Việt, toàn bộ hiệu ứng tự động khớp từng giây")],
            suggested_sec: 4.5,
        },
        TemplateInfo {
            id: "fact_box",
            name: "Con số",
            description: "Con số lớn và chú thích, bên phải khung hình",
            fields: vec![f("value", "Con số", "124"), f("label", "Chú thích", "bức thư gửi Lucilius còn lại")],
            suggested_sec: 5.0,
        },
        TemplateInfo {
            id: "subscribe",
            name: "Nút đăng ký",
            description: "Viên thuốc đỏ có biểu tượng play, góc dưới phải",
            fields: vec![f("text", "Chữ", "ĐĂNG KÝ"), f("handle", "Tên kênh", "@TrietHocDem")],
            suggested_sec: 4.0,
        },
        TemplateInfo {
            id: "end_card",
            name: "Outro kênh",
            description: "Màn kết: tên kênh, khẩu hiệu, tập tiếp theo, nút đăng ký (chừa chỗ avatar phía trên)",
            fields: vec![
                f("channel", "Tên kênh", "LĂNG KÍNH VÔ THỨC"),
                fm("tagline", "Khẩu hiệu", "Nhìn thấu những góc khuất tâm hồn\nqua lăng kính điện ảnh và triết học."),
                f("next_label", "Nhãn tập sau", "TẬP TIẾP THEO"),
                f("next_title", "Tên tập sau (trống = lời hẹn)", ""),
                f("cta", "Nút", "ĐĂNG KÝ KÊNH"),
            ],
            suggested_sec: 9.0,
        },
        TemplateInfo {
            id: "car_spec_hud",
            name: "HUD Thông số Siêu xe / Tech",
            description: "Bảng HUD thông số kỹ thuật góc phải dưới kiểu review siêu xe & công nghệ",
            fields: vec![
                f("model", "Tên mẫu / Phiên bản", "PORSCHE 911 GT3 RS"),
                f("hp", "Công suất / Chỉ số 1", "518 HP · 9000 RPM"),
                f("speed", "Tốc độ / Chỉ số 2", "0–100 KM/H: 3.2S"),
                f("note", "Ghi chú / Giá", "KHÍ ĐỘNG HỌC CHỦ ĐỘNG DRS"),
            ],
            suggested_sec: 5.0,
        },
        TemplateInfo {
            id: "vs_compare",
            name: "So sánh Đối đầu (VS Card)",
            description: "Khung so sánh 2 đối thủ đối xứng có huy hiệu VS rực sáng ở tâm",
            fields: vec![
                f("left_title", "Vế trái", "ĐỘNG CƠ V8 TỰ NHIÊN"),
                f("left_val", "Thông số trái", "8.500 VÒNG/PHÚT"),
                f("right_title", "Vế phải", "TĂNG ÁP KÉP BI-TURBO"),
                f("right_val", "Thông số phải", "800 NM MÔ-MEN XOẮN"),
            ],
            suggested_sec: 5.5,
        },
        TemplateInfo {
            id: "breaking_banner",
            name: "Cảnh báo Nóng (Breaking Alert)",
            description: "Băng rôn khẩn cấp tương phản cao kiểu tin chấn động / cảnh báo",
            fields: vec![
                f("badge", "Nhãn cảnh báo", "CẢNH BÁO NÓNG"),
                f("title", "Tiêu đề chính", "BƯỚC NGOẶT LỊCH SỬ KHÔNG THỂ ĐẢO NGƯỢC"),
                f("sub", "Chi tiết phụ", "Tài liệu giải mật lần đầu được công bố"),
            ],
            suggested_sec: 5.0,
        },
        TemplateInfo {
            id: "price_tag",
            name: "Nhãn Giá & Đánh giá (Price Badge)",
            description: "Huy hiệu giá tiền & điểm số góc trên phải khung hình",
            fields: vec![
                f("item", "Hạng mục", "GIÁ KHỞI ĐIỂM"),
                f("price", "Mức giá / Con số", "$241,300"),
                f("rating", "Đánh giá", "★ 9.8 / 10 TUYỆT TÁC"),
            ],
            suggested_sec: 4.5,
        },
        TemplateInfo {
            id: "neon_title",
            name: "Tiêu đề Cyberpunk Neon",
            description: "Chữ lớn viền phát sáng kép kiểu Car Edit / Sci-Fi hiện đại",
            fields: vec![
                f("top", "Dòng mở đầu", "NIGHT RUN EDITION"),
                f("main", "Tiêu đề Neon", "APEX PREDATOR"),
                f("bottom", "Dòng chân trang", "TỐC ĐỘ · CHÍNH XÁC · ĐẲNG CẤP"),
            ],
            suggested_sec: 4.0,
        },
        TemplateInfo {
            id: "step_callout",
            name: "Bước Quy trình (Step Badge)",
            description: "Nhãn đánh số Bước 01, 02... góc trên trái kèm tiêu đề và giải thích",
            fields: vec![
                f("step", "Số thứ tự", "BƯỚC 01"),
                f("title", "Tên bước", "Khởi động hệ thống tăng áp"),
                f("detail", "Mô tả ngắn", "Áp suất buồng đốt đạt ngưỡng tối ưu trong 0.8 giây"),
            ],
            suggested_sec: 5.0,
        },
        TemplateInfo {
            id: "stat_bar",
            name: "Thanh Chỉ số Phần trăm (Stat Bar)",
            description: "Thanh tiến trình đồ họa trực quan hiển thị tỷ lệ %",
            fields: vec![
                f("label", "Tên chỉ số", "HIỆU SUẤT KHÍ ĐỘNG HỌC"),
                f("percent", "Phần trăm (0-100)", "94%"),
                f("note", "Chú thích", "Lực ép xuống mặt đường 860 kg ở 285 km/h"),
            ],
            suggested_sec: 5.0,
        },
        TemplateInfo {
            id: "social_handle",
            name: "Tag Mạng xã hội (Social Pill)",
            description: "Viên thuốc hiện đại góc trên phải (YOUTUBE / TIKTOK · @Channel)",
            fields: vec![
                f("platform", "Nền tảng", "YOUTUBE · TIKTOK"),
                f("handle", "Tài khoản", "@TrietHocDem"),
            ],
            suggested_sec: 4.5,
        },
        TemplateInfo {
            id: "minimal_corner",
            name: "Nhãn Góc Tối giản (Watermark)",
            description: "Đóng dấu thương hiệu tinh tế ở góc trên trái màn hình",
            fields: vec![
                f("title", "Thương hiệu", "DRAFT STUDIO PRO"),
                f("sub", "Phụ đề nhỏ", "4K CINEMATIC MASTER"),
            ],
            suggested_sec: 6.0,
        },
        TemplateInfo {
            id: "top_list_badge",
            name: "Huy hiệu Xếp hạng (Top Rank #1)",
            description: "Huy hiệu xếp hạng Top 10 / #1 nổi bật góc trái",
            fields: vec![
                f("rank", "Thứ hạng", "#01"),
                f("title", "Tên hạng mục", "Kiệt tác cơ khí thế kỷ 21"),
                f("tag", "Nhãn phụ", "BẢNG XẾP HẠNG ĐẶC BIỆT"),
            ],
            suggested_sec: 4.5,
        },
        TemplateInfo {
            id: "cinematic_letterbox",
            name: "Khung Điện ảnh 2.39:1 & Phụ đề",
            description: "Hai dải đen điện ảnh trên dưới kèm dòng chú thích nghệ thuật",
            fields: vec![
                f("caption", "Câu thoại / Chú thích", "Trong tĩnh lặng, tiếng gầm cơ khí trở thành bản giao hưởng."),
                f("credit", "Ghi chú góc", "DIRECTOR'S CUT · 24 FPS"),
            ],
            suggested_sec: 5.5,
        },
    ]
}

pub fn template(id: &str) -> Option<TemplateInfo> {
    templates().into_iter().find(|t| t.id == id)
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn valid_hex(c: &str) -> bool {
    c.len() == 7 && c.starts_with('#') && c[1..].chars().all(|ch| ch.is_ascii_hexdigit())
}

/// Rough advance width of a line of text, for sizing panels (VectorCraft lays out the real text).
fn text_w(s: &str, size: f64, bold: bool) -> f64 {
    let per = if bold { 0.58 } else { 0.52 };
    s.chars()
        .map(|c| if c == ' ' { 0.3 } else if c.is_uppercase() || c.is_ascii_digit() { 1.12 } else { 1.0 })
        .sum::<f64>()
        * size
        * per
}

struct Ctx {
    w: f64,
    h: f64,
    u: f64,
    accent: String,
    font: String,
}

impl Ctx {
    /// Distance from the bottom edge for lower graphics: vertical video keeps clear of the
    /// captions and buttons TikTok / Shorts draw over the bottom fifth.
    fn bottom(&self, landscape: f64) -> f64 {
        if self.h > self.w {
            self.h * 0.21
        } else {
            landscape * self.u
        }
    }

    fn text(&self, x: f64, y: f64, size: f64, weight: u32, fill: &str, anchor: &str, extra: &str, s: &str) -> String {
        format!(
            r#"<text x="{x:.1}" y="{y:.1}" font-family="{font}" font-size="{size:.1}" font-weight="{weight}" fill="{fill}" text-anchor="{anchor}" {extra}>{t}</text>"#,
            font = esc(&self.font),
            t = esc(s)
        )
    }
}

/// The SVG document for one graphic on a `w`×`h` transparent canvas.
pub fn render_svg(g: &GraphicSpec, w: u32, h: u32) -> Result<String> {
    let t = template(&g.template).ok_or_else(|| anyhow::anyhow!("không có mẫu đồ họa '{}'", g.template))?;
    let get = |k: &str| -> String {
        g.fields.get(k).cloned().unwrap_or_else(|| t.fields.iter().find(|fi| fi.key == k).map(|fi| fi.default.to_string()).unwrap_or_default())
    };
    let accent = g.accent.clone().filter(|c| valid_hex(c)).unwrap_or_else(|| DEFAULT_ACCENT.into());
    let font = g.font.clone().filter(|f| !f.trim().is_empty()).unwrap_or_else(|| DEFAULT_FONT.into());
    let (wf, hf) = (w as f64, h as f64);
    let c = Ctx { w: wf, h: hf, u: wf.min(hf) / 1080.0, accent, font };
    let body = match t.id {
        "lower_third" => lower_third(&c, &get("name"), &get("role")),
        "chapter_title" => chapter_title(&c, &get("kicker"), &get("title"), &get("subtitle")),
        "quote_card" => quote_card(&c, &get("quote"), &get("author")),
        "location_tag" => location_tag(&c, &get("place"), &get("date")),
        "headline_bar" => headline_bar(&c, &get("tag"), &get("headline")),
        "subtitle" => subtitle(&c, &get("tag"), &get("text")),
        "fact_box" => fact_box(&c, &get("value"), &get("label")),
        "subscribe" => subscribe(&c, &get("text"), &get("handle")),
        "end_card" => end_card(&c, &get("channel"), &get("tagline"), &get("next_label"), &get("next_title"), &get("cta")),
        "car_spec_hud" => car_spec_hud(&c, &get("model"), &get("hp"), &get("speed"), &get("note")),
        "vs_compare" => vs_compare(&c, &get("left_title"), &get("left_val"), &get("right_title"), &get("right_val")),
        "breaking_banner" => breaking_banner(&c, &get("badge"), &get("title"), &get("sub")),
        "price_tag" => price_tag(&c, &get("item"), &get("price"), &get("rating")),
        "neon_title" => neon_title(&c, &get("top"), &get("main"), &get("bottom")),
        "step_callout" => step_callout(&c, &get("step"), &get("title"), &get("detail")),
        "stat_bar" => stat_bar(&c, &get("label"), &get("percent"), &get("note")),
        "social_handle" => social_handle(&c, &get("platform"), &get("handle")),
        "minimal_corner" => minimal_corner(&c, &get("title"), &get("sub")),
        "top_list_badge" => top_list_badge(&c, &get("rank"), &get("title"), &get("tag")),
        "cinematic_letterbox" => cinematic_letterbox(&c, &get("caption"), &get("credit")),
        other => bail!("mẫu '{other}' chưa vẽ được"),
    };
    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<defs>
  <linearGradient id="panel" x1="0" y1="0" x2="1" y2="0">
    <stop offset="0" stop-color="#0B0E13" stop-opacity="0.92"/>
    <stop offset="0.75" stop-color="#0B0E13" stop-opacity="0.78"/>
    <stop offset="1" stop-color="#0B0E13" stop-opacity="0"/>
  </linearGradient>
  <linearGradient id="band" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#000000" stop-opacity="0"/>
    <stop offset="0.5" stop-color="#000000" stop-opacity="0.55"/>
    <stop offset="1" stop-color="#000000" stop-opacity="0"/>
  </linearGradient>
  <linearGradient id="accentfade" x1="0" y1="0" x2="1" y2="0">
    <stop offset="0" stop-color="{a}" stop-opacity="0"/>
    <stop offset="0.5" stop-color="{a}" stop-opacity="1"/>
    <stop offset="1" stop-color="{a}" stop-opacity="0"/>
  </linearGradient>
</defs>
{body}
</svg>
"##,
        a = c.accent
    ))
}

fn lower_third(c: &Ctx, name: &str, role: &str) -> String {
    let u = c.u;
    let (ns, rs) = (54.0 * u, 30.0 * u);
    let x = 96.0 * u;
    let tw = text_w(name, ns, true).max(text_w(role, rs, false));
    let pw = tw + 150.0 * u;
    let ph = 150.0 * u;
    let y = c.h - c.bottom(130.0) - ph;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" fill="url(#panel)"/>
<rect x="{x:.1}" y="{y:.1}" width="{bw:.1}" height="{ph:.1}" fill="{a}"/>
<rect x="{lx:.1}" y="{ly:.1}" width="{lw:.1}" height="{lh:.1}" fill="{a}" opacity="0.85"/>
{n}
{r}"##,
        bw = 9.0 * u,
        a = c.accent,
        lx = x + 40.0 * u,
        ly = y + 86.0 * u,
        lw = 64.0 * u,
        lh = 3.0 * u,
        n = c.text(x + 40.0 * u, y + 68.0 * u, ns, 700, "#FFFFFF", "start", "", name),
        r = c.text(x + 40.0 * u, y + 126.0 * u, rs, 400, "#D9DEE7", "start", "", role),
    )
}

fn chapter_title(c: &Ctx, kicker: &str, title: &str, subtitle: &str) -> String {
    let u = c.u;
    let cx = c.w / 2.0;
    let cy = c.h / 2.0;
    let mut ts = 96.0 * u;
    let max_w = c.w * 0.86;
    let tw = text_w(title, ts, true);
    if tw > max_w {
        ts *= max_w / tw;
    }
    let ks = 30.0 * u;
    let kw = text_w(kicker, ks, true) + kicker.chars().count() as f64 * 6.0 * u;
    let line = 120.0 * u;
    let gap = 28.0 * u;
    format!(
        r##"<rect x="0" y="{by:.1}" width="{w:.1}" height="{bh:.1}" fill="url(#band)"/>
<rect x="{l1:.1}" y="{ky:.1}" width="{line:.1}" height="{lt:.1}" fill="{a}"/>
<rect x="{l2:.1}" y="{ky:.1}" width="{line:.1}" height="{lt:.1}" fill="{a}"/>
{k}
{t}
{s}"##,
        by = cy - 260.0 * u,
        w = c.w,
        bh = 520.0 * u,
        l1 = cx - kw / 2.0 - gap - line,
        l2 = cx + kw / 2.0 + gap,
        ky = cy - 112.0 * u,
        lt = 3.0 * u,
        a = c.accent,
        k = c.text(cx, cy - 100.0 * u, ks, 700, &c.accent, "middle", &format!(r#"letter-spacing="{:.1}""#, 6.0 * u), kicker),
        t = c.text(cx, cy + 20.0 * u, ts, 700, "#FFFFFF", "middle", "", title),
        s = c.text(cx, cy + 92.0 * u, 36.0 * u, 400, "#E6E9EF", "middle", r#"font-style="italic""#, subtitle),
    )
}

fn quote_card(c: &Ctx, quote: &str, author: &str) -> String {
    let u = c.u;
    let lines: Vec<&str> = quote.lines().filter(|l| !l.trim().is_empty()).take(4).collect();
    let mut qs = 54.0 * u;
    let widest = lines.iter().map(|l| text_w(l, qs, false)).fold(0.0, f64::max);
    let max_w = c.w * 0.72;
    if widest > max_w {
        qs *= max_w / widest;
    }
    let lh = qs * 1.3;
    let pw = (lines.iter().map(|l| text_w(l, qs, false)).fold(0.0, f64::max) + 200.0 * u).max(700.0 * u).min(c.w * 0.9);
    let ph = lines.len() as f64 * lh + 220.0 * u;
    let x = (c.w - pw) / 2.0;
    let y = (c.h - ph) / 2.0;
    let mut out = format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#0B0E13" fill-opacity="0.82"/>
<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{bt:.1}" rx="{bt2:.1}" fill="{a}"/>
{q}
"##,
        r = 18.0 * u,
        bt = 6.0 * u,
        bt2 = 3.0 * u,
        a = c.accent,
        q = c.text(x + 60.0 * u, y + 150.0 * u, 160.0 * u, 700, &c.accent, "start", r#"opacity="0.9""#, "\u{201C}"),
    );
    let cx = c.w / 2.0;
    for (i, l) in lines.iter().enumerate() {
        out.push_str(&c.text(cx, y + 120.0 * u + i as f64 * lh + qs * 0.4, qs, 400, "#FFFFFF", "middle", r#"font-style="italic""#, l));
        out.push('\n');
    }
    out.push_str(&c.text(cx, y + ph - 56.0 * u, 30.0 * u, 600, &c.accent, "middle", "", &format!("— {author}")));
    out
}

fn location_tag(c: &Ctx, place: &str, date: &str) -> String {
    let u = c.u;
    let (ps, ds) = (34.0 * u, 30.0 * u);
    let x = 80.0 * u;
    let y = 80.0 * u;
    let pw = text_w(place, ps, true) * 1.1 + place.chars().count() as f64 * 4.0 * u;
    let dw = text_w(date, ds, false);
    let w = 62.0 * u + pw + 56.0 * u + dw * 1.05 + 34.0 * u;
    let h = 72.0 * u;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" rx="{r:.1}" fill="#0B0E13" fill-opacity="0.8"/>
<circle cx="{dx:.1}" cy="{dy:.1}" r="{dr:.1}" fill="{a}"/>
{p}
<rect x="{sx:.1}" y="{sy:.1}" width="{sw:.1}" height="{sh:.1}" fill="#FFFFFF" fill-opacity="0.35"/>
{d}"##,
        r = h / 2.0,
        dx = x + 36.0 * u,
        dy = y + h / 2.0,
        dr = 9.0 * u,
        a = c.accent,
        p = c.text(x + 62.0 * u, y + 48.0 * u, ps, 700, "#FFFFFF", "start", &format!(r#"letter-spacing="{:.1}""#, 4.0 * u), place),
        sx = x + 62.0 * u + pw + 26.0 * u,
        sy = y + 20.0 * u,
        sw = 2.0 * u,
        sh = 32.0 * u,
        d = c.text(x + 62.0 * u + pw + 54.0 * u, y + 47.0 * u, ds, 400, "#D9DEE7", "start", "", date),
    )
}

fn headline_bar(c: &Ctx, tag: &str, headline: &str) -> String {
    let u = c.u;
    let h = 112.0 * u;
    let y = c.h - c.bottom(90.0) - h;
    let ts = 30.0 * u;
    let tw = text_w(tag, ts, true) + tag.chars().count() as f64 * 4.0 * u + 64.0 * u;
    let mut hs = 44.0 * u;
    let avail = c.w - tw - 160.0 * u;
    let hw = text_w(headline, hs, true);
    if hw > avail {
        hs *= avail / hw;
    }
    format!(
        r##"<rect x="0" y="{y:.1}" width="{w:.1}" height="{h:.1}" fill="#0B0E13" fill-opacity="0.88"/>
<rect x="0" y="{y:.1}" width="{w:.1}" height="{lt:.1}" fill="url(#accentfade)"/>
<rect x="{tx:.1}" y="{ty:.1}" width="{tw:.1}" height="{th:.1}" fill="{a}"/>
{t}
{hd}"##,
        w = c.w,
        lt = 4.0 * u,
        tx = 60.0 * u,
        ty = y + 26.0 * u,
        th = h - 52.0 * u,
        a = c.accent,
        t = c.text(60.0 * u + tw / 2.0, y + h / 2.0 + 11.0 * u, ts, 800, "#0B0E13", "middle", &format!(r#"letter-spacing="{:.1}""#, 4.0 * u), tag),
        hd = c.text(60.0 * u + tw + 36.0 * u, y + h / 2.0 + 16.0 * u, hs, 700, "#FFFFFF", "start", "", headline),
    )
}

fn subtitle(c: &Ctx, tag: &str, text: &str) -> String {
    let u = c.u;
    let h = 104.0 * u;
    let y = c.h - c.bottom(84.0) - h;
    let ts = 28.0 * u;
    let has_tag = !tag.trim().is_empty();
    let tw = if has_tag { text_w(tag, ts, true) + tag.chars().count() as f64 * 3.5 * u + 52.0 * u } else { 0.0 };
    let mut hs = 44.0 * u;
    let max_box = c.w * 0.90;
    let avail = max_box - tw - if has_tag { 130.0 * u } else { 90.0 * u };
    let hw_raw = text_w(text, hs, true);
    if hw_raw > avail && hw_raw > 0.0 {
        hs *= avail / hw_raw;
    }
    let hw = text_w(text, hs, true);
    let pad = 42.0 * u;
    let gap = if has_tag { 28.0 * u } else { 0.0 };
    let bw = (pad * 2.0 + tw + gap + hw).max(520.0 * u).min(max_box);
    let bx = (c.w - bw) / 2.0;
    let tag_svg = if has_tag {
        let tx = bx + 24.0 * u;
        let ty = y + 20.0 * u;
        let th = h - 40.0 * u;
        format!(
            r##"<rect x="{tx:.1}" y="{ty:.1}" width="{tw:.1}" height="{th:.1}" rx="{tr:.1}" fill="{a}"/>
{t}"##,
            tr = th / 2.0,
            a = c.accent,
            t = c.text(tx + tw / 2.0, y + h / 2.0 + 10.0 * u, ts, 800, "#080B12", "middle", &format!(r#"letter-spacing="{:.1}""#, 3.0 * u), tag),
        )
    } else {
        String::new()
    };
    let text_x = if has_tag { bx + 24.0 * u + tw + gap } else { c.w / 2.0 };
    let anchor = if has_tag { "start" } else { "middle" };
    format!(
        r##"<rect x="{bx:.1}" y="{y:.1}" width="{bw:.1}" height="{h:.1}" rx="{r:.1}" fill="#080B12" fill-opacity="0.90" stroke="{a}" stroke-width="{sw:.1}" stroke-opacity="0.75"/>
<rect x="{lx:.1}" y="{y:.1}" width="{lw:.1}" height="{lt:.1}" rx="{lt2:.1}" fill="{a}"/>
{tag_svg}
{txt}"##,
        r = 24.0 * u,
        a = c.accent,
        sw = 2.5 * u,
        lx = bx + bw * 0.12,
        lw = bw * 0.76,
        lt = 5.0 * u,
        lt2 = 2.5 * u,
        txt = c.text(text_x, y + h / 2.0 + 15.0 * u, hs, 700, "#FFFFFF", anchor, "", text),
    )
}

fn fact_box(c: &Ctx, value: &str, label: &str) -> String {
    let u = c.u;
    let vs = 150.0 * u;
    let ls = 34.0 * u;
    let w = text_w(value, vs, true).max(text_w(label, ls, false)) + 120.0 * u;
    let h = 300.0 * u;
    let x = c.w - 100.0 * u - w;
    let y = (c.h - h) / 2.0;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" rx="{r:.1}" fill="#0B0E13" fill-opacity="0.82"/>
<rect x="{x:.1}" y="{by:.1}" width="{bw:.1}" height="{bh:.1}" fill="{a}"/>
{v}
{l}"##,
        r = 14.0 * u,
        by = y + 40.0 * u,
        bw = 8.0 * u,
        bh = h - 80.0 * u,
        a = c.accent,
        v = c.text(x + 60.0 * u, y + 180.0 * u, vs, 800, &c.accent, "start", "", value),
        l = c.text(x + 62.0 * u, y + 240.0 * u, ls, 400, "#E6E9EF", "start", "", label),
    )
}

fn subscribe(c: &Ctx, text: &str, handle: &str) -> String {
    let u = c.u;
    let ts = 38.0 * u;
    let hs = 28.0 * u;
    let h = 96.0 * u;
    let w = 120.0 * u + text_w(text, ts, true) + text.chars().count() as f64 * 3.0 * u + 50.0 * u;
    let x = c.w - 90.0 * u - w;
    let y = c.h - c.bottom(120.0) - h;
    let (px, py) = (x + 58.0 * u, y + h / 2.0);
    let tri = 15.0 * u;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" rx="{r:.1}" fill="#E62117"/>
<circle cx="{px:.1}" cy="{py:.1}" r="{cr:.1}" fill="#FFFFFF"/>
<path d="M {ax:.1} {ay:.1} L {bx:.1} {py:.1} L {ax:.1} {cy:.1} Z" fill="#E62117"/>
{t}
{hd}"##,
        r = h / 2.0,
        cr = 28.0 * u,
        ax = px - tri * 0.55,
        ay = py - tri,
        bx = px + tri,
        cy = py + tri,
        t = c.text(x + 104.0 * u, y + h / 2.0 + 13.0 * u, ts, 800, "#FFFFFF", "start", &format!(r#"letter-spacing="{:.1}""#, 3.0 * u), text),
        hd = c.text(x + w - 10.0 * u, y + h + 44.0 * u, hs, 600, "#FFFFFF", "end", "", handle),
    )
}

/// Channel outro. The upper third stays empty for the channel avatar (composited by the caller).
fn end_card(c: &Ctx, channel: &str, tagline: &str, next_label: &str, next_title: &str, cta: &str) -> String {
    let u = c.u;
    let cx = c.w / 2.0;
    let mut cs = 76.0 * u;
    let spacing = 12.0 * u;
    let cw = text_w(channel, cs, true) + channel.chars().count() as f64 * spacing;
    if cw > c.w * 0.86 {
        cs *= c.w * 0.86 / cw;
    }
    let y_name = c.h * 0.50;
    let mut out = String::new();
    out.push_str(&format!(r##"<rect x="0" y="{by:.1}" width="{w:.1}" height="{bh:.1}" fill="url(#band)"/>"##, by = y_name - 190.0 * u, w = c.w, bh = 560.0 * u));
    out.push('\n');
    out.push_str(&c.text(cx, y_name, cs, 700, "#FFFFFF", "middle", &format!(r#"letter-spacing="{spacing:.1}""#), channel));
    out.push('\n');
    for (i, line) in tagline.lines().filter(|l| !l.trim().is_empty()).take(2).enumerate() {
        out.push_str(&c.text(cx, y_name + 62.0 * u + i as f64 * 40.0 * u, 30.0 * u, 400, "#D9DEE7", "middle", r#"font-style="italic""#, line));
        out.push('\n');
    }
    let y_div = y_name + 150.0 * u;
    out.push_str(&format!(r##"<rect x="{x:.1}" y="{y:.1}" width="{lw:.1}" height="{lt:.1}" fill="url(#accentfade)"/>"##, x = cx - 220.0 * u, y = y_div, lw = 440.0 * u, lt = 2.0 * u));
    out.push('\n');
    let y_next = y_div + 58.0 * u;
    if next_title.trim().is_empty() {
        out.push_str(&c.text(cx, y_next + 10.0 * u, 34.0 * u, 400, "#E6E9EF", "middle", r#"font-style="italic""#, "Hẹn gặp lại bạn ở tập tiếp theo."));
    } else {
        out.push_str(&c.text(cx, y_next - 6.0 * u, 24.0 * u, 700, &c.accent, "middle", &format!(r#"letter-spacing="{:.1}""#, 6.0 * u), next_label));
        out.push('\n');
        let mut ns = 40.0 * u;
        let nw = text_w(next_title, ns, true);
        if nw > c.w * 0.84 {
            ns *= c.w * 0.84 / nw;
        }
        out.push_str(&c.text(cx, y_next + 46.0 * u, ns, 700, "#FFFFFF", "middle", "", next_title));
    }
    out.push('\n');
    let ts = 30.0 * u;
    let ph = 76.0 * u;
    let pw = 104.0 * u + text_w(cta, ts, true) + cta.chars().count() as f64 * 3.0 * u + 40.0 * u;
    let px = cx - pw / 2.0;
    let py = y_next + 112.0 * u;
    let (ix, iy) = (px + 46.0 * u, py + ph / 2.0);
    let tri = 12.0 * u;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#E62117"/>
<circle cx="{ix:.1}" cy="{iy:.1}" r="{cr:.1}" fill="#FFFFFF"/>
<path d="M {ax:.1} {ay:.1} L {bx:.1} {iy:.1} L {ax:.1} {cy:.1} Z" fill="#E62117"/>
{t}"##,
        r = ph / 2.0,
        cr = 22.0 * u,
        ax = ix - tri * 0.55,
        ay = iy - tri,
        bx = ix + tri,
        cy = iy + tri,
        t = c.text(px + 82.0 * u, py + ph / 2.0 + 10.5 * u, ts, 800, "#FFFFFF", "start", &format!(r#"letter-spacing="{:.1}""#, 3.0 * u), cta),
    ));
    out
}

fn car_spec_hud(c: &Ctx, model: &str, hp: &str, speed: &str, note: &str) -> String {
    let u = c.u;
    let pw = if c.h > c.w { 820.0 * u } else { 560.0 * u };
    let ph = 210.0 * u;
    let x = if c.h > c.w { (c.w - pw) / 2.0 } else { c.w - pw - 86.0 * u };
    let y = if c.h > c.w { c.h * 0.12 } else { c.h - c.bottom(120.0) - ph };
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#0A0E17" fill-opacity="0.88" stroke="{a}" stroke-width="{sw:.1}"/>
<rect x="{x:.1}" y="{y:.1}" width="{bw:.1}" height="{ph:.1}" fill="{a}"/>
{m}
{h1}
{s1}
{n1}"##,
        r = 12.0 * u,
        a = c.accent,
        sw = 2.5 * u,
        bw = 10.0 * u,
        m = c.text(x + 34.0 * u, y + 52.0 * u, 34.0 * u, 800, &c.accent, "start", "", model),
        h1 = c.text(x + 34.0 * u, y + 102.0 * u, 30.0 * u, 700, "#FFFFFF", "start", "", hp),
        s1 = c.text(x + 34.0 * u, y + 146.0 * u, 28.0 * u, 600, "#D9DEE7", "start", "", speed),
        n1 = c.text(x + 34.0 * u, y + 186.0 * u, 22.0 * u, 600, "#9BA6B8", "start", "", note),
    )
}

fn vs_compare(c: &Ctx, lt: &str, lv: &str, rt: &str, rv: &str) -> String {
    let u = c.u;
    let cx = c.w / 2.0;
    let cy = if c.h > c.w { c.h * 0.18 } else { c.h / 2.0 };
    let bw = 560.0 * u;
    let bh = 220.0 * u;
    let lx = cx - bw - 48.0 * u;
    let rx = cx + 48.0 * u;
    let y = cy - bh / 2.0;
    format!(
        r##"<rect x="{lx:.1}" y="{y:.1}" width="{bw:.1}" height="{bh:.1}" rx="{r:.1}" fill="#0B0F18" fill-opacity="0.90" stroke="{a}" stroke-width="{sw:.1}"/>
<rect x="{rx:.1}" y="{y:.1}" width="{bw:.1}" height="{bh:.1}" rx="{r:.1}" fill="#0B0F18" fill-opacity="0.90" stroke="#FFFFFF" stroke-width="{sw:.1}" stroke-opacity="0.35"/>
<circle cx="{cx:.1}" cy="{cy:.1}" r="{cr:.1}" fill="{a}"/>
{vs}
{lt_el}
{lv_el}
{rt_el}
{rv_el}"##,
        r = 16.0 * u,
        a = c.accent,
        sw = 3.0 * u,
        cr = 44.0 * u,
        vs = c.text(cx, cy + 12.0 * u, 34.0 * u, 900, "#0B0F18", "middle", "", "VS"),
        lt_el = c.text(lx + bw / 2.0, y + 86.0 * u, 30.0 * u, 700, &c.accent, "middle", "", lt),
        lv_el = c.text(lx + bw / 2.0, y + 150.0 * u, 42.0 * u, 800, "#FFFFFF", "middle", "", lv),
        rt_el = c.text(rx + bw / 2.0, y + 86.0 * u, 30.0 * u, 700, "#D9DEE7", "middle", "", rt),
        rv_el = c.text(rx + bw / 2.0, y + 150.0 * u, 42.0 * u, 800, "#FFFFFF", "middle", "", rv),
    )
}

fn breaking_banner(c: &Ctx, badge: &str, title: &str, sub: &str) -> String {
    let u = c.u;
    let x = 84.0 * u;
    let pw = c.w - 168.0 * u;
    let ph = 156.0 * u;
    let y = c.h - c.bottom(110.0) - ph;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#12080A" fill-opacity="0.94" stroke="#E62117" stroke-width="{sw:.1}"/>
<rect x="{bx:.1}" y="{by:.1}" width="{bw:.1}" height="{bh:.1}" rx="{br:.1}" fill="#E62117"/>
{b_el}
{t_el}
{s_el}"##,
        r = 12.0 * u,
        sw = 3.0 * u,
        bx = x + 28.0 * u,
        by = y - 22.0 * u,
        bw = 260.0 * u,
        bh = 44.0 * u,
        br = 6.0 * u,
        b_el = c.text(x + 158.0 * u, y + 8.0 * u, 24.0 * u, 800, "#FFFFFF", "middle", "", badge),
        t_el = c.text(x + 34.0 * u, y + 78.0 * u, 42.0 * u, 800, "#FFFFFF", "start", "", title),
        s_el = c.text(x + 34.0 * u, y + 126.0 * u, 28.0 * u, 500, &c.accent, "start", "", sub),
    )
}

fn price_tag(c: &Ctx, item: &str, price: &str, rating: &str) -> String {
    let u = c.u;
    let pw = 420.0 * u;
    let ph = 168.0 * u;
    let x = c.w - pw - 84.0 * u;
    let y = 78.0 * u;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#0B0E14" fill-opacity="0.90" stroke="{a}" stroke-width="{sw:.1}"/>
{i_el}
{p_el}
{r_el}"##,
        r = 14.0 * u,
        a = c.accent,
        sw = 2.5 * u,
        i_el = c.text(x + 28.0 * u, y + 44.0 * u, 22.0 * u, 700, "#9BA6B8", "start", "", item),
        p_el = c.text(x + 28.0 * u, y + 104.0 * u, 54.0 * u, 900, "#FFFFFF", "start", "", price),
        r_el = c.text(x + 28.0 * u, y + 146.0 * u, 24.0 * u, 700, &c.accent, "start", "", rating),
    )
}

fn neon_title(c: &Ctx, top: &str, main: &str, bottom: &str) -> String {
    let u = c.u;
    let cx = c.w / 2.0;
    let cy = c.h / 2.0;
    format!(
        r##"<rect x="0" y="{by:.1}" width="{w:.1}" height="{bh:.1}" fill="url(#band)"/>
{t_el}
{m_glow}
{m_el}
<rect x="{lx:.1}" y="{ly:.1}" width="{lw:.1}" height="{lh:.1}" fill="url(#accentfade)"/>
{b_el}"##,
        by = cy - 170.0 * u,
        w = c.w,
        bh = 340.0 * u,
        t_el = c.text(cx, cy - 82.0 * u, 28.0 * u, 700, &c.accent, "middle", &format!(r#"letter-spacing="{:.1}""#, 8.0 * u), top),
        m_glow = c.text(cx, cy + 16.0 * u, 92.0 * u, 900, &c.accent, "middle", r#"opacity="0.38""#, main),
        m_el = c.text(cx, cy + 12.0 * u, 90.0 * u, 900, "#FFFFFF", "middle", "", main),
        lx = cx - 260.0 * u,
        ly = cy + 44.0 * u,
        lw = 520.0 * u,
        lh = 3.0 * u,
        b_el = c.text(cx, cy + 92.0 * u, 26.0 * u, 600, "#D9DEE7", "middle", &format!(r#"letter-spacing="{:.1}""#, 5.0 * u), bottom),
    )
}

fn step_callout(c: &Ctx, step: &str, title: &str, detail: &str) -> String {
    let u = c.u;
    let pw = if c.h > c.w { 860.0 * u } else { 640.0 * u };
    let ph = 148.0 * u;
    let x = if c.h > c.w { (c.w - pw) / 2.0 } else { 84.0 * u };
    let y = if c.h > c.w { c.h * 0.13 } else { 78.0 * u };
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="url(#panel)"/>
<rect x="{x:.1}" y="{y:.1}" width="{bw:.1}" height="{ph:.1}" rx="{r:.1}" fill="{a}"/>
{st}
{ti}
{de}"##,
        r = 10.0 * u,
        bw = 136.0 * u,
        a = c.accent,
        st = c.text(x + 68.0 * u, y + 86.0 * u, 30.0 * u, 900, "#0B0E13", "middle", "", step),
        ti = c.text(x + 164.0 * u, y + 66.0 * u, 36.0 * u, 800, "#FFFFFF", "start", "", title),
        de = c.text(x + 164.0 * u, y + 114.0 * u, 24.0 * u, 400, "#D9DEE7", "start", "", detail),
    )
}

fn stat_bar(c: &Ctx, label: &str, percent: &str, note: &str) -> String {
    let u = c.u;
    let pw = if c.h > c.w { 840.0 * u } else { 680.0 * u };
    let ph = 154.0 * u;
    let x = if c.h > c.w { (c.w - pw) / 2.0 } else { 96.0 * u };
    let y = if c.h > c.w { c.h * 0.13 } else { c.h - c.bottom(130.0) - ph };
    let pct_num: f64 = percent.trim_end_matches('%').trim().parse().unwrap_or(85.0);
    let bar_w = (pw - 64.0 * u) * (pct_num.clamp(5.0, 100.0) / 100.0);
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#0B0E14" fill-opacity="0.90"/>
{l_el}
{p_el}
<rect x="{bx:.1}" y="{by:.1}" width="{bw_full:.1}" height="{bh:.1}" rx="{br:.1}" fill="#232A38"/>
<rect x="{bx:.1}" y="{by:.1}" width="{bar_w:.1}" height="{bh:.1}" rx="{br:.1}" fill="{a}"/>
{n_el}"##,
        r = 12.0 * u,
        a = c.accent,
        l_el = c.text(x + 32.0 * u, y + 50.0 * u, 28.0 * u, 700, "#FFFFFF", "start", "", label),
        p_el = c.text(x + pw - 32.0 * u, y + 52.0 * u, 36.0 * u, 900, &c.accent, "end", "", percent),
        bx = x + 32.0 * u,
        by = y + 74.0 * u,
        bw_full = pw - 64.0 * u,
        bh = 18.0 * u,
        br = 9.0 * u,
        n_el = c.text(x + 32.0 * u, y + 130.0 * u, 22.0 * u, 400, "#9BA6B8", "start", "", note),
    )
}

fn social_handle(c: &Ctx, platform: &str, handle: &str) -> String {
    let u = c.u;
    let pw = 420.0 * u;
    let ph = 92.0 * u;
    let x = c.w - pw - 78.0 * u;
    let y = 72.0 * u;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="#0B0E14" fill-opacity="0.88" stroke="{a}" stroke-width="{sw:.1}"/>
{pl}
{hd}"##,
        r = ph / 2.0,
        a = c.accent,
        sw = 2.5 * u,
        pl = c.text(x + 36.0 * u, y + 38.0 * u, 20.0 * u, 700, &c.accent, "start", "", platform),
        hd = c.text(x + 36.0 * u, y + 72.0 * u, 30.0 * u, 800, "#FFFFFF", "start", "", handle),
    )
}

fn minimal_corner(c: &Ctx, title: &str, sub: &str) -> String {
    let u = c.u;
    let x = 78.0 * u;
    let y = 72.0 * u;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{bw:.1}" height="{bh:.1}" fill="{a}"/>
{t}
{s}"##,
        bw = 5.0 * u,
        bh = 62.0 * u,
        a = c.accent,
        t = c.text(x + 20.0 * u, y + 28.0 * u, 26.0 * u, 800, "#FFFFFF", "start", "", title),
        s = c.text(x + 20.0 * u, y + 56.0 * u, 19.0 * u, 600, &c.accent, "start", "", sub),
    )
}

fn top_list_badge(c: &Ctx, rank: &str, title: &str, tag: &str) -> String {
    let u = c.u;
    let x = 84.0 * u;
    let y = 84.0 * u;
    let pw = 620.0 * u;
    let ph = 136.0 * u;
    format!(
        r##"<rect x="{x:.1}" y="{y:.1}" width="{pw:.1}" height="{ph:.1}" rx="{r:.1}" fill="url(#panel)"/>
<rect x="{x:.1}" y="{y:.1}" width="{bw:.1}" height="{ph:.1}" rx="{r:.1}" fill="{a}"/>
{rk}
{tg}
{ti}"##,
        r = 12.0 * u,
        bw = 136.0 * u,
        a = c.accent,
        rk = c.text(x + 68.0 * u, y + 88.0 * u, 56.0 * u, 900, "#0B0E13", "middle", "", rank),
        tg = c.text(x + 162.0 * u, y + 50.0 * u, 21.0 * u, 700, &c.accent, "start", "", tag),
        ti = c.text(x + 162.0 * u, y + 98.0 * u, 36.0 * u, 800, "#FFFFFF", "start", "", title),
    )
}

fn cinematic_letterbox(c: &Ctx, caption: &str, credit: &str) -> String {
    let u = c.u;
    let bar = (c.h * 0.11).max(88.0 * u);
    let by = c.h - bar;
    format!(
        r##"<rect x="0" y="0" width="{w:.1}" height="{bar:.1}" fill="#000000"/>
<rect x="0" y="{by:.1}" width="{w:.1}" height="{bar:.1}" fill="#000000"/>
{cr}
{cp}"##,
        w = c.w,
        cr = c.text(c.w - 64.0 * u, bar * 0.62, 20.0 * u, 600, &c.accent, "end", "", credit),
        cp = c.text(c.w / 2.0, by + bar * 0.58, 32.0 * u, 500, "#F3F5F8", "middle", r#"font-style="italic""#, caption),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_renders() {
        for t in templates() {
            let g = GraphicSpec { template: t.id.into(), ..Default::default() };
            for (w, h) in [(1920, 1080), (1080, 1920)] {
                let svg = render_svg(&g, w, h).unwrap();
                assert!(svg.contains("<svg") && svg.contains("</svg>"), "{}", t.id);
            }
        }
    }

    #[test]
    fn escapes_text() {
        let mut g = GraphicSpec { template: "lower_third".into(), ..Default::default() };
        g.fields.insert("name".into(), "A & <B>".into());
        assert!(render_svg(&g, 1920, 1080).unwrap().contains("A &amp; &lt;B&gt;"));
    }
}
