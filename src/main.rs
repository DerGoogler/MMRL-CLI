extern crate reqwest;

pub mod android_root;
pub mod cmd;
pub mod repo;
pub mod utils;
use crate::cmd::{
    download::download, info::info, install::install, search::search, upself::upself,
};
use crate::repo::Module;

use clap::{Parser, Subcommand};
use repo::Repo;
use std::io::{Read, Write};
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
    #[command(arg_required_else_help = true, aliases = &["sup", "up"])]
    Upself {
        /// Skip confirm
        #[arg(short, long)]
        yes: bool,
        /// Example: 0.1.0
        version: String,
    },
    #[command(arg_required_else_help = true, aliases = &["view"])]
    Info {
        /// Give info from given module ids
        ids: Vec<String>,
    },
    #[command(arg_required_else_help = true,  aliases = &["lookup", "find"])]
    Search {
        #[clap(subcommand)]
        commands: SearchCommands,
        // Downloads the modules from the given ids
        // query: String,
    },
    #[command(arg_required_else_help = true,  aliases = &["dl"])]
    Download {
        /// Downloads the modules from the given ids
        ids: Vec<String>,
    },
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
    // Enable {
    //     /// Enabled selected modules
    //     ids: Vec<String>,
    // },
    // Disable {
    //     /// Disabled selected modules
    //     ids: Vec<String>,
    // },
    // Remove {
    //     /// Remove selected modules
    //     ids: Vec<String>,
    // },
}

/// Magisk Module Repo Loader CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
}

fn setup() {
    let file_path = Path::new("/data/adb/mmrl/repos.list"); // Replace with the desired file path

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
        if let Err(err) = writeln!(file, "https://raw.githubusercontent.com/ya0211/magisk-modules-alt-repo/main/json/modules.json;") {
            eprintln!("Error writing to file: {}", err);
        }
    }
}

#[tokio::main]
async fn main() {
    setup();
    let client = reqwest::Client::new();
    let args = Args::parse();
    let mut modules: Vec<Module> = vec![];
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Available repos:");
    for repo in contents.split(";") {
        let response = client.get(repo).send().await.unwrap();
        let mut json: Repo = response.json().await.unwrap();
        println!("{}", json.name);
        modules.append(&mut json.modules);
    }

    match args.commands {
        Commands::Info { ids } => {
            for id in ids {
                info(&modules, id).await;
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
        //         Commands::Enable { ids } => {
        //              let mut some_disabled= false;
        //             for id in ids {
        //                 let module = find_module(json.clone(), id);
        //                 let disable = &format!("/data/adb/modules/{}/disable", module.id);
        //                 if !Path::new(&disable).exists() {
        //                     if !File::create(disable).is_err() {
        //                         some_disabled = true;
        //                         println!("{} has been disabled.", module.name);
        //                     }
        //                 }
        //             }
        //             if !some_disabled {
        //                 println!("Nothing were disabled");
        //             }
        //         }
        // Commands::Disable { ids } => {
        //     let mut some_disabled= false;
        //     for id in ids {
        //         let module = find_module(&json, id);
        //         let disable = Path::new("/data/abd/modules").join(module.id).join("disable");
        //         if !disable.exists() {
        //             let mut f = File::create(disable).unwrap();
        //                 match f.write_all(b"") {
        //                     Ok(addr) => {
        //                         some_disabled = true;
        //                         println!("{} will be removed.", module.name);
        //                     },
        //                     Err(err) => {
        //                         println!("{}", err);
        //                         exit(1);
        //                     },
        //                 }
        //         }
        //     }
        //     if !some_disabled {
        //         println!("Nothing were disabled");
        //     }
        // }
        //         Commands::Remove { ids } => {
        //             let mut some_removed= false;
        //             for id in ids {
        //                 let module = find_module(&json, id);

        // //                 let remove = Path::new("/data/adb/modules/");

        // //                 let gg = remove.join(module.id).join("remove");
        // // println!("{:?}", gg);
        //                 // if !remove.exists() {
        //                 //     match fs::write(remove, b"Lorem ipsum") {
        //                 //         Ok(addr) => {
        //                 //             some_removed = true;
        //                 //             println!("{} will be removed.", module.name);
        //                 //         },
        //                 //         Err(_) => (),
        //                 //     }
        //                 // }
        //             }
        //             if !some_removed {
        //                 println!("Nothing were removed");
        //             }
        //         }
        Commands::Download { ids } => {
            for id in ids {
                download(client.clone(), &modules, id).await;
            }
            exit(0);
        }
    }
}
