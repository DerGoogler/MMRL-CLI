extern crate serde;
extern crate serde_ini;

use crate::android_root::{get_downloads_dir, get_install_cli};
use crate::repo::{find_module, find_version, get_id_details, Module};
use crate::utils::{confirm, download_from_url, get_last, get_mmrl_json, is_git, is_url, zip_dir};
use async_recursion::async_recursion;
use git2::Repository;
use reqwest::Client;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use walkdir::WalkDir;

#[async_recursion]
async fn check_requires(
    _is_git: bool,
    path: String,
    install_requires: bool,
    client: Client,
    yes: bool,
    modules: &Vec<Module>,
) -> () {
    let mini: Result<crate::utils::MMRLJSON, serde_json::Error>;

    if _is_git {
        mini = match File::open(path) {
            Ok(file) => serde_json::from_reader(file),
            Err(..) => serde_json::from_str("{\"require\":[]}"),
        };
    } else {
        mini = get_mmrl_json(&path);
    }

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
                install(client.clone(), yes, install_requires, modules, req).await;
            } else {
                println!("This module requires {} to be installed", req.clone());
                exit(1)
            }
        }
    }
}

const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);

#[async_recursion]
pub async fn install(client: Client, yes: bool, requires: bool, modules: &Vec<Module>, id: String) {
    let _url = &id.to_owned()[..];
    if is_git(_url) {
        let name = get_last(_url);
        let path = &[get_downloads_dir(), name.unwrap()].join("/");
        match Repository::clone(_url, path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };

        check_requires(
            true,
            [path, "mmrl.json"].join("/"),
            requires,
            client.clone(),
            yes,
            modules,
        )
        .await;

        let file = File::create([path, "zip"].join(".")).unwrap();

        let walkdir = WalkDir::new(path);
        let it = walkdir.into_iter();

        zip_dir(
            &mut it.filter_map(|e| e.ok()),
            path,
            file,
            METHOD_STORED.unwrap(),
        )
        .unwrap();

        if Path::new(path).exists() {
            fs::remove_dir_all(path).expect("File delete failed");
        }

        println!("Info not availabe in git install");
        let success = yes || confirm("Do you want to continue [y/N] ");

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
    } else if is_url(_url) {
        let name = get_last(_url);
        let path = &[
            get_downloads_dir(),
            [name.clone().unwrap().to_string(), "zip".to_string()].join("."),
        ]
        .join("/");
        download_from_url(client.clone(), id.clone(), name.unwrap(), path).await;
        check_requires(false, path.clone(), requires, client.clone(), yes, modules).await;

        println!("Info not availabe in url install");
        let success = yes || confirm("Do you want to continue [y/N] ");

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
        let (_id, _ver) = get_id_details(id);
        let module = find_module(&modules, _id.clone());
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
        check_requires(false, path.clone(), requires, client.clone(), yes, modules).await;

        let success = yes || confirm("Do you want to continue [y/N] ");

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
