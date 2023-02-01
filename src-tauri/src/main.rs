#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use dll_syringe::process::OwnedProcess;
use dll_syringe::Syringe;

/**
 * Injects a dll into a process
 * @param process The name of the process to inject into
 * @param dll_path The path to the dll to inject
 **/
#[tauri::command]
fn inject(process: &str, dll_path: &str) {
    // TODO : Make this work with id's instead of names

    // Find the process that the user wants to inject into
    let target_process = OwnedProcess::find_first_by_name(&process).unwrap();

    // Creates Syringe instance with the target process
    let instance = Syringe::for_process(target_process);

    // Injects the dll into the process
    let _ = instance.inject(dll_path).unwrap();
}

fn main() {
    // Tauri app builder and run
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inject])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
