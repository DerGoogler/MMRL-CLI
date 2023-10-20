use std::fs::File;
use std::io;
use std::io::prelude::*;
use zip::ZipArchive;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::io::Write;
use regex::Regex;

pub fn read_module_prop_file(zip_file_path: &str) -> std::io::Result<String> {
    let zip_file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    let mut module_prop_file = archive.by_name("mmrl.ini")?;
    let mut contents = String::new();
    module_prop_file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn confirm(msg: &str) -> bool {
    loop {
        println!("{}", msg);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let trimmed = input.trim().to_lowercase();
        match trimmed.to_lowercase().as_str() {
            "yes" | "y" => {
                return true;
            }
            "no" | "n" => {
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}


pub async fn download_from_url(client: Client, url: String, name: String, path: &String) -> String {
    let res = client
        .get(url.clone())
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", url)))
        .unwrap();
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", url))
        .unwrap();

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
.template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
.progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", name));

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

    pb.finish_with_message(format!("Downloaded {} to {}", name, path));

    return path.to_string();
}

pub fn is_url(url: &str) -> bool {
    let url_regex: &str =
    r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}(\.[a-z]{2,4})?\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";
    return Regex::new(url_regex).unwrap().is_match(url);
}