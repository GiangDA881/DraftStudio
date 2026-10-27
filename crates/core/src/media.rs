//! Media facts the draft needs: kind, duration, frame size. Durations come from FilmCraft's
//! pure-Rust prober when it is built, else from ffprobe.

use serde::Serialize;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    Image,
    Video,
    Audio,
    Other,
}

pub fn kind_of<P: AsRef<Path>>(p: P) -> MediaKind {
    let ext = p.as_ref().extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "bmp" | "gif" | "tif" | "tiff" | "heic" => MediaKind::Image,
        "mp4" | "mov" | "mkv" | "webm" | "avi" | "flv" | "ts" | "m4v" | "mxf" => MediaKind::Video,
        "mp3" | "wav" | "m4a" | "aac" | "flac" | "ogg" | "opus" | "aiff" | "wma" => MediaKind::Audio,
        _ => MediaKind::Other,
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MediaInfo {
    pub duration_us: u64,
    pub width: u32,
    pub height: u32,
    pub has_audio: bool,
    /// "filmcraft" | "ffprobe" | "header" | "none"
    pub source: &'static str,
}

/// Duration and size of a media file. Images report their pixel size from the file header.
pub fn probe<P: AsRef<Path>>(p: P) -> MediaInfo {
    let p = p.as_ref();
    if !p.exists() {
        return MediaInfo { source: "none", ..Default::default() };
    }
    if kind_of(p) == MediaKind::Image {
        let (width, height) = image_size(p).unwrap_or((0, 0));
        return MediaInfo { width, height, source: "header", ..Default::default() };
    }
    if let Some(info) = crate::crafts::film::probe(p) {
        if info.duration_us > 0 {
            return info;
        }
    }
    ffprobe(p).unwrap_or(MediaInfo { source: "none", ..Default::default() })
}

pub fn duration_us<P: AsRef<Path>>(p: P) -> u64 {
    probe(p).duration_us
}

fn ffprobe(p: &Path) -> Option<MediaInfo> {
    let out = crate::crafts::command("ffprobe")
        .args(["-v", "error", "-show_entries", "format=duration:stream=codec_type,width,height", "-of", "json"])
        .arg(p)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
    let dur = v["format"]["duration"].as_str().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let mut info = MediaInfo { duration_us: crate::sec_to_us(dur), source: "ffprobe", ..Default::default() };
    for s in v["streams"].as_array().into_iter().flatten() {
        match s["codec_type"].as_str() {
            Some("video") if info.width == 0 => {
                info.width = s["width"].as_u64().unwrap_or(0) as u32;
                info.height = s["height"].as_u64().unwrap_or(0) as u32;
            }
            Some("audio") => info.has_audio = true,
            _ => {}
        }
    }
    Some(info)
}

/// Pixel size from a PNG / JPEG / WebP / GIF / BMP header, without decoding the image.
pub fn image_size(p: &Path) -> Option<(u32, u32)> {
    let mut f = std::fs::File::open(p).ok()?;
    let mut buf = vec![0u8; 256 * 1024];
    let n = f.read(&mut buf).ok()?;
    let b = buf.get(..n)?;
    let be16 = |i: usize| -> Option<u32> { Some(u16::from_be_bytes([*b.get(i)?, *b.get(i + 1)?]) as u32) };
    let be32 = |i: usize| -> Option<u32> { Some(u32::from_be_bytes([*b.get(i)?, *b.get(i + 1)?, *b.get(i + 2)?, *b.get(i + 3)?])) };
    let le16 = |i: usize| -> Option<u32> { Some(u16::from_le_bytes([*b.get(i)?, *b.get(i + 1)?]) as u32) };
    let le24 = |i: usize| -> Option<u32> { Some(u32::from_le_bytes([*b.get(i)?, *b.get(i + 1)?, *b.get(i + 2)?, 0])) };
    let le32 = |i: usize| -> Option<u32> { Some(u32::from_le_bytes([*b.get(i)?, *b.get(i + 1)?, *b.get(i + 2)?, *b.get(i + 3)?])) };
    if b.starts_with(b"\x89PNG") {
        return Some((be32(16)?, be32(20)?));
    }
    if b.starts_with(b"GIF8") {
        return Some((le16(6)?, le16(8)?));
    }
    if b.starts_with(b"BM") {
        return Some((le32(18)?, le32(22)?.min(i32::MAX as u32)));
    }
    if b.len() > 30 && &b[0..4] == b"RIFF" && &b[8..12] == b"WEBP" {
        return match &b[12..16] {
            b"VP8 " => Some((le16(26)? & 0x3fff, le16(28)? & 0x3fff)),
            b"VP8L" => {
                let bits = le32(21)?;
                Some(((bits & 0x3fff) + 1, ((bits >> 14) & 0x3fff) + 1))
            }
            b"VP8X" => Some((le24(24)? + 1, le24(27)? + 1)),
            _ => None,
        };
    }
    if b.starts_with(&[0xFF, 0xD8]) {
        let mut i = 2;
        while i + 9 < b.len() {
            if b[i] != 0xFF {
                i += 1;
                continue;
            }
            let marker = b[i + 1];
            if marker == 0xD8 || marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
                i += 2;
                continue;
            }
            let len = be16(i + 2)? as usize;
            if matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF) {
                return Some((be16(i + 7)?, be16(i + 5)?));
            }
            i += 2 + len;
        }
    }
    None
}
