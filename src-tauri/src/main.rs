#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use raw_cpuid::CpuId;

// Credits: https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn os_name() -> String {
    return uppercase(std::env::consts::OS).to_string().into();
}

#[tauri::command]
fn uptime() -> String {
    "uptime".to_string().into()
}

#[tauri::command]
fn packages() -> String {
    "packages".to_string().into()
}

#[tauri::command]
fn shell() -> String {
    "shell".to_string().into()
}

#[tauri::command]
fn gui() -> String {
    "gui".to_string().into()
}

#[tauri::command]
fn cpu() -> String {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        assert!(vf.as_str() == "GenuineIntel" || vf.as_str() == "AuthenticAMD");
    }

    let has_sse = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_sse());
    if has_sse {
        println!("CPU supports SSE!");
    }

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets();
            println!("L{}-Cache size is {}", cache.level(), size);
        }
    } else {
        println!("No cache parameter information available")
    }

    println!(" ");

    "cpu".to_string().into()
}

#[tauri::command]
fn gpu() -> String {
    "gpu".to_string().into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            os_name, uptime, packages, shell, gui, cpu, gpu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
