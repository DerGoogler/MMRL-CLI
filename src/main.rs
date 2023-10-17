extern crate ini;
extern crate reqwest;

pub mod android_root;
pub mod cmd;
pub mod repo;

use crate::cmd::{download::download, info::info, install::install, search::search};

use clap::{Parser, Subcommand};
use repo::Repo;
use std::process::exit;

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
        /// Downloads the selected version
        #[arg(short, long, default_value_t = 0)]
        version: i64,
        /// Downloads the modules from the given ids
        ids: Vec<String>,
    },
    #[command(arg_required_else_help = true,  aliases = &["add", "get", "fetch"])]
    Install {
        /// Installs the selected version
        #[arg(short, long, default_value_t = 0)]
        version: i64,
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
    /// Were the modules comes from
    #[arg(short, long, default_value_t = String::from("https://raw.githubusercontent.com/ya0211/magisk-modules-alt-repo/main/json/modules.json"))]
    repo: String,

    #[clap(subcommand)]
    commands: Commands,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let args = Args::parse();

    let response = client.get(args.repo).send().await.unwrap();

    let json: Repo = response.json().await.unwrap();

    println!("\nSelected Repo: {}\n", json.name);
    // println!("Root manager: {}\n", &get_root_manager());

    match args.commands {
        Commands::Info { ids } => {
            for id in ids {
                info(&json, id).await;
            }
            exit(0);
        }
        Commands::Search { commands } => match commands {
            SearchCommands::All { query } => {
                search(json.clone(), |module| {
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
                search(json.clone(), |module| {
                    module.id.to_lowercase().contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Name { query } => {
                search(json.clone(), |module| {
                    module.name.to_lowercase().contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Author { query } => {
                search(json.clone(), |module| {
                    module.author.to_lowercase().contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Description { query } => {
                search(json.clone(), |module| {
                    module
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::Version { query } => {
                search(json.clone(), |module| {
                    module
                        .version
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                })
                .await;
                exit(0);
            }
            SearchCommands::License { query } => {
                search(json.clone(), |module| {
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
        Commands::Install { version, ids } => {
            for id in ids {
                install(client.clone(), version, &json, &id).await;
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
        Commands::Download { version, ids } => {
            for id in ids {
                download(client.clone(), version, &json, id).await;
            }
            exit(0);
        }
    }
}
