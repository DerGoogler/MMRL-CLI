extern crate serde;
extern crate serde_ini;

use crate::utils::{confirm, download_from_url, read_module_prop_file};

use crate::android_root::{get_install_cli, get_downloads_dir};
use crate::cmd::info::info;
use crate::repo::{Repo, get_id_details, find_module, find_version};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::process::{exit, Command, Stdio};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
struct Dependencies {
    name: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
struct MMRLINI {
    // key1: String,
    // key2: u32,
    // key3: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependencies: Option<Box<Dependencies>>,
}

pub async fn install(client: Client, yes: bool, json: &Repo, id: String) {
    let (_id, _ver) = get_id_details(id);
    info(json, _id.clone()).await;
    let module = find_module(&json, _id.clone());
    let version = find_version(module.versions.clone(), _ver);

    let path = &[
        get_downloads_dir(),
        [[version.version.clone(), module.id].join("-"), "zip".to_string()].join("."),
    ]
    .join("/");

    println!("Downloading {}", module.name);
    println!("Version: {}\n", &version.version);

    let path = download_from_url(client.clone(), version.zip_url, module.name, path).await;

    let mini = read_module_prop_file(&path);

    if mini.is_ok() {
        let props: Result<MMRLINI, serde_ini::de::Error> =
            serde_ini::from_str::<MMRLINI>(&mini.unwrap());
        if props.is_ok() {
            let deps = props.unwrap().dependencies.unwrap();
            for dep in deps.name {
                let dep_path = Path::new("/data/adb/modules")
                    .join(dep.clone())
                    .join("module.prop");
                let dep_path_update = Path::new("/data/adb/modules_update")
                    .join(dep.clone())
                    .join("module.prop");
                if !dep_path.exists() || !dep_path_update.exists() {
                    println!("This module requires {} to be installed", dep.clone());
                    exit(1)
                }
            }
        }
    }

    let success = yes || confirm("\nDo you want to continue [y/N]? ");

    if success {
        let (bin, args) = get_install_cli(&path);

        let stdout = Command::new(bin)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
            .unwrap();

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    } else {
        println!("Installtion canceled");
        exit(0);
    }
}
