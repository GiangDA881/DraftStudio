//! Draft Studio core: writes CapCut PC drafts and drives the three craft engines
//! (VectorCraft graphics, EffectCraft motion, FilmCraft probe + preview) as external CLIs.
//! The CLI/MCP binary and the desktop studio are thin front ends over this crate.

#![recursion_limit = "256"]

pub mod activity;
pub mod build;
pub mod crafts;
pub mod draft;
pub mod graphics;
pub mod library;
pub mod license;
pub mod mcp;
pub mod media;
pub mod paths;
pub mod settings;
pub mod spec;

pub use build::{build_project, BuildLog, BuildReport};
pub use spec::ProjectSpec;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fresh upper-case UUID, the id style CapCut uses everywhere.
pub fn new_id() -> String {
    uuid::Uuid::new_v4().to_string().to_uppercase()
}

pub fn sec_to_us(s: f64) -> u64 {
    if s.is_finite() && s > 0.0 {
        (s * 1_000_000.0).round() as u64
    } else {
        0
    }
}
