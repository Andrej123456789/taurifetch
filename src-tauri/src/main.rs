#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod details;
mod functions;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            details::os::os_name,
            details::uptime::uptime,
            details::packages::packages,
            details::shell::shell,
            details::gui::gui,
            details::cpu::cpu,
            details::gpu::gpu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
