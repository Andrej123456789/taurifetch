#[tauri::command]
pub fn gui() -> String {
    "gui".to_string().into()
}
