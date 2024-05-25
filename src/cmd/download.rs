use crate::{
    android_root::get_downloads_dir,
    repo::{find_module, find_version, get_id_details, Module},
    utils::{download_from_url, get_last, is_url},
};
use reqwest::Client;

pub async fn download(client: Client, modules: &Vec<Module>, id: String) -> () {
    let _url = &id.to_owned()[..];
    if is_url(_url) {
        let name = get_last(_url);
        let path = &[
            get_downloads_dir(),
            [name.clone().unwrap().to_string(), "zip".to_string()].join("."),
        ]
        .join("/");
        download_from_url(client, id.clone(), id, path).await;
    } else {
        let (_id, _ver) = get_id_details(id);
        let module = find_module(&modules, _id.clone());

        let version = find_version(module.versions.clone(), _ver);

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
    }
}
