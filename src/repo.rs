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
    pub require: Option<Vec<String>>,
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

pub(crate) fn find_module(modules: &Vec<Module>, id: String) -> Module {
    let module_exists = modules.iter().any(|m| m.id == id);
    if !module_exists {
        eprintln!("Unable to find {}", id);
        exit(1);
    }
    let module_pos = modules.iter().position(|m| m.id == id).unwrap();
    return modules[module_pos].clone();
}

pub(crate) fn find_version(versions: Vec<Version>, version_name: String) -> Version {
    if version_name.to_lowercase() == "latest" {
        return versions.last().unwrap().clone();
    } else {
        let version_exists = versions.iter().any(|v| v.version == version_name);
        if !version_exists {
            println!("Unable to find {}", version_name);
            exit(1);
        }
        let version_pos = versions
            .iter()
            .position(|v| v.version == version_name)
            .unwrap();
        return versions[version_pos].clone();
    }
}

pub fn get_id_details(id: String) -> (String, String) {
    let parts: Vec<&str> = id.split('@').collect();
    let _id = parts.get(0).unwrap();
    let _ver = parts.get(1).unwrap_or(&"latest");
    return (_id.to_string(), _ver.to_string())
 }