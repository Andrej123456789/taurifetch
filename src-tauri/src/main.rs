#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod pages;
mod functions;

#[tauri::command]
fn user_computer() -> String{
    let user: String = whoami::username().to_string();
    let hostname: String = whoami::hostname().to_string();
    let at: String = String::from("@");

    let mut for_return: String = String::new();
    for_return.push_str(&user);
    for_return.push_str(&at);
    for_return.push_str(&hostname);

    return for_return
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            user_computer,
            pages::os::os_name,
            pages::os::distro,
            pages::uptime::uptime,
            pages::uptime::epoch,
            pages::uptime::epoch_nice,
            pages::packages::packages,
            pages::shell::shell,
            pages::gui::gui,
            pages::cpu::cpu,
            pages::gpu::gpu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
