extern crate serde;
extern crate serde_ini;

use crate::utils::read_module_prop_file;

use crate::android_root::get_install_cli;
use crate::cmd::{download::download, info::info};
use crate::repo::Repo;
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

pub async fn install(client: Client, version: i64, json: &Repo, id: &String) {
    info(json, id.clone()).await;
    let path = download(client.clone(), version, &json, id.clone()).await;

    let mini = read_module_prop_file(&path);

    if mini.is_ok() {
        let props: Result<MMRLINI, serde_ini::de::Error> = serde_ini::from_str::<MMRLINI>(&mini.unwrap());
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
}
