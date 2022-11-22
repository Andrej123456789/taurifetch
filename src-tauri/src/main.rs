#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod functions;
mod pages;

#[tauri::command]
fn theme_name(new_theme: String) -> String {
    let filename = ".theme.txt";

    if Path::new(filename).exists() == false {
        let mut file = File::create(filename).expect("Error encountered while creating file!");

        file.write_all(b"dark")
            .expect("Error while writing to file! (1)");
    }

    if new_theme == "none" {
        let mut file = File::open(filename).expect("File not found!");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Error while reading file! (2)");
            
        return data;
    } else {
        if Path::new(filename).exists() == true {
            std::fs::remove_file(filename).expect("Error occured while trying to delete file!");
        }

        let mut file = File::create(filename).expect("Error encountered while creating file!");

        file.write_all(new_theme.as_bytes())
            .expect("Error while writing to file! (3)");
    }

    return new_theme;
}

#[tauri::command]
fn user_computer() -> String {
    let user: String = whoami::username().to_string();
    let hostname: String = whoami::hostname().to_string();
    let at: String = String::from("@");

    let mut for_return: String = String::new();
    for_return.push_str(&user);
    for_return.push_str(&at);
    for_return.push_str(&hostname);

    return for_return;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            theme_name,
            user_computer,
            pages::os::os_name,
            pages::os::kernel_version,
            pages::os::distro,
            pages::uptime::uptime,
            pages::uptime::epoch,
            pages::uptime::epoch_nice,
            pages::packages::packages,
            pages::shell::shell,
            pages::gui::gui,
            pages::cpu::cpu,
            pages::cpu::cpu_manufacturer,
            pages::cpu::cpu_cores,
            pages::cpu::cpu_threads,
            pages::cpu::sse_support,
            pages::cpu::l1_cache,
            pages::cpu::l2_cache,
            pages::cpu::l3_cache,
            pages::gpu::gpu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
