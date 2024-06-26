extern crate serde;
extern crate serde_ini;

use crate::android_root::{get_downloads_dir, get_install_cli};
use crate::repo::{find_module, find_version, get_id_details, Module};
use crate::utils::{confirm, download_from_url, get_last, is_url};
use async_recursion::async_recursion;
use reqwest::Client;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{exit, Command, Stdio};

#[async_recursion]
pub async fn install(
    client: Client,
    yes: bool,
    _requires: bool,
    modules: &Vec<Module>,
    id: String,
) {
    let _url = &id.to_owned()[..];
    if is_url(_url) {
        let name = get_last(_url);
        let path = &[
            get_downloads_dir(),
            [name.clone().unwrap().to_string(), "zip".to_string()].join("."),
        ]
        .join("/");
        download_from_url(client.clone(), id.clone(), name.unwrap(), path).await;
        // check_requires(path.clone(), requires, client.clone(), yes, modules).await;

        println!("? Info not availabe in url install");
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

            for line in reader.lines() {
                match line {
                    Ok(ln) => println!("{}", ln),
                    Err(e) => {
                        println!("{}", e);
                        exit(500)
                    }
                }
            }
            exit(0);
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

        download_from_url(client.clone(), version.zip_url, module.name, path).await;
        // check_requires(path.clone(), requires, client.clone(), yes, modules).await;

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

            for line in reader.lines() {
                match line {
                    Ok(ln) => println!("{}", ln),
                    Err(e) => {
                        println!("{}", e);
                        exit(500)
                    }
                }
            }
        } else {
            exit(0);
        }
    }
}

pub async fn install_local(yes: bool, id: String) -> () {
    let success = yes || confirm("Do you want to continue [y/N] ");

    if success {
        let (bin, args) = get_install_cli(&id);

        let stdout = Command::new(bin)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
            .unwrap();

        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(ln) => println!("{}", ln),
                Err(e) => {
                    println!("{}", e);
                    exit(500)
                }
            }
        }
    } else {
        exit(0);
    }
}
