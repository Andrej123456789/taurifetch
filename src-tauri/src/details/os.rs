use crate::functions::utils;

#[tauri::command]
pub fn os_name() -> String {
    return utils::uppercase(std::env::consts::OS).to_string().into();
}
