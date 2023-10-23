extern crate reqwest;

pub mod android_root;
pub mod cmd;
pub mod repo;
pub mod utils;
use crate::cmd::{
    download::download, info::info, install::install, search::search, upself::upself,
};
use crate::repo::Module;

use android_root::module_state;
use clap::{Parser, Subcommand};
use repo::Repo;
use std::io::Write;
use std::{
    fs::{self, File},
    path::Path,
    process::exit,
};

#[derive(Debug, Subcommand)]
enum SearchCommands {
    #[command(arg_required_else_help = true)]
    All { query: String },
    #[command(arg_required_else_help = true)]
    Id { query: String },
    #[command(arg_required_else_help = true)]
    Name { query: String },
    #[command(arg_required_else_help = true)]
    Author { query: String },
    #[command(arg_required_else_help = true, alias = "desc")]
    Description { query: String },
    #[command(arg_required_else_help = true, alias = "ver")]
    Version { query: String },
    #[command(arg_required_else_help = true, alias = "li")]
    License { query: String },
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Update MMRL CLI
    #[command(arg_required_else_help = true, aliases = &["sup", "up"])]
    Upself {
        /// Skip confirm
        #[arg(short, long)]
        yes: bool,
        /// Example: 0.1.0
        version: String,
    },
    /// View module infomation
    #[command(arg_required_else_help = true, aliases = &["view"])]
    Info {
        /// Give info from given module ids
        ids: Vec<String>,
    },
    /// Search through modules
    #[command(arg_required_else_help = true,  aliases = &["lookup", "find"])]
    Search {
        #[clap(subcommand)]
        commands: SearchCommands,
        // Downloads the modules from the given ids
        // query: String,
    },
    /// Download any module
    #[command(arg_required_else_help = true,  aliases = &["dl"])]
    Download {
        /// Downloads the modules from the given ids
        ids: Vec<String>,
    },
    /// Install any module
    #[command(arg_required_else_help = true,  aliases = &["add", "get", "fetch"])]
    Install {
        /// Skip confirm
        #[arg(short, long)]
        yes: bool,
        /// Also install requires of a module
        #[arg(short, long)]
        requires: bool,
        /// Installs selected modules
        ids: Vec<String>,
    },
    Enable {
        /// Enabled selected modules
        ids: Vec<String>,
    },
    Disable {
        /// Disabled selected modules
        ids: Vec<String>,
    },
    Remove {
        /// Remove selected modules
        ids: Vec<String>,
    },
}

/// Magisk Module Repo Loader CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
}

const REPOS_SOURCE: &str = "/data/adb/mmrl/repos.json";

fn setup() {
    let file_path = Path::new(REPOS_SOURCE); // Replace with the desired file path

    if !file_path.exists() {
        // Create all directories in the path if they don't exist
        if let Some(parent_dir) = file_path.parent() {
            if !parent_dir.exists() {
                if let Err(err) = fs::create_dir_all(parent_dir) {
                    eprintln!("Error creating directories: {}", err);
                    return;
                }
            }
        }

        // Create the file
        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file: {}", err);
                return;
            }
        };

        // You can write to the file if needed
        if let Err(err) = writeln!(file, "[\n\t\"https://raw.githubusercontent.com/ya0211/magisk-modules-alt-repo/main/json/modules.json\"\n]") {
            eprintln!("Error writing to file: {}", err);
        }
    }
}

async fn fetch_repos(url: String) -> Result<Repo, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.json().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    setup();
    let client = reqwest::Client::builder().build().unwrap();
    let args = Args::parse();
    let mut modules: Vec<Module> = vec![];

    let file = File::open(REPOS_SOURCE).expect("file should open read only");
    let contents: Vec<String> = serde_json::from_reader(file).unwrap();

    let mut tasks = vec![];

    for url in contents {
        let task = tokio::spawn(fetch_repos(url));
        tasks.push(task);
    }

    for task in tasks {
        let result = task.await.unwrap();
        modules.append(&mut result.unwrap().modules);
    }

    match args.commands {
        Commands::Info { ids } => {
            for id in ids {
                info(&modules.clone(), id).await;
            }
            exit(0);
        }
        Commands::Upself { yes, version } => {
            upself(client, version, yes).await;
            exit(0);
        }
        Commands::Search { commands } => match commands {
            SearchCommands::All { query } => {
                search(modules.clone(), |module| {
                    module.id.to_lowercase().contains(&query.to_lowercase())
                        || module.name.to_lowercase().contains(&query.to_lowercase())
                        || module
                            .description
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                        || module.author.to_lowercase().contains(&query.to_lowercase())
                        || module
                            .version
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Id { query } => {
                search(modules.clone(), |module| {
                    module.id.to_lowercase().contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Name { query } => {
                search(modules.clone(), |module| {
                    module.name.to_lowercase().contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Author { query } => {
                search(modules.clone(), |module| {
                    module.author.to_lowercase().contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Description { query } => {
                search(modules.clone(), |module| {
                    module
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Version { query } => {
                search(modules.clone(), |module| {
                    module
                        .version
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::License { query } => {
                search(modules.clone(), |module| {
                    module
                        .track
                        .license
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
        },
        Commands::Install { yes, requires, ids } => {
            for id in ids {
                install(client.clone(), yes, requires, &modules, id).await;
            }
            exit(0);
        }
        Commands::Enable { ids } => {
            for id in ids {
                let base_path = Path::new("/data/adb/modules").join(id);
                let disable = base_path.join("disable");
                let remove = base_path.join("remove");

                if disable.exists() {
                    fs::remove_file(disable).expect("File delete failed");
                }

                if remove.exists() {
                    fs::remove_file(remove).expect("File delete failed");
                }
            }
        }
        Commands::Disable { ids } => {
            for id in ids {
                module_state(id, "disable");
            }
        }
        Commands::Remove { ids } => {
            for id in ids {
                module_state(id, "remove");
            }
        }
        Commands::Download { ids } => {
            for id in ids {
                download(client.clone(), &modules, id).await;
            }
            exit(0);
        }
    }
}
