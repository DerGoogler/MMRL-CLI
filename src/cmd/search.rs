use crate::repo::Repo;

pub async fn search(json: Repo, query: String) {
    print!("\x1B[1mFound these modules:\x1B[0m\n\n");
    for module in json.modules {
        if query == "all" || module.id.to_lowercase().contains(&query.to_lowercase())
            || module.name.to_lowercase().contains(&query.to_lowercase())
        {
            println!(
                "\x1B[36m\x1B[4m{}\x1B[0m {}\n",
                module.name,
                [
                    "\x1B[34m".to_string(),
                    module.version,
                    "\x1B[34m".to_string(),
                    " \x1B[34m(".to_string(),
                    "\x1B[33m".to_string(),
                    module.version_code.to_string(),
                    "\x1B[0m".to_string(),
                    "\x1B[0m)".to_string(),
                    " \x1B[94m[".to_string(),
                    module.track.license,
                    "]\x1B[0m".to_string(),
                    "\n".to_string(),
                    "\x1B[2mId: ".to_string(),
                    module.id,
                    "\x1B[0m".to_string()
                ]
                .join("")
            );
        }
    }
}
