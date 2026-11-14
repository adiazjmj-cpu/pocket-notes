// Pocket Notes — desktop shell (Tauri 2).
// The app itself is a single self-contained HTML file (dist/index.html); this
// binary only hosts it in the system webview. No plugins for data, no network,
// no filesystem access beyond what the webview itself provides (its own
// IndexedDB/localStorage, the browser file picker for import, and downloads
// for export) — except for the one native save path below. All of the app's
// behaviour lives in that one HTML file.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Write text to a file at an absolute path the user picked in the native
/// Save As dialog. This is the desktop shell's native save path, and it exists
/// for one reason: the webview's browser anchor download (`<a download>`) is
/// silently dropped by WebView2 on Windows, so export (backup .weave, page and
/// notebook .md/.html/.txt) cannot use the anchor in the desktop app. The live
/// app file routes export through this command when it runs inside Tauri
/// (it checks `window.__TAURI__` first and falls back to the anchor in a
/// plain browser). All exports are text, so one text command covers them.
#[tauri::command]
fn save_text_file(path: String, contents: String) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| e.to_string())
}

/// Fetch a URL's HTML as text, for web clips. The desktop shell fetches
/// server-side, so a pasted link works for any page — no browser CORS limit
/// (the webapp's browser fetch only works where the site allows it). Only
/// http(s) URLs, 10s timeout, and the body is capped by ureq's own limit,
/// so a huge page can't blow up memory. The app parses the title and
/// description out of the HTML itself; nothing is sent anywhere else.
#[tauri::command]
fn fetch_url_text(url: String) -> Result<String, String> {
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("only http(s) URLs".to_string());
    }
    ureq::get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
        .map_err(|e| e.to_string())?
        .into_string()
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![save_text_file, fetch_url_text])
        .run(tauri::generate_context!())
        .expect("error while running Pocket Notes");
}
