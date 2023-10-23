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

pub fn has_magisk_su() -> bool {
    let msu_paths = vec![
        "/system/bin/magisk",
        "/data/adb/magisk.db",
        "/data/adb/magisk/busybox",
        "/data/adb/magisk/magisk64",
        "/data/adb/magisk/magiskboot",
        "/data/adb/magisk/magiskinit",
        "/data/adb/magisk/magiskpolicy",
    ];
    return check_paths(msu_paths).unwrap();
}

pub fn has_kernel_su() -> bool {
    let ksu_paths = vec![
        "/data/adb/ksud",
        "/data/adb/ksu/modules.img",
        "/data/adb/ksu/bin/busybox",
        "/data/adb/ksu/bin/ksud",
        "/data/adb/ksu/bin/resetprop",
    ];
    return check_paths(ksu_paths).unwrap();
}

pub fn get_root_manager() -> String {
    if has_magisk_su() {
        return String::from("Magisk");
    } else if has_kernel_su() {
        return String::from("KernelSU");
    } else {
        return String::from("Unknown");
    }
}

pub fn get_install_cli(path: &str) -> (&str, Vec<&str>) {
    let msu = "/system/bin/magisk";
    let ksu = "/data/adb/ksu/bin/ksud";

    if Path::new(msu).exists() {
        return (msu, vec!["--install-module", path]);
    } else if Path::new(ksu).exists() {
        return (ksu, vec!["module", "install", path]);
    } else {
        println!("Unable to determine install cli");
        exit(0)
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
