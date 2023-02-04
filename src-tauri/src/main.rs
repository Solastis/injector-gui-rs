#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
use commands::inject_command;

fn main() {
    // Tauri app builder and run
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inject_command::inject])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
