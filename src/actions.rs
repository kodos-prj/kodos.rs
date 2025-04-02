//! Actions that can be performed by the CLI

use std::path::PathBuf;

#[derive(Debug)]
pub enum Action {
    Install {
        config: PathBuf,
    },
    Rebuild {
        config: PathBuf,
        new_generation: bool,
        update: bool,
    },
    RebuildUser {
        config: PathBuf,
    },
    // Shell,
}

pub fn install(config: PathBuf, use_verbose: bool) {
    println!("Installing KodOS using configuration file: {:?}", config);
    // todo!()
}

pub fn rebuild(config: PathBuf, new_generation: bool, update: bool, use_verbose: bool) {
    println!(
        "Rebuilding KodOS using configuration file: {:?} new generation: {}, update packages: {}",
        config, new_generation, update
    );
    // todo!()
}

pub fn rebuild_user(config: PathBuf, use_verbose: bool) {
    println!(
        "Rebuilding KodOS using user configuration file: {:?}",
        config
    );
    // todo!()
}
