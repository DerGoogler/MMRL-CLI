extern crate serde;
extern crate serde_ini;

use crate::utils::{confirm, download_from_url, get_last, get_mmrl_json, is_url};
use crate::android_root::{get_downloads_dir, get_install_cli};
use crate::repo::{find_module, find_version, get_id_details, Repo};
use reqwest::Client;
use async_recursion::async_recursion;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::process::{exit, Command, Stdio};

#[async_recursion]
async fn check_requires(
    path: String,
    install_requires: bool,
    client: Client,
    yes: bool,
    json: &Repo,
) {
    let mini = get_mmrl_json(&path);

    for req in mini.unwrap().require {
        let dep_path = Path::new("/data/adb/modules")
            .join(req.clone())
            .join("module.prop");
        let dep_path_update = Path::new("/data/adb/modules_update")
            .join(req.clone())
            .join("module.prop");
        if !(dep_path_update.exists() || dep_path.exists()) {
            if install_requires {
                println!("Install requires");
                install(client.clone(), yes, install_requires,  json, req).await;
            } else {
                println!("This module requires {} to be installed", req.clone());
                exit(1)
            }
        }
    }
}

#[async_recursion]
pub async fn install(client: Client, yes: bool, requires: bool, json: &Repo, id: String) {
    let _url = &id.to_owned()[..];
    if !is_url(_url) {
        let (_id, _ver) = get_id_details(id);
        let module = find_module(&json, _id.clone());
        let version = find_version(module.versions.clone(), _ver);

        let path = &[
            get_downloads_dir(),
            [
                [version.version.clone(), module.id].join("-"),
                "zip".to_string(),
            ]
            .join("."),
        ]
        .join("/");

        println!("Downloading {}", module.name);
        println!("Version: {}", &version.version);

        download_from_url(client.clone(), version.zip_url, module.name, path).await;
        check_requires(path.clone(), requires, client.clone(), yes, json).await;

        let success = yes || confirm("Do you want to continue [y/N]? ");

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
            exit(0);
        }
    } else {
        let name = get_last(_url);
        let path = &[
            get_downloads_dir(),
            [name.clone().unwrap().to_string(), "zip".to_string()].join("."),
        ]
        .join("/");
        download_from_url(client.clone(), id.clone(), name.unwrap(), path).await;
        check_requires(path.clone(), requires, client.clone(), yes, json).await;

        println!("Info not availabe in url install");
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
            exit(0);
        }
    }
}
