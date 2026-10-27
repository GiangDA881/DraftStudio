//! Bridges to the three craft engines, run as their own headless CLIs:
//! VectorCraft draws graphics, EffectCraft animates them into alpha video, FilmCraft probes media
//! and renders a preview of the cut. Nothing here imitates an engine: when one is missing the
//! call fails and says how to build it.

pub mod effect;
pub mod film;
pub mod vector;

use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::paths::exe_dir;
use crate::settings::Settings;

/// Where the craft repositories were cloned (their shared build output lives in `_target`):
/// `craft_engines_dir` in the settings, else the `CRAFT_ENGINES_DIR` environment variable.
pub fn engines_dir(s: &Settings) -> Option<PathBuf> {
    s.craft_engines_dir.clone().or_else(|| std::env::var("CRAFT_ENGINES_DIR").ok()).filter(|d| !d.trim().is_empty()).map(PathBuf::from)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Craft {
    Vector,
    Effect,
    Film,
}

impl Craft {
    pub const ALL: [Craft; 3] = [Craft::Vector, Craft::Effect, Craft::Film];

    pub fn id(self) -> &'static str {
        match self {
            Craft::Vector => "vectorcraft",
            Craft::Effect => "effectcraft",
            Craft::Film => "filmcraft",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Craft::Vector => "VectorCraft",
            Craft::Effect => "EffectCraft",
            Craft::Film => "FilmCraft",
        }
    }

    pub fn role(self) -> &'static str {
        match self {
            Craft::Vector => "Vẽ title, lower-third, badge, quote card → PNG trong suốt",
            Craft::Effect => "Làm chuyển động cho đồ họa → video alpha (ProRes 4444 / WebM)",
            Craft::Film => "Đo media thuần Rust + render bản xem trước MP4 của cả timeline",
        }
    }

    fn exe_name(self) -> String {
        format!("{}-cli{}", self.id(), std::env::consts::EXE_SUFFIX)
    }

    fn configured(self, s: &Settings) -> Option<String> {
        match self {
            Craft::Vector => s.vectorcraft_cli.clone(),
            Craft::Effect => s.effectcraft_cli.clone(),
            Craft::Film => s.filmcraft_cli.clone(),
        }
    }

    /// Candidate locations, first match wins: settings, `<ID>_CLI` env var, `crafts/` next to this
    /// app, next to this app, PATH, then the craft engines dir (shared build dir, each
    /// repository's own target dir).
    pub fn candidates(self) -> Vec<PathBuf> {
        let s = Settings::load();
        let mut v = vec![];
        if let Some(p) = self.configured(&s).filter(|p| !p.trim().is_empty()) {
            v.push(PathBuf::from(p));
        }
        if let Ok(p) = std::env::var(format!("{}_CLI", self.id().to_uppercase())) {
            v.push(PathBuf::from(p));
        }
        let exe = self.exe_name();
        v.push(exe_dir().join("crafts").join(&exe));
        v.push(exe_dir().join(&exe));
        if let Some(path) = std::env::var_os("PATH") {
            v.extend(std::env::split_paths(&path).map(|d| d.join(&exe)));
        }
        if let Some(root) = engines_dir(&s) {
            for t in ["_target/release", "_target/x86_64-pc-windows-gnu/release"] {
                v.push(root.join(t).join(&exe));
            }
            for t in ["target/release", "target/x86_64-pc-windows-gnu/release"] {
                v.push(root.join(self.id()).join(t).join(&exe));
            }
        }
        v
    }

    pub fn locate(self) -> Option<PathBuf> {
        self.candidates().into_iter().find(|p| p.is_file())
    }

    pub fn require(self) -> Result<PathBuf> {
        self.locate().ok_or_else(|| {
            anyhow!(
                "Không tìm thấy {} ({}). Đặt file vào thư mục crafts/ cạnh app hoặc vào PATH, đặt biến môi trường {}_CLI, hoặc chỉ đường dẫn / thư mục craft_engines trong Cài đặt. Mã nguồn: https://github.com/storytold/{}",
                self.name(),
                self.exe_name(),
                self.id().to_uppercase(),
                self.id()
            )
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CraftStatus {
    pub id: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub path: Option<String>,
    pub version: Option<String>,
    pub ready: bool,
}

pub fn status() -> Vec<CraftStatus> {
    Craft::ALL
        .iter()
        .map(|&c| {
            let path = c.locate();
            let version = path.as_ref().and_then(|p| version_of(c, p));
            CraftStatus {
                id: c.id(),
                name: c.name(),
                role: c.role(),
                ready: path.is_some(),
                path: path.map(|p| crate::paths::clean_path_str(&p.to_string_lossy())),
                version,
            }
        })
        .collect()
}

fn version_of(c: Craft, exe: &Path) -> Option<String> {
    let out = run(exe, &["--version"], None, Duration::from_secs(10)).ok()?;
    let s = out.stdout.trim().to_string();
    if !s.is_empty() {
        return Some(s.lines().next().unwrap_or("").to_string());
    }
    // effectcraft-cli has no --version; `info --json` carries it.
    if c == Craft::Effect {
        let out = run(exe, &["info", "--json", "--empty"], None, Duration::from_secs(20)).ok()?;
        let v: serde_json::Value = serde_json::from_str(&out.stdout).ok()?;
        return v["version"].as_str().map(|s| format!("effectcraft-cli {s}"));
    }
    None
}

/// A `Command` that never flashes a console window from the desktop app.
pub fn command<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
    let mut c = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    c
}

pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
}

/// Run a craft CLI, fail on a non-zero exit (with its stderr) or after `timeout`.
pub fn run(exe: &Path, args: &[&str], cwd: Option<&Path>, timeout: Duration) -> Result<RunOutput> {
    let mut cmd = command(exe);
    cmd.args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(d) = cwd {
        cmd.current_dir(d);
    }
    let mut child = cmd.spawn().map_err(|e| anyhow!("không chạy được {}: {e}", exe.display()))?;
    let mut out = child.stdout.take();
    let mut err = child.stderr.take();
    let t_out = std::thread::spawn(move || {
        let mut s = Vec::new();
        if let Some(o) = out.as_mut() {
            let _ = o.read_to_end(&mut s);
        }
        s
    });
    let t_err = std::thread::spawn(move || {
        let mut s = Vec::new();
        if let Some(e) = err.as_mut() {
            let _ = e.read_to_end(&mut s);
        }
        s
    });
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break s;
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            bail!("{} quá thời gian ({} s)", exe.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), timeout.as_secs());
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let stdout = String::from_utf8_lossy(&t_out.join().unwrap_or_default()).to_string();
    let stderr = String::from_utf8_lossy(&t_err.join().unwrap_or_default()).to_string();
    if !status.success() {
        let msg = if stderr.trim().is_empty() { stdout.trim() } else { stderr.trim() };
        let tail: String = msg.chars().rev().take(1200).collect::<Vec<_>>().into_iter().rev().collect();
        bail!("{} lỗi (mã {}): {}", exe.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), status.code().unwrap_or(-1), tail);
    }
    Ok(RunOutput { stdout, stderr })
}
