// No console window in release builds; `mcp` still talks over the stdio pipes its parent hands it.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;

use capcut_core::activity::studio_alive;
use capcut_core::settings::Settings;
use std::net::TcpListener;
use tauri::Manager;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let port = args.iter().position(|a| a == "--port").and_then(|i| args.get(i + 1)).and_then(|p| p.parse().ok()).unwrap_or_else(|| Settings::load().port());
    match args.first().map(String::as_str) {
        Some("mcp") => {
            if let Err(e) = capcut_core::mcp::serve_stdio() {
                eprintln!("mcp: {e:#}");
                std::process::exit(1);
            }
        }
        Some("serve") => {
            eprintln!("Draft Studio {}: http://127.0.0.1:{port}/", capcut_core::VERSION);
            let rt = tokio::runtime::Runtime::new().expect("tokio");
            if let Err(e) = rt.block_on(server::serve(port)) {
                eprintln!("server: {e:#}");
                std::process::exit(1);
            }
        }
        _ => run_desktop(port),
    }
}

/// Start the local server (unless another studio already owns the port) and open it in a
/// native window.
fn run_desktop(port: u16) {
    let busy = TcpListener::bind(("127.0.0.1", port)).is_err();
    if !(busy && studio_alive(port)) {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("tokio");
            if let Err(e) = rt.block_on(server::serve(port)) {
                eprintln!("server: {e:#}");
            }
        });
        for _ in 0..100 {
            if studio_alive(port) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
    let url: tauri::Url = format!("http://127.0.0.1:{port}/").parse().expect("url");
    tauri::Builder::default()
        .setup(move |app| {
            tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::External(url.clone()))
                .title("Draft Studio")
                .inner_size(1440.0, 900.0)
                .min_inner_size(1100.0, 680.0)
                .theme(Some(tauri::Theme::Dark))
                .maximized(true)
                .build()?;
            // setting "open the studio when an agent builds": bring this window forward
            let handle = app.handle().clone();
            server::on_agent_build(move || {
                if let Some(w) = handle.get_webview_window("main") {
                    let _ = w.unminimize();
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("không mở được cửa sổ Draft Studio");
}
