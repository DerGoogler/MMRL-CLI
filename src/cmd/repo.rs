use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, Write};
use std::process::exit;

use crate::repo::{find_module, Module};

fn add_repos_to_json(file_path: &str, new_repos: Vec<String>) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path);

    let mut file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut json: Value = if content.trim().is_empty() {
        Value::Array(vec![])
    } else {
        serde_json::from_str(&content)?
    };

    if let Value::Array(ref mut repos) = json {
        for repo in new_repos {
            let repo_value = Value::String(repo.to_string());
            if !repos.contains(&repo_value) {
                repos.push(repo_value);
            } else {
                println!("- \"{}\" has been already added", repo)
            }
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "JSON is not an array",
        ));
    }

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(serde_json::to_string_pretty(&json)?.as_bytes())?;

    Ok(())
}

pub async fn add(url: Vec<String>) -> () {
    let file_path = "/data/adb/mmrl/repos.json";
    match add_repos_to_json(file_path, url) {
        Ok(_) => {
            println!("- Repositories added successfully.");
            exit(0)
        }
        Err(e) => {
            eprintln!("! Error adding repositories: {}", e);
            exit(500)
        }
    }
}
