extern crate walkdir;
extern crate zip;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{stdout, Seek, Write};
use std::iter::Iterator;
use std::path::Path;
use url::Url;
use zip::write::FileOptions;
use zip::ZipArchive;

use walkdir::DirEntry;

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
            return serde_json::from_str("{\"require\":[]}");
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return serde_json::from_str(contents.as_str());
}

pub fn confirm(msg: &str) -> bool {
    loop {
        print!("{}", msg);
        let mut input = String::new();
        io::stdout().flush().unwrap();
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

pub fn get_last(url: &str) -> Result<String, &str> {
    Url::parse(url)
        .map_err(|_| "Unable to parse")?
        .path_segments()
        .ok_or("No segments")?
        .last()
        .ok_or("No items")
        .map(String::from)
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

pub fn is_git(url: &str) -> bool {
    let git_regex: &str =
        r"((git|ssh|http(s)?)|(git@[\w\.]+))(:(\/\/)?)([\w\.@\:\/\-~]+)(\.git)(\/)?";
    return Regex::new(git_regex).unwrap().is_match(url);
}

pub fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
