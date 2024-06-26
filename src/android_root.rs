use regex::Regex;
use std::fs;
use std::{
    env,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    process::exit,
};

use ini::Ini;

#[cfg(target_os = "linux")]
pub fn get_downloads_dir() -> String {
    return match env::var("HOME") {
        Ok(val) => [val, String::from("Downloads")].join("/"),
        Err(e) => panic!("could not find {}: {}", "HOME", e),
    };
}

pub fn is_mmrl() -> bool {
    return match env::var("MMRL_INTR") {
        Ok(_val) => true,
        Err(_e) => false,
    };
}

#[cfg(target_os = "android")]
pub fn get_downloads_dir() -> String {
    return match env::var("EXTERNAL_STORAGE") {
        Ok(val) => [val, String::from("Download")].join("/"),
        Err(e) => panic!("could not find {}: {}", "EXTERNAL_STORAGE", e),
    };
}

pub fn check_paths(paths: Vec<&str>) -> Result<bool, io::Error> {
    for _path in paths {
        let path = PathBuf::from(_path);
        if path.exists() {
            let _ = Ok::<bool, io::Error>(path.exists());
        } else {
            let _ = Ok::<bool, io::Error>(false);
        }
    }

    Ok(false)
}

struct Searcher {
    regex: Regex,
}

impl Searcher {
    fn new(pattern: &str) -> Searcher {
        Searcher {
            regex: Regex::new(pattern).unwrap(),
        }
    }

    fn search(&self, contents: &str) -> bool {
        self.regex.is_match(contents)
    }
}

fn mount_detect(searcher: &Searcher) -> bool {
    let path = Path::new("/proc/self/mounts");

    if path.exists() {
        if let Ok(contents) = fs::read_to_string(path) {
            searcher.search(&contents)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn has_magisk_su() -> bool {
    return mount_detect(&Searcher::new(r"(magisk|core\/mirror|core\/img)"));
}

pub fn has_kernel_su() -> bool {
    return mount_detect(&Searcher::new(r"(KSU|KernelSU)"));
}

pub fn has_apatch_su() -> bool {
    return mount_detect(&Searcher::new(r"(APD|APatch)"));
}

pub fn get_root_manager() -> &'static str {
    if has_magisk_su() {
        return "Magisk";
    } else if has_kernel_su() {
        return "KernelSU";
    } else if has_apatch_su() {
        return "APatchSU";
    } else {
        return "Unknown";
    }
}

pub fn get_install_cli(path: &str) -> (&str, Vec<&str>) {
    let msu = "/data/adb/magisk/magisk64";
    let ksu = "/data/adb/ksu/bin/ksud";
    let asu = "/data/adb/ap/bin/apd";

    match get_root_manager() {
        "Magisk" => {
            return (msu, vec!["--install-module", path]);
        }
        "KernelSU" => {
            return (ksu, vec!["module", "install", path]);
        }
        "APatchSU" => {
            return (asu, vec!["module", "install", path]);
        }
        "Unknown" => {
            println!("! Unable to determine install cli");
            exit(0)
        }
        _ => {
            println!("! Unable to determine install cli");
            exit(0)
        }
    }
}

pub fn module_state(id: String, state: &str) {
    let base_path = Path::new("/data/adb/modules").join(id);
    let mod_state = base_path.join(state);

    let moduleprop = base_path.join("module.prop");

    if base_path.exists() && moduleprop.exists() && !mod_state.exists() {
        let conf = Ini::load_from_file(moduleprop.to_str().unwrap()).unwrap();
        let prop = conf.section(None::<String>).unwrap();
        let mut f = File::create(mod_state).unwrap();
        match f.write_all(b"") {
            Ok(_addr) => {
                println!("{} will be {}d.", prop.get("name").unwrap(), state);
            }
            Err(err) => {
                println!("{}", err);
                exit(1);
            }
        }
    }
}
