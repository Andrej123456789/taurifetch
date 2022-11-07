#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn os_name() -> String {
    "operating system".to_string().into()
}

#[tauri::command]
fn uptime() -> String {
    "uptime".to_string().into()
}

#[tauri::command]
fn packages() -> String {
    "packages".to_string().into()
}

#[tauri::command]
fn shell() -> String {
    "shell".to_string().into()
}

#[tauri::command]
fn gui() -> String {
    "gui".to_string().into()
}

#[tauri::command]
fn cpu() -> String {
    "cpu".to_string().into()
}

#[tauri::command]
fn gpu() -> String {
    "gpu".to_string().into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![os_name, uptime, packages, shell, gui, cpu, gpu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
