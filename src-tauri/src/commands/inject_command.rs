use chrono;
use dll_syringe::process::OwnedProcess;
use dll_syringe::Syringe;
use std::fs::File;
use std::io::prelude::*;

/// Injects a dll into a process
/// # Arguments
/// * `process` - The name of the process to inject into
/// * `dll_path` - The path to the dll to inject
#[tauri::command]
pub fn inject(process: &str, dll_path: &str) {
    // TODO : Make this not depend on dll_syringe

    // Find the process that the user wants to inject into
    let target_process = OwnedProcess::find_first_by_name(&process).unwrap();

    // Creates Syringe instance with the target process
    // Injects the dll into the process
    let instance = Syringe::for_process(target_process);

    let result = instance.inject(dll_path).unwrap();

    // Create folder if it doesn't exist in the running directory with the name sola
    std::fs::create_dir_all("sola").unwrap();
    let mut file = File::create(format!(
        "sola\\log-{}.txt",
        chrono::Local::now().format("%Y-%m-%d %H-%M-%S")
    ))
    .unwrap();

    // Write the header to the file
    file.write_all(format!("Process: {}\r", process).as_bytes())
        .unwrap();
    file.write_all(format!("DLL: {}\r", dll_path).as_bytes())
        .unwrap();
    file.write_all(format!("Result: {}\r", format!("{:?}", result)).as_bytes())
        .unwrap();

    // Closes the tauri application
    std::process::exit(0);
}
