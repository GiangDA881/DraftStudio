//! VectorCraft: the graphic's SVG is opened by `vectorcraft-cli convert` and exported by
//! VectorCraft's own renderer to a transparent PNG, plus a `.vectorcraft` document the user can
//! open in VectorCraft to keep editing the design.

use anyhow::{bail, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::{run, Craft};
use crate::graphics::render_svg;
use crate::paths::clean_path_str;
use crate::spec::GraphicSpec;

#[derive(Debug, Clone, Serialize)]
pub struct GraphicOutput {
    pub png: String,
    pub svg: String,
    pub vectorcraft_doc: Option<String>,
    pub cached: bool,
}

/// Stable short hash of a text (FNV-1a), used to reuse renders of identical graphics.
pub fn short_hash(s: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:010x}", h & 0xff_ffff_ffff)
}

pub fn render(g: &GraphicSpec, w: u32, h: u32, out_dir: &Path) -> Result<GraphicOutput> {
    let svg = render_svg(g, w, h)?;
    std::fs::create_dir_all(out_dir)?;
    let stem = format!("{}-{}", g.template, short_hash(&svg));
    let svg_path = out_dir.join(format!("{stem}.svg"));
    let png_path = out_dir.join(format!("{stem}.png"));
    let doc_path = out_dir.join(format!("{stem}.vectorcraft"));
    let s = |p: &PathBuf| clean_path_str(&p.to_string_lossy());
    if png_path.is_file() {
        return Ok(GraphicOutput {
            png: s(&png_path),
            svg: s(&svg_path),
            vectorcraft_doc: doc_path.is_file().then(|| s(&doc_path)),
            cached: true,
        });
    }
    let exe = Craft::Vector.require()?;
    std::fs::write(&svg_path, &svg)?;
    let (svg_a, png_a) = (svg_path.to_string_lossy().to_string(), png_path.to_string_lossy().to_string());
    run(&exe, &["convert", &svg_a, &png_a], None, Duration::from_secs(120))?;
    if !png_path.is_file() {
        bail!("VectorCraft không xuất ra {}", png_path.display());
    }
    let doc_a = doc_path.to_string_lossy().to_string();
    let doc = run(&exe, &["convert", &svg_a, &doc_a], None, Duration::from_secs(60)).ok().filter(|_| doc_path.is_file()).map(|_| s(&doc_path));
    Ok(GraphicOutput { png: s(&png_path), svg: s(&svg_path), vectorcraft_doc: doc, cached: false })
}
