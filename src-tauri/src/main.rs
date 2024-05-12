// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| {
        if let &tauri::RunEvent::MainEventsCleared = &event {
            return;
        }
        eprintln!("event = {event:?}")
    });
}
