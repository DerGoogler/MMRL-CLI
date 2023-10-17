use crate::android_root::get_install_cli;
use crate::cmd::{download::download, info::info};
use crate::repo::Repo;
use reqwest::Client;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

pub async fn install(client: Client, version:i64, json: &Repo, id: &String) {
    info(json, id.clone()).await;
    let path = download(client.clone(), version, &json, id.clone()).await;
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
