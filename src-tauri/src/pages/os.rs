use sysinfo::{System, SystemExt};

use crate::functions::utils;

#[tauri::command]
pub fn os_name() -> String {
    return utils::uppercase(std::env::consts::OS).to_string().into();
}

#[tauri::command]
pub fn kernel_version() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();
    sys.kernel_version().unwrap().to_string()
}

#[tauri::command]
pub fn distro() -> String {
    return whoami::distro().to_string();
}
