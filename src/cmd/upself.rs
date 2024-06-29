use crate::android_root::{get_downloads_dir, get_install_cli, ANDROID_PATH};
use crate::exit;
use crate::utils::{confirm, download_from_url};
use reqwest::Client;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

pub async fn upself(client: Client, version: String, yes: bool) {
    let zip_url = format!(
        "https://github.com/DerGoogler/MMRL-CLI/releases/download/v{}/mmrl-{}-module-aarch64.zip",
        version, version
    );

    let path = &[
        get_downloads_dir(),
        [["mmrl", &version].join("-"), "zip".to_string()].join("."),
    ]
    .join("/");

    download_from_url(client, zip_url, version, path).await;

    let success = yes || confirm("\nDo you want to continue [y/N]? ");

    if success {
        let (bin, args) = get_install_cli(&path);

        let stdout = Command::new(bin)
            .args(args)
            .env("PATH", ANDROID_PATH)
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
    }
}
