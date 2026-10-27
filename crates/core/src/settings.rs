use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::paths::{app_data_dir, exe_dir};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Explicit CLI paths; empty = auto-detect.
    #[serde(default)]
    pub vectorcraft_cli: Option<String>,
    #[serde(default)]
    pub effectcraft_cli: Option<String>,
    #[serde(default)]
    pub filmcraft_cli: Option<String>,
    /// Folder where the three craft repositories were cloned and built (searched after the
    /// explicit paths, `crafts/` next to the app and PATH).
    #[serde(default)]
    pub craft_engines_dir: Option<String>,
    /// Where generated graphics / motion / previews go. CapCut links these files, so keep them.
    #[serde(default)]
    pub assets_dir: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    /// When an agent starts a build over MCP: start the desktop studio if it is closed, bring
    /// its window forward if it is open (`activity.rs`).
    #[serde(default = "default_true")]
    pub open_studio_on_agent_build: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            vectorcraft_cli: None,
            effectcraft_cli: None,
            filmcraft_cli: None,
            craft_engines_dir: None,
            assets_dir: None,
            port: None,
            open_studio_on_agent_build: true,
        }
    }
}

impl Settings {
    pub fn file() -> PathBuf {
        app_data_dir().join("settings.json")
    }

    pub fn load() -> Settings {
        std::fs::read_to_string(Self::file()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(app_data_dir())?;
        std::fs::write(Self::file(), serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn assets_root(&self) -> PathBuf {
        match &self.assets_dir {
            Some(d) if !d.trim().is_empty() => PathBuf::from(d),
            _ => exe_dir().join("studio_assets"),
        }
    }

    pub fn port(&self) -> u16 {
        std::env::var("CAPCUT_STUDIO_PORT").ok().and_then(|p| p.parse().ok()).or(self.port).unwrap_or(8797)
    }
}
