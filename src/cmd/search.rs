use crate::repo::{Module, Repo};

pub async fn search(json: Repo, cb: impl Fn(&Module) -> bool) {
    print!("\x1B[1mFound these modules:\x1B[0m\n\n");
    for module in json.modules {
        let m = module.clone();
        if cb(&m) {
            println!(
                "\x1B[36m\x1B[4m{}\x1B[0m {}\n",
                m.name,
                [
                    "\x1B[34m".to_string(),
                    m.version,
                    "\x1B[34m".to_string(),
                    " \x1B[34m(".to_string(),
                    "\x1B[33m".to_string(),
                    m.version_code.to_string(),
                    "\x1B[0m".to_string(),
                    "\x1B[0m)".to_string(),
                    " \x1B[94m[".to_string(),
                    m.track.license,
                    "]\x1B[0m".to_string(),
                    "\n".to_string(),
                    "\x1B[2mId: ".to_string(),
                    m.id,
                    "\x1B[0m".to_string()
                ]
                .join("")
            );
        }
    }
}
