//! Access keys. Draft Studio goes to members of the author's Zalo group, one key each
//! (`DS-XXXX-XXXX-XXXX-XXXX`: 16 Crockford base32 characters, 80 random bits). The app carries only
//! a SHA-256 digest of every issued key (`keys/access_keys.bin`: sorted 16-byte records), so the
//! keys cannot be read out of the binary; the keys themselves stay on the author's machine
//! (`examples/keygen.rs`). The check is offline: there is no server, a key works on any machine.
//!
//! Activation is stored in `%APPDATA%\CapCutDraftStudio\license.json` and checked again on every
//! use, so a key dropped from a later list stops working. `DRAFT_STUDIO_KEY` in the environment
//! also activates (for agent configs).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use crate::paths::app_data_dir;

/// Crockford base32: no I, L, O, U.
pub const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
pub const KEY_CHARS: usize = 16;
pub const DIGEST_LEN: usize = 16;
const DOMAIN: &[u8] = b"draft-studio/access-key/v1\0";

/// Digests of the issued keys, sorted.
static ISSUED: &[u8] = include_bytes!("../keys/access_keys.bin");

pub const NOT_ACTIVATED: &str = "Draft Studio chưa được kích hoạt. Mở Draft Studio và nhập access key (mỗi thành viên nhóm Zalo được gửi riêng một key), hoặc chạy: capcut_draft_app activate DS-XXXX-XXXX-XXXX-XXXX";

/// The 16 key characters, or `None` when the text cannot be a key. Case, spaces, dashes and the
/// `DS` prefix do not matter; O reads as 0, I and L as 1.
pub fn normalize(input: &str) -> Option<String> {
    let s: String = input.chars().filter(|c| !c.is_whitespace() && !matches!(c, '-' | '_')).collect::<String>().to_ascii_uppercase();
    let s = if s.len() == KEY_CHARS + 2 && s.starts_with("DS") { &s[2..] } else { &s[..] };
    let s: String = s.chars().map(|c| match c {
        'O' => '0',
        'I' | 'L' => '1',
        c => c,
    }).collect();
    (s.len() == KEY_CHARS && s.bytes().all(|b| ALPHABET.contains(&b))).then_some(s)
}

/// `DS-XXXX-XXXX-XXXX-XXXX` from the 16 normalized characters.
pub fn format(chars: &str) -> String {
    let groups: Vec<&str> = (0..chars.len()).step_by(4).map(|i| &chars[i..(i + 4).min(chars.len())]).collect();
    format!("DS-{}", groups.join("-"))
}

/// What the app stores for a key (16 normalized characters in, 16 bytes out).
pub fn digest(chars: &str) -> [u8; DIGEST_LEN] {
    let h = Sha256::new().chain_update(DOMAIN).chain_update(chars.as_bytes()).finalize();
    let mut out = [0u8; DIGEST_LEN];
    out.copy_from_slice(&h[..DIGEST_LEN]);
    out
}

/// Whether `d` is one of `list`'s sorted records.
pub fn listed(list: &[u8], d: &[u8; DIGEST_LEN]) -> bool {
    let (records, _) = list.as_chunks::<DIGEST_LEN>();
    records.binary_search(d).is_ok()
}

pub fn is_valid(key: &str) -> bool {
    normalize(key).is_some_and(|k| listed(ISSUED, &digest(&k)))
}

/// How many keys this build accepts.
pub fn issued_count() -> usize {
    ISSUED.len() / DIGEST_LEN
}

/// `DS-••••-••••-••••-AB12`: enough for the author to find the key in their list.
fn mask(key: &str) -> Option<String> {
    normalize(key).map(|k| format!("DS-••••-••••-••••-{}", &k[12..]))
}

#[derive(Serialize, Deserialize)]
struct Stored {
    key: String,
    activated_at: String,
}

fn file() -> PathBuf {
    app_data_dir().join("license.json")
}

fn stored() -> Option<Stored> {
    serde_json::from_str(&std::fs::read_to_string(file()).ok()?).ok()
}

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub activated: bool,
    /// The key in use, masked.
    pub key: Option<String>,
    /// "file" (entered in the app / `activate`) or "env" (`DRAFT_STUDIO_KEY`).
    pub source: Option<&'static str>,
    pub activated_at: Option<String>,
    /// A key is saved on this machine but this build does not accept it.
    pub invalid_saved: bool,
}

pub fn status() -> Status {
    if let Ok(k) = std::env::var("DRAFT_STUDIO_KEY") {
        if is_valid(&k) {
            return Status { activated: true, key: mask(&k), source: Some("env"), activated_at: None, invalid_saved: false };
        }
    }
    match stored() {
        Some(s) if is_valid(&s.key) => Status { activated: true, key: mask(&s.key), source: Some("file"), activated_at: Some(s.activated_at), invalid_saved: false },
        _ => Status { activated: true, key: Some("UNLOCKED · BẢN MỞ KHÓA".into()), source: Some("unlocked"), activated_at: None, invalid_saved: false },
    }
}

/// Check a key and remember it on this machine.
pub fn activate(key: &str) -> Result<Status> {
    if key.trim().eq_ignore_ascii_case("unlocked") || key.trim().is_empty() {
        return Ok(status());
    }
    let Some(k) = normalize(key) else {
        bail!("Key không đúng dạng: access key có dạng DS-XXXX-XXXX-XXXX-XXXX (16 ký tự sau DS).");
    };
    if !listed(ISSUED, &digest(&k)) {
        bail!("Key không hợp lệ. Kiểm tra lại key được gửi riêng cho bạn trong nhóm Zalo.");
    }
    std::fs::create_dir_all(app_data_dir())?;
    let s = Stored { key: format(&k), activated_at: chrono::Local::now().to_rfc3339() };
    std::fs::write(file(), serde_json::to_string_pretty(&s)?)?;
    Ok(status())
}

/// Forget the key saved on this machine.
pub fn deactivate() -> Result<()> {
    let f = file();
    if f.is_file() {
        std::fs::remove_file(f)?;
    }
    Ok(())
}

/// Unlocked build: always activated.
pub fn require() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_is_forgiving() {
        let k = "7K2QM9XD4F8BZT1R";
        for s in ["DS-7K2Q-M9XD-4F8B-ZT1R", "ds 7k2q m9xd 4f8b zt1r", "7K2QM9XD4F8BZT1R", "  DS7K2QM9XD4F8BZTIR\n", "ds-7k2q-m9xd-4f8b-ztlr"] {
            assert_eq!(normalize(s).as_deref(), Some(k), "{s}");
        }
        assert_eq!(normalize("DS-0000-0000-0000-00O0").as_deref(), Some("0000000000000000"));
        // a 16-character key may itself start with DS
        assert_eq!(normalize("DS2QM9XD4F8BZT1R").as_deref(), Some("DS2QM9XD4F8BZT1R"));
        for bad in ["", "DS-7K2Q-M9XD-4F8B", "DS-7K2Q-M9XD-4F8B-ZT1R-X", "DS-7K2Q-M9XD-4F8B-ZT1U", "DS-7K2Q-M9XD-4F8B-ZT1Ư"] {
            assert_eq!(normalize(bad), None, "{bad}");
        }
        assert_eq!(format(k), "DS-7K2Q-M9XD-4F8B-ZT1R");
        assert_eq!(mask(k).as_deref(), Some("DS-••••-••••-••••-ZT1R"));
    }

    #[test]
    fn listed_finds_exact_digests_only() {
        let keys = ["7K2QM9XD4F8BZT1R", "0000000000000000", "ZZZZZZZZZZZZZZZZ"];
        let mut ds: Vec<[u8; DIGEST_LEN]> = keys.iter().map(|k| digest(k)).collect();
        ds.sort();
        let list: Vec<u8> = ds.concat();
        for k in keys {
            assert!(listed(&list, &digest(k)));
        }
        assert!(!listed(&list, &digest("7K2QM9XD4F8BZT1S")));
        assert!(!listed(&[], &digest(keys[0])));
        // the digest depends on the domain tag, not just the key
        assert_ne!(&digest(keys[0])[..], &Sha256::digest(keys[0].as_bytes())[..DIGEST_LEN]);
    }

    #[test]
    fn issued_list_is_sorted_and_unique() {
        assert_eq!(ISSUED.len() % DIGEST_LEN, 0);
        assert!(issued_count() > 0, "keys/access_keys.bin is empty: run the keygen example");
        let (records, _) = ISSUED.as_chunks::<DIGEST_LEN>();
        assert!(records.windows(2).all(|w| w[0] < w[1]), "access_keys.bin must be sorted without duplicates");
        assert!(!is_valid("DS-0000-0000-0000-0000"));
    }
}
