#[tauri::command]
pub fn uptime() -> String {
    "uptime".to_string().into()
}
