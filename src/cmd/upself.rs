use crate::android_root::{get_install_cli, get_downloads_dir};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::process::{Command, Stdio};

pub async fn upself(client: Client, version: String) {
    let zip_url = &format!("https://github.com/DerGoogler/MMRL-CLI/releases/download/v0.1.0/mmrl-{}-module-aarch64.zip", version);

    let path = &[
        get_downloads_dir(),
        [["mmrl", &version].join("-"), "zip".to_string()].join("."),
    ]
    .join("/");

    let res = client
        .get(zip_url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &zip_url)))
        .unwrap();
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &zip_url))
        .unwrap();

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
.template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
.progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", version));

    // download chunks
    let mut file = File::create(path)
        .or(Err(format!("Failed to create file '{}'", path)))
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item
            .or(Err(format!("Error while downloading file")))
            .unwrap();
        let _ = file
            .write_all(&chunk)
            .or(Err(format!("Error while writing to file")));
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", version, path));

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
