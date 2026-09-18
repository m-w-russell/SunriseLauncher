mod commands;
// The parsing and argument helpers only run on macOS; the tests exercise them everywhere.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
mod crossover;
mod depot_errors;
mod error;
mod github;
mod installer;
mod missions;
mod models;
mod storage;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(commands::OperationState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_zoom(1.0);
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if matches!(
                event,
                tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed
            ) {
                window
                    .app_handle()
                    .state::<commands::OperationState>()
                    .cancel_active();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_snapshot,
            commands::inspect_installation,
            commands::save_preferences,
            commands::run_operation,
            commands::send_terminal_input,
            commands::cancel_operation,
            commands::launch_game,
            commands::get_crossover_status,
            commands::prepare_crossover,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
