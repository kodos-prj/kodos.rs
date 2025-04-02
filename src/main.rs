// Main command to interact with the KodOS system
// @Author: Anatal Buss
// @version 0.1

use std::env;

use mlua::prelude::*;
use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};
use kod::actions::{install, rebuild, rebuild_user, Action};

fn main() -> LuaResult<()> {
    let matches = command!()
        .arg(
            arg!(
                -d --debug ... "Turn debugging information on"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -v --verbose ... "Verbose in the commands executed"
            )
            .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("install")
                .about("Install KodOS following the given configuration")
                .arg(
                    arg!(-c --config <FILE> "Sets a custom config file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("rebuild")
                .about("Rebuild KodOS installation based on configuration file")
                .arg(
                    arg!(-c --config <FILE> "Sets a custom config file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!(-n --new ... "Creates a new generation").action(ArgAction::SetTrue))
                .arg(arg!(-u --update ... "Update installed packages").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("rebuild-user")
                .about("Rebuild user configuration")
                .arg(
                    arg!(-c --config <FILE> "Sets a custom config file")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    println!("verbose: {:?}", matches.get_flag("verbose"));

    let action: Action = match matches.subcommand() {
        Some(("install", sub_matches)) => {
            print!("install ");
            if let Some(config) = sub_matches.get_one::<PathBuf>("config") {
                println!("using {:?}", config.display());
                Action::Install {
                    config: config.clone(),
                }
            } else {
                println!("using /etc/kodos");
                Action::Install {
                    config: PathBuf::from("/etc/kodos/configuration.lua"),
                }
            }
        }
        Some(("rebuild", sub_matches)) => {
            print!("rebuild ");
            if let Some(config) = sub_matches.get_one::<PathBuf>("config") {
                println!("using {:?}", config.display());
                Action::Rebuild {
                    config: config.clone(),
                    new_generation: sub_matches.get_flag("new"),
                    update: sub_matches.get_flag("update"),
                }
            } else {
                println!("using /etc/kodos/");
                Action::Rebuild {
                    config: PathBuf::from("/etc/kodos/configuration.lua"),
                    new_generation: sub_matches.get_flag("new"),
                    update: sub_matches.get_flag("update"),
                }
            }
        }
        Some(("rebuild-user", sub_matches)) => {
            print!("rebuild-user ");
            if let Some(config) = sub_matches.get_one::<PathBuf>("config") {
                println!("using {:?}", config.display());
                Action::RebuildUser {
                    config: config.clone(),
                }
            } else {
                println!("using /etc/kodos");
                Action::RebuildUser {
                    config: PathBuf::from("/etc/kodos/configuration.lua"),
                }
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    // --------------------------------------------------

    println!("{:?}", action);

    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let use_verbose = matches.get_flag("verbose");

    match action {
        Action::Install { config } => {
            println!("Installing KodOS using configuration file: {:?}", config);
            install(config, use_verbose)
        }
        Action::Rebuild {
            config,
            new_generation,
            update,
        } => {
            println!(
                "Rebuilding KodOS using configuration file: {:?}, new_generation: {}, update: {}",
                config, new_generation, update
            );
            rebuild(config, new_generation, update, use_verbose)
        }
        Action::RebuildUser { config } => {
            println!(
                "Rebuilding user configuration using configuration file: {:?}",
                config
            );
            rebuild_user(config, use_verbose)
        }
    }

    // // TODO: Replace to use a proper argument parsing
    // let mut config_file = "config.lua".to_string();
    // if &args.len() > &1 {
    //     config_file = args[1].clone();
    // }

    // let lua = Lua::new();
    // let globals = lua.globals();

    // let repo_table = lua.create_table()?;
    // let boot_table = lua.create_table()?;
    // let service_table= lua.create_table()?;
    // let program_table = lua.create_table()?;

    // globals.set("repo", &repo_table)?;
    // globals.set("boot", &boot_table)?;
    // globals.set("service", &service_table)?;
    // globals.set("program", &program_table)?;

    // let contents = fs::read_to_string(config_file)
    //     .expect("Should have been able to read the file");

    // // Load configuration
    // let code = lua.load(contents);
    // code.exec()?;

    // let repo_table = globals.get::<_, LuaTable>("repo")?;

    // // let repo_url = globals.get::<String, String>("url")?;
    // let repo_url = repo_table.get::<_, String>("url")?;
    // let repo = repo_table.get::<_, String>("repo")?;
    // let arch = repo_table.get::<_, String>("arch")?;
    // let db_filename = "core.db.tar.gz".to_string();
    // let db_url = format!("{repo_url}/{repo}/os/{arch}/{db_filename}");
    // println!("repo = {}", db_url);

    // get_index(db_url, "index".to_owned());

    // for pair in boot_table.pairs::<String, Value>() {
    //     let (key, value) = pair?;
    //     println!("{}: {:?}", key, value);
    // }

    // service_table.pairs().for_each(|p:Result::<(String, Value), _>| {
    //     let Ok((k,v)) = p else { todo!(); };
    //     println!("{}: {:?}", k, v);
    // });

    Ok(())
}
