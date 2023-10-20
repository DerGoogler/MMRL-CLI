use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use zip::ZipArchive;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MMRLJSON {
    pub require: Vec<String>,
}

pub fn get_mmrl_json(path: &str) -> Result<MMRLJSON, serde_json::Error> {
    let fname = Path::new(path);
    let zipfile = File::open(fname).unwrap();

    let mut archive = ZipArchive::new(zipfile).unwrap();

    let mut file = match archive.by_name("mmrl.json") {
        Ok(file) => file,
        Err(..) => {
            println!("mmrl.json not found");
            return serde_json::from_str("{\"require\":[]}")
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return serde_json::from_str(contents.as_str());
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
    let url_regex: &str = r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}(\.[a-z]{2,4})?\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";
    return Regex::new(url_regex).unwrap().is_match(url);
}
