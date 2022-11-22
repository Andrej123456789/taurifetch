#[tauri::command]
pub fn gui() -> String {
    /* Credits to: https://github.com/demurgos/detect-desktop-environment */
    #[cfg(target_os = "macos")]
    pub fn detect() -> Self {
        return String::from("macOS DE")
    }

    #[cfg(target_os = "windows")]
    pub fn detect() -> Self {
        return String::from("Windows DE")
    }

    match std::env::var("XDG_CURRENT_DESKTOP") {
        Err(_) => return String::from("Unknown"),
        Ok(current_desktop) => {
          match current_desktop.as_str() {
            "Cinnamon" => return String::from("Cinnamon"),
            "ENLIGHTENMENT" => String::from("Enlightenment"),
            "GNOME" => String::from("GNOME"),
            "KDE" => String::from("KDE"),
            "LXDE" => String::from("LXDE"),
            "LXQt" => String::from("LXQt"),
            "MATE" => String::from("MATE"),
            "Unity" => String::from("Unity"),
            "X-Cinnamon" => String::from("X-Cinnamon"),
            "XFCE" => String::from("XFCE"),
            _ => String::from("Unknown"),
          }
        }
      }

    // whoami::desktop_env().to_string()
}
