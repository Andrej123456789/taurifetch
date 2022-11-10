#[tauri::command]
pub fn gpu() -> String {
    "gpu".to_string().into()
}
