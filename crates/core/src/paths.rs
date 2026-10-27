use std::path::{Path, PathBuf};

/// CapCut PC's draft folder (`com.lveditor.draft`).
pub fn capcut_root() -> PathBuf {
    if let Ok(p) = std::env::var("CAPCUT_DRAFT_ROOT") {
        return PathBuf::from(p);
    }
    #[cfg(target_os = "macos")]
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join("Movies/CapCut/User Data/Projects/com.lveditor.draft");
    }
    let local = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("USERPROFILE").map(|u| PathBuf::from(u).join("AppData").join("Local")))
        .unwrap_or_else(|_| PathBuf::from("."));
    local.join("CapCut").join("User Data").join("Projects").join("com.lveditor.draft")
}

/// Where the studio keeps its settings.
pub fn app_data_dir() -> PathBuf {
    let base = std::env::var("APPDATA").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));
    base.join("CapCutDraftStudio")
}

/// Folder of the running executable.
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Absolute path with forward slashes and without the `\\?\` / `//?/` long-path prefix:
/// CapCut on Windows crashes or reports "media not found" on prefixed paths.
pub fn capcut_path<P: AsRef<Path>>(p: P) -> String {
    let raw = p.as_ref();
    let abs = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        std::env::current_dir().map(|c| c.join(raw)).unwrap_or_else(|_| raw.to_path_buf())
    };
    clean_path_str(&abs.to_string_lossy())
}

pub fn clean_path_str(s: &str) -> String {
    s.replace(r"\\?\", "").replace("//?/", "").replace('\\', "/")
}

/// File-system safe slug for asset folders.
pub fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        let c = fold_vietnamese(c);
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    let t = out.trim_matches('-').to_string();
    if t.is_empty() {
        "project".into()
    } else {
        t.chars().take(48).collect()
    }
}

fn fold_vietnamese(c: char) -> char {
    const MAP: &[(&str, char)] = &[
        ("àáạảãâầấậẩẫăằắặẳẵ", 'a'),
        ("ÀÁẠẢÃÂẦẤẬẨẪĂẰẮẶẲẴ", 'A'),
        ("èéẹẻẽêềếệểễ", 'e'),
        ("ÈÉẸẺẼÊỀẾỆỂỄ", 'E'),
        ("ìíịỉĩ", 'i'),
        ("ÌÍỊỈĨ", 'I'),
        ("òóọỏõôồốộổỗơờớợởỡ", 'o'),
        ("ÒÓỌỎÕÔỒỐỘỔỖƠỜỚỢỞỠ", 'O'),
        ("ùúụủũưừứựửữ", 'u'),
        ("ÙÚỤỦŨƯỪỨỰỬỮ", 'U'),
        ("ỳýỵỷỹ", 'y'),
        ("ỲÝỴỶỸ", 'Y'),
        ("đ", 'd'),
        ("Đ", 'D'),
        ("äÄ", 'a'),
        ("öÖ", 'o'),
        ("üÜ", 'u'),
        ("ß", 's'),
    ];
    for (set, to) in MAP {
        if set.contains(c) {
            return *to;
        }
    }
    c
}

/// True when CapCut PC is running. CapCut rewrites `root_meta_info.json` when it exits, so a
/// draft registered while it runs may only show up after a restart of CapCut.
pub fn capcut_running() -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let out = std::process::Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq CapCut.exe", "/NH"])
            .creation_flags(0x0800_0000)
            .output();
        if let Ok(o) = out {
            return String::from_utf8_lossy(&o.stdout).to_lowercase().contains("capcut.exe");
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_long_path_prefix() {
        assert_eq!(clean_path_str(r"\\?\E:\a\b.png"), "E:/a/b.png");
        assert_eq!(clean_path_str("//?/E:/a/b.png"), "E:/a/b.png");
    }

    #[test]
    fn slug_folds_vietnamese() {
        assert_eq!(slug("Seneca — Cái giá của quyền lực"), "seneca-cai-gia-cua-quyen-luc");
    }
}
