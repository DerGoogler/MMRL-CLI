use crate::{
    android_root::get_downloads_dir,
    repo::{find_module, find_version, get_id_details, Repo},
    utils::{download_from_url, is_url},
};
use reqwest::Client;

pub async fn download(client: Client, json: &Repo, id: String) -> () {
    if !is_url(&id.to_owned()[..]) {
        let (_id, _ver) = get_id_details(id);
        let module = find_module(&json, _id.clone());

        let version = find_version(module.versions.clone(), _ver);
        println!("Downloading {}", module.name);
        println!("Version: {}\n", version.version);

        let path = &[
            get_downloads_dir(),
            [
                [version.version.clone(), module.id].join("-"),
                "zip".to_string(),
            ]
            .join("."),
        ]
        .join("/");

        download_from_url(client, version.zip_url, module.name, path).await;
    } else {
        let path = &[get_downloads_dir(), ["URL-File".to_string(), "zip".to_string()].join(".")].join("/");
        download_from_url(client, id.clone(), id, path).await;
    }
}
