# Third-party notices

Draft Studio itself is MIT licensed (see [LICENSE](LICENSE)). It is not affiliated with,
endorsed by or sponsored by CapCut or ByteDance; "CapCut" is their trademark and is used only to
say which editor the drafts are written for.

## Tools it runs (not part of this repository)

| Tool | License | Source |
|---|---|---|
| VectorCraft (`vectorcraft-cli`) | MIT OR Apache-2.0 | https://github.com/storytold/vectorcraft |
| EffectCraft (`effectcraft-cli`) | MIT OR Apache-2.0 | https://github.com/storytold/effectcraft |
| FilmCraft (`filmcraft-cli`) | MIT OR Apache-2.0 | https://github.com/storytold/filmcraft |
| FFmpeg / ffprobe (optional: cover frames, probing fallback) | LGPL/GPL, per build | https://ffmpeg.org |

The release zip bundles the three craft CLIs in `crafts/`, unmodified apart from stripped debug
symbols. Each one's license (MIT OR Apache-2.0), NOTICE, asset list and the licenses of the fonts and
icons built into it (SIL OFL 1.1, ISC) are in `crafts/licenses/<name>/`; their source is at the URLs
above. Keep those files if you redistribute the CLIs. FFmpeg is never bundled: users install it
themselves.

## Files shipped with GNU-toolchain builds

`WebView2Loader.dll` is Microsoft's WebView2 loader, redistributed under the Microsoft WebView2
SDK license (https://www.nuget.org/packages/Microsoft.Web.WebView2). It is not covered by this
project's MIT license. MSVC builds link the loader statically and do not need the DLL.

## Rust crates compiled into the binaries

297 crates, generated with `cargo metadata` for the Windows targets. All are permissively
licensed (MIT, Apache-2.0, BSD, Zlib, ISC, Unicode, Unlicense, BSL-1.0) except the following, which
are MPL-2.0 (file-level copyleft; used unmodified, source on crates.io): cssparser, cssparser-macros, dtoa-short, option-ext, selectors.

### MIT OR Apache-2.0 (146)

anyhow 1.0.104, async-trait 0.1.92, base64 0.22.1, base64 0.23.1, bitflags 2.13.2, block-buffer 0.10.4, block-buffer 0.12.1, camino 1.2.6, cargo-platform 0.1.9, cc 1.6.0, cfg-if 1.0.5, chrono 0.4.45, cookie 0.18.2, cpufeatures 0.2.17, cpufeatures 0.3.1, crc32fast 1.5.2, crossbeam-channel 0.5.17, crossbeam-utils 0.8.23, crypto-common 0.1.7, crypto-common 0.2.2, defmt 1.1.1, defmt-macros 1.1.1, defmt-parser 1.0.0, deranged 0.5.8, digest 0.10.7, digest 0.11.3, dirs 7.0.0, dirs-sys 0.5.0, displaydoc 0.2.7, dtoa 1.0.11, dyn-clone 1.0.20, erased-serde 0.4.10, fdeflate 0.3.7, find-msvc-tools 0.1.14, flate2 1.1.10, form_urlencoded 1.2.2, futures-channel 0.3.34, futures-core 0.3.34, futures-io 0.3.34, futures-macro 0.3.34, futures-sink 0.3.34, futures-task 0.3.34, futures-util 0.3.34, getrandom 0.3.4, getrandom 0.4.3, glob 0.3.4, hashbrown 0.12.3, hashbrown 0.17.1, heck 0.5.0, hex 0.4.3, html5ever 0.39.0, http 1.5.0, httparse 1.10.1, httpdate 1.0.3, hybrid-array 0.4.15, idna 1.1.0, ipnet 2.12.2, itoa 1.0.18, jsonptr 0.7.1, keyboard-types 0.8.3, libc 0.2.190, lock_api 0.4.14, log 0.4.34, markup5ever 0.39.0, mime 0.3.17, num-conv 0.2.2, num-traits 0.2.19, once_cell 1.21.4, parking_lot 0.12.5, parking_lot_core 0.9.12, percent-encoding 2.3.2, png 0.17.16, png 0.18.1, powerfmt 0.2.1, proc-macro2 1.0.107, quote 1.0.47, ref-cast 1.0.27, ref-cast-impl 1.0.27, regex 1.13.1, regex-automata 0.4.18, regex-syntax 0.8.11, rustc_version 0.4.1, rustversion 1.0.23, scopeguard 1.2.0, semver 1.0.28, serde 1.0.229, serde-untagged 0.1.9, serde_core 1.0.229, serde_derive 1.0.229, serde_derive_internals 0.29.1, serde_json 1.0.151, serde_path_to_error 0.1.20, serde_repr 0.1.21, serde_spanned 1.1.1, serde_with 3.24.0, serde_with_macros 3.24.0, serialize-to-javascript 0.1.2, serialize-to-javascript-impl 0.1.2, servo_arc 0.4.3, sha2 0.10.9, sha2 0.11.0, shlex 2.0.1, siphasher 1.0.4, smallvec 1.16.2, socket2 0.6.5, softbuffer 0.4.8, stable_deref_trait 1.2.1, string_cache 0.9.0, string_cache_codegen 0.6.1, syn 2.0.119, syn 3.0.6, tendril 0.5.1, thiserror 2.0.21, thiserror-impl 2.0.21, time 0.3.55, time-core 0.1.9, time-macros 0.2.32, toml 1.1.6+spec-1.1.0, toml_datetime 1.1.1+spec-1.1.0, toml_parser 1.1.3+spec-1.1.0, toml_writer 1.1.2+spec-1.1.0, tray-icon 0.25.1, typeid 1.0.3, typenum 1.20.1, unicase 2.10.0, unicode-segmentation 1.13.3, url 2.5.8, web-time 1.1.0, web_atoms 0.2.6, windows 0.62.2, windows-collections 0.3.2, windows-core 0.62.2, windows-future 0.3.2, windows-implement 0.60.2, windows-interface 0.59.3, windows-link 0.2.1, windows-numerics 0.3.1, windows-result 0.4.1, windows-strings 0.5.1, windows-sys 0.59.0, windows-sys 0.61.2, windows-targets 0.52.6, windows-threading 0.2.1, windows-version 0.1.7, windows_x86_64_gnu 0.52.6, windows_x86_64_msvc 0.52.6

### MIT (65)

axum 0.7.9, axum-core 0.4.5, bytes 1.12.1, cargo_metadata 0.19.2, cfb 0.14.0, darling 0.24.1, darling_core 0.24.1, darling_macro 0.24.1, derive_more 2.1.1, derive_more-impl 2.1.1, dom_query 0.28.0, embed-resource 3.0.12, generic-array 0.14.7, http-body 1.1.0, http-body-util 0.1.5, http-range-header 0.4.2, hyper 1.12.0, hyper-util 0.1.21, ico 0.5.0, infer 0.22.0, mime_guess 2.0.5, mio 1.2.4, new_debug_unreachable 1.0.6, phf 0.13.1, phf_codegen 0.13.1, phf_generator 0.13.1, phf_macros 0.13.1, phf_shared 0.13.1, plist 1.10.1, precomputed-hash 0.1.1, quick-xml 0.42.0, rfd 0.15.4, rust-embed 8.13.0, rust-embed-impl 8.13.0, rust-embed-utils 8.13.0, schemars 0.8.22, schemars 0.9.0, schemars 1.2.2, schemars_derive 0.8.22, simd-adler32 0.3.10, slab 0.4.12, strsim 0.11.1, synstructure 0.14.0, tauri-winres 0.3.6, tokio 1.53.2, tokio-macros 2.7.2, tokio-util 0.7.19, tower 0.4.13, tower 0.5.3, tower-http 0.5.2, tower-layer 0.3.3, tower-service 0.3.3, tracing 0.1.44, tracing-core 0.1.36, try-lock 0.2.5, urlpattern 0.6.0, vswhom 0.1.0, vswhom-sys 0.1.3, want 0.3.2, webview2-com 0.39.1, webview2-com-macros 0.8.1, webview2-com-sys 0.39.1, winnow 1.0.4, winreg 0.56.0, zmij 1.0.23

### Apache-2.0 OR MIT (28)

atomic-waker 1.1.2, autocfg 1.5.1, bit-set 0.8.0, bit-vec 0.8.0, cargo_toml 1.0.1, const-oid 0.10.2, ctor 1.0.13, equivalent 1.0.2, fastrand 2.5.0, idna_adapter 1.2.2, indexmap 1.9.3, indexmap 2.14.2, muda 0.20.0, pin-project 1.1.13, pin-project-internal 1.1.13, pin-project-lite 0.2.17, rustc-hash 2.1.3, tauri 2.12.1, tauri-build 2.7.1, tauri-codegen 2.7.1, tauri-macros 2.7.1, tauri-runtime 2.12.1, tauri-runtime-wry 2.12.1, tauri-utils 2.10.1, utf8_iter 1.0.4, uuid 1.27.0, window-vibrancy 0.8.1, wry 0.57.0

### Unicode-3.0 (18)

icu_collections 2.3.0, icu_locale_core 2.3.0, icu_normalizer 2.3.0, icu_normalizer_data 2.3.0, icu_properties 2.3.0, icu_properties_data 2.3.0, icu_provider 2.3.1, litemap 0.8.3, potential_utf 0.1.6, tinystr 0.8.4, writeable 0.6.4, yoke 0.8.3, yoke-derive 0.8.4, zerofrom 0.1.8, zerofrom-derive 0.1.8, zerotrie 0.2.5, zerovec 0.11.8, zerovec-derive 0.11.6

### Unlicense OR MIT (8)

aho-corasick 1.1.5, byteorder 1.5.0, jiff 0.2.38, jiff-core 0.1.1, jiff-tzdb 0.1.9, jiff-tzdb-platform 0.1.3, memchr 2.8.3, winapi-util 0.1.11

### MIT/Apache-2.0 (6)

bitflags 1.3.2, bs58 0.5.1, ident_case 1.0.1, json-patch 4.2.0, serde_urlencoded 0.7.1, version_check 0.9.5

### MPL-2.0 (5)

cssparser 0.37.0, cssparser-macros 0.7.1, dtoa-short 0.3.5, option-ext 0.2.0, selectors 0.38.0

### Apache-2.0 (2)

sync_wrapper 1.0.2, tao 0.37.1

### BSD-3-Clause (2)

alloc-no-stdlib 3.0.0, alloc-stdlib 0.3.0

### MIT OR Zlib OR Apache-2.0 (2)

miniz_oxide 0.8.9, miniz_oxide 0.9.1

### Unlicense/MIT (2)

same-file 1.0.6, walkdir 2.5.0

### Zlib (2)

foldhash 0.2.0, zlib-rs 0.6.8

### (MIT OR Apache-2.0) AND Unicode-3.0 (1)

unicode-ident 1.0.26

### 0BSD OR MIT OR Apache-2.0 (1)

adler2 2.0.1

### Apache-2.0 / MIT (1)

fnv 1.0.7

### Apache-2.0 AND MIT (1)

dpi 0.1.2

### Apache-2.0 OR BSL-1.0 (1)

ryu 1.0.23

### BSD-3-Clause AND MIT (1)

brotli 9.0.0

### BSD-3-Clause/MIT (1)

brotli-decompressor 6.0.1

### CC0-1.0 OR MIT-0 OR Apache-2.0 (1)

dunce 1.0.5

### MIT AND BSD-3-Clause (1)

matchit 0.7.3

### MIT OR Apache-2.0 OR Zlib (1)

raw-window-handle 0.6.2

### Zlib OR Apache-2.0 OR MIT (1)

tinyvec 1.13.3
