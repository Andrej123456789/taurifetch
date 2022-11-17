use std::time::{SystemTime, UNIX_EPOCH};
use compound_duration::format_dhms;

#[tauri::command]
pub fn uptime() -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
            return format_dhms(uptime.as_secs());
        }
        Err(err) => return err.to_string(),
    }
}

#[tauri::command]
pub fn epoch() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    return since_the_epoch.as_secs_f64().to_string();
}

#[tauri::command]
pub fn epoch_nice() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    return format_dhms(since_the_epoch.as_secs()); 
}
