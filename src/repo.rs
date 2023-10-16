use std::process::exit;

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name: String,
    pub metadata: Metadata,
    pub modules: Vec<Module>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub version: i64,
    pub timestamp: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub id: String,
    pub name: String,
    pub version: String,
    pub version_code: i64,
    pub author: String,
    pub description: String,
    pub track: Track,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    #[serde(rename = "type")]
    pub type_field: String,
    pub added: f64,
    pub license: String,
    pub homepage: String,
    pub source: String,
    pub support: String,
    pub donate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub timestamp: f64,
    pub version: String,
    pub version_code: i64,
    pub zip_url: String,
    pub changelog: String,
}

pub(crate) fn find_module(json: &Repo, id: String) -> Module {
    let module_exists = json.modules.iter().any(|m| m.id == id);
    if !module_exists {
        println!("Unable to find {}", id);
        exit(1);
    }
    let module_pos = json.modules.iter().position(|m| m.id == id).unwrap();
    return json.modules[module_pos].clone();
}
