// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use file_sorter_lib::{commands, processor::ProcessorState};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(ProcessorState::new())
        .invoke_handler(tauri::generate_handler![
            commands::enqueue_zips,
            commands::cancel_current,
            commands::clear_finished,
            commands::open_in_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
