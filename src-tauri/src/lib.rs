mod player;
use player::{init_gstreamer, play_video};

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
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn play(file_path: &str) -> Result<(), String> {
    init_gstreamer().map_err(|e| e.to_string())?;
    play_video(format!("file:/{file_path}").as_str()).map_err(|e| e.to_string())?;
    Ok(())
}