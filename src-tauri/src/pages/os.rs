use crate::functions::utils;

#[tauri::command]
pub fn os_name() -> String {
    return utils::uppercase(std::env::consts::OS).to_string().into();
}

#[tauri::command]
pub fn distro() -> String {
    let mut base: String = String::from("N/A");

    if os_name() == "Linux" {
        base = whoami::distro();
    }

    return base;
}
