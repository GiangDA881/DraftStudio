//! Make Draft Studio access keys. For the author only: the keys never go into the app or the repo.
//!
//!   cargo run -p capcut-core --example keygen -- --count 10000 --out E:/DraftStudio_Keys
//!   cargo run -p capcut-core --example keygen -- --count 500 --out E:/DraftStudio_Keys --append
//!
//! Writes the new keys to `<out>/draft_studio_keys_<date-time>.csv` (to hand out; keep a backup:
//! a lost key cannot be recovered) and their digests to `crates/core/keys/access_keys.bin`, which
//! the next build of the app accepts. `--append` keeps every key already issued; without it an
//! existing list is left alone, because replacing it would turn off every key handed out so far.

use capcut_core::license::{digest, format, ALPHABET, DIGEST_LEN, KEY_CHARS};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    if let Err(e) = run() {
        eprintln!("keygen: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let count: usize = opt("--count").map(|c| c.parse().map_err(|_| format!("--count {c}?"))).transpose()?.unwrap_or(10_000);
    let out = PathBuf::from(opt("--out").ok_or("cần --out <thư mục lưu key> (ngoài thư mục mã nguồn)")?);
    let append = args.iter().any(|a| a == "--append");
    let list_path = opt("--list").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("keys").join("access_keys.bin"));

    let old = std::fs::read(&list_path).unwrap_or_default();
    if old.len() % DIGEST_LEN != 0 {
        return Err(format!("{} hỏng (độ dài {} không chia hết cho {DIGEST_LEN})", list_path.display(), old.len()));
    }
    if !old.is_empty() && !append {
        return Err(format!(
            "{} đã có {} key. Thêm key mới: chạy lại với --append (key cũ vẫn dùng được). Tạo lại từ đầu sẽ vô hiệu hóa mọi key đã gửi: tự xoá file đó trước nếu thật sự muốn vậy.",
            list_path.display(),
            old.len() / DIGEST_LEN
        ));
    }
    let mut digests: BTreeSet<[u8; DIGEST_LEN]> = old.as_chunks::<DIGEST_LEN>().0.iter().copied().collect();
    let first = digests.len() + 1;

    let mut keys = Vec::with_capacity(count);
    while keys.len() < count {
        let mut raw = [0u8; 10]; // 80 bits = 16 base32 characters
        getrandom::fill(&mut raw).map_err(|e| format!("không lấy được số ngẫu nhiên: {e}"))?;
        let mut v = raw.iter().fold(0u128, |acc, b| (acc << 8) | *b as u128);
        let mut chars = [0u8; KEY_CHARS];
        for c in chars.iter_mut().rev() {
            *c = ALPHABET[(v & 31) as usize];
            v >>= 5;
        }
        let chars = String::from_utf8(chars.to_vec()).map_err(|e| e.to_string())?;
        if digests.insert(digest(&chars)) {
            keys.push(format(&chars));
        }
    }

    std::fs::create_dir_all(&out).map_err(|e| format!("không tạo được {}: {e}", out.display()))?;
    let csv = out.join(format!("draft_studio_keys_{}.csv", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    let mut f = std::fs::File::create_new(&csv).map_err(|e| format!("không tạo được {}: {e}", csv.display()))?;
    // BOM so Excel reads the Vietnamese headers as UTF-8
    let mut text = String::from("\u{feff}STT,Key,Người nhận,Ngày gửi,Ghi chú\r\n");
    for (i, k) in keys.iter().enumerate() {
        text.push_str(&format!("{},{k},,,\r\n", first + i));
    }
    f.write_all(text.as_bytes()).map_err(|e| e.to_string())?;

    if let Some(dir) = list_path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let bin: Vec<u8> = digests.iter().flatten().copied().collect();
    std::fs::write(&list_path, bin).map_err(|e| format!("không ghi được {}: {e}", list_path.display()))?;

    println!("{} key mới (số {} – {}) → {}", keys.len(), first, first + keys.len() - 1, csv.display());
    println!("{} key được chấp nhận → {}", digests.len(), list_path.display());
    println!("Build lại app để key mới có hiệu lực. Giữ file CSV ở máy bạn và sao lưu nó: key không khôi phục được từ app.");
    Ok(())
}
