use std::env;

#[cfg(target_os = "linux")]
pub fn get_downloads_dir() -> String {
    return match env::var("HOME") {
        Ok(val) => [val, String::from("Downloads")].join("/"),
        Err(e) => panic!("could not find {}: {}", "HOME", e),
    };
}

#[cfg(target_os = "android")]
pub fn get_downloads_dir() -> String {
    return match env::var("EXTERNAL_STORAGE") {
        Ok(val) => [val, String::from("Download")].join("/"),
        Err(e) => panic!("could not find {}: {}", "EXTERNAL_STORAGE", e),
    };
}
