use crate::{
    android_root::get_downloads_dir,
    repo::{find_module, Repo},
};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io::Write;

pub async fn download(client: Client, json: &Repo, id: String) -> String {
    let module = find_module(&json, id);

    println!("Downloading {}", module.name);

    let zip_url = &module.versions[0].clone().zip_url.to_owned()[..];

    let path = &[
        get_downloads_dir(),
        [
            [module.versions[0].clone().version, module.id].join("-"),
            "zip".to_string(),
        ]
        .join("."),
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
    pb.set_message(format!("Downloading {}", module.name));

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

    pb.finish_with_message(format!("Downloaded {} to {}", module.name, path));

    return path.to_string();
}
