use crate::{
    android_root::get_downloads_dir,
    repo::{find_module, find_version, get_id_details, Repo},
    utils::{download_from_url, is_url, get_last},
};
use reqwest::Client;

pub async fn download(client: Client, json: &Repo, id: String) -> () {
    let _url = &id.to_owned()[..];
    if !is_url(_url) {
        let (_id, _ver) = get_id_details(id);
        let module = find_module(&json, _id.clone());

        let version = find_version(module.versions.clone(), _ver);
        println!("Downloading {}", module.name);
        println!("Version: {}\n", version.version);

        let name = get_last(_url);
        let path = &[
            get_downloads_dir(),
            [name.clone().unwrap().to_string(), "zip".to_string()].join("."),
        ]
        .join("/");

        download_from_url(client, version.zip_url, module.name, path).await;
    } else {
        let url_ = &id.to_owned()[..];
        let name = get_last(url_);
        let path = &[
            get_downloads_dir(),
            [name.clone().unwrap().to_string(), "zip".to_string()].join("."),
        ]
        .join("/");
        download_from_url(client, id.clone(), id, path).await;
    }
}
