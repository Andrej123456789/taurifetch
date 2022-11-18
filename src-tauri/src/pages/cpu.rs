use raw_cpuid::CpuId;

#[tauri::command]
pub fn cpu() -> String {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        assert!(vf.as_str() == "GenuineIntel" || vf.as_str() == "AuthenticAMD");
        return vf.to_string();
    } else {
        return String::from("Error occured while fetching CPU name!");
    }
}

#[tauri::command]
pub fn cpu_manufacturer() -> String {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        assert!(vf.as_str() == "GenuineIntel" || vf.as_str() == "AuthenticAMD");

        if vf.to_string() == "GenuineIntel" {
            return String::from("Intel");
        } else if vf.to_string() == "AuthenticAMD" {
            return String::from("AMD");
        } else {
            return String::from("Unknown manufacturer!");
        }
    } else {
        return String::from("Error occured while fetching CPU manufacturer!");
    }
}

#[tauri::command]
pub fn sse_support() -> String {
    let cpuid = CpuId::new();

    let has_sse = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_sse());

    if has_sse {
        return String::from("true");
    } else {
        return String::from("false");
    }
}

#[tauri::command]
pub fn cpu_cores() -> String {
    let num = num_cpus::get();

    if num < 5 {
        return num.to_string();
    } else {
        return (num / 2).to_string();
    }
}

#[tauri::command]
pub fn cpu_threads() -> String {
    return num_cpus::get().to_string();
}

#[tauri::command]
pub fn l1_cache() -> String {
    let cpuid = CpuId::new();

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets();

            if cache.level() == 1 {
                return size.to_string();
            }
        }
        return String::from("Error while fetching CPU cache data!")
    } else {
        return String::from("No cache parameter information available");
    }
}

#[tauri::command]
pub fn l2_cache() -> String {
    let cpuid = CpuId::new();

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets();

            if cache.level() == 2 {
                return size.to_string();
            }
        }
        return String::from("Error while fetching CPU cache data!")
    } else {
        return String::from("No cache parameter information available");
    }
}

#[tauri::command]
pub fn l3_cache() -> String {
    let cpuid = CpuId::new();

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets();

            if cache.level() == 3 {
                return size.to_string();
            }
        }
        return String::from("Error while fetching CPU cache data!")
    } else {
        return String::from("No cache parameter information available");
    }
}
