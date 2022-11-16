#[tauri::command]
pub fn packages() -> String {
    "packages".to_string().into()
}
