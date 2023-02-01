#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use dll_syringe::process::OwnedProcess;
use dll_syringe::Syringe;

#[tauri::command]
fn inject(process: &str, dll_path: &str) {
    let target_process = OwnedProcess::find_first_by_name(&process).unwrap();
    let instance = Syringe::for_process(target_process);
    let _payload = instance.inject(dll_path).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inject])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
