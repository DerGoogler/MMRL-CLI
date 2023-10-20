use std::fs::File;
use std::io;
use std::io::prelude::*;
use zip::ZipArchive;

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
