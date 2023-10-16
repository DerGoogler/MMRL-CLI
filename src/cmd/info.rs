use std::path::Path;

use ini::Ini;

use crate::repo::{find_module, Repo};

pub async fn info(json: &Repo, id: String) {
    let module = find_module(&json, id);
                
    let _id = &module.id;

    let moduleprop = Path::new("/data/adb/modules/").join(_id).join("module.prop");
    println!("\x1B[1mName:\x1B[0m {}", module.name);
    println!("\x1B[1mAuthor:\x1B[0m {}", module.author);
    if moduleprop.exists() {
        let conf = Ini::load_from_file(moduleprop.to_str().unwrap()).unwrap();
        let prop = conf.section(None::<String>).unwrap();
        println!("\x1B[4m\x1B[1mInstalled version: \x1B[34m{} \x1B[33m(\x1B[32m{}\x1B[33m)\x1B[0m", prop.get("version").unwrap(), prop.get("versionCode").unwrap()) 
    }
    println!(
        "\x1B[1mLatest version (Cloud):\x1B[0m \x1B[4m\x1B[34m{}\x1B[0m \x1B[33m(\x1B[32m{}\x1B[33m)\x1B[0m",
        module.version,
        module.version_code.to_string()    
    );
    println!("\x1B[1mDescription:\x1B[0m {}", module.description);
    println!( "\x1B[1mLicense:\x1B[0m \x1B[36m{}\x1B[0m", module.track.license);
    println!("\x1B[2mModule id: {}\x1B[0m\n", _id);
}