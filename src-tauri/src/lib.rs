mod player;
use player::{init_gstreamer, play_video_overlay};
use raw_window_handle::{RawWindowHandle, HasWindowHandle};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![play_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn play_video(app: tauri::AppHandle, uri: &str) -> Result<(), String> {
    let window = app.get_webview_window("main").unwrap();
    let handle_wrapper = window.window_handle().unwrap();
    let raw_handle: RawWindowHandle = handle_wrapper.as_raw();
    let handle: usize = match raw_handle {
        RawWindowHandle::WinRt(h) => h.core_window.as_ptr() as usize,
        RawWindowHandle::Win32(h) => h.hwnd.get() as usize,
        RawWindowHandle::AppKit(h) => h.ns_view.as_ptr() as usize,
        RawWindowHandle::Wayland(h) => h.surface.as_ptr() as usize,
        RawWindowHandle::Xlib(h) => h.window as usize,
        _ => 0,
    };

    init_gstreamer().map_err(|e| e.to_string())?;

    play_video_overlay(handle, format!("file:/{uri}").as_str()).map_err(|e| e.to_string())?;

    Ok(())
}
