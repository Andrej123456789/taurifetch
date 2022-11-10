#[tauri::command]
pub fn shell() -> String {
    "shell".to_string().into()
}
