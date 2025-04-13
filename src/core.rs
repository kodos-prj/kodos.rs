use mlua::prelude::*;
use std::io::Result;
use std::process::Output;
// std::result::Result
use std::{fs, path::PathBuf, process::Command};

use crate::{arch::Arch, distribution::Distribution};

pub struct Context {
    pub user: String,
    pub mount_point: PathBuf,
    pub use_chroot: bool,
    pub stage: String, // TODO: Change to use enem
}

pub fn exec(cmd: &str, args: Vec<&str>) -> Result<String> {
    let result = Command::new(cmd)
        .args(args)
        .output()
        .expect("ls command failed to start");
    let res = String::from_utf8(result.stdout).expect("Error");
    println!("{}", &res);
    Ok(res)
}

pub fn chroot_exec(cmd: &str, args: Vec<&str>, mount_point: &PathBuf) -> Result<String> {
    let mp = mount_point.to_str().unwrap();
    let mut new_args = vec![mp, cmd];
    new_args.extend(args);
    exec("chroot", new_args)
}

pub fn load_config(lua: &Lua, config_file: PathBuf) -> LuaResult<LuaTable> {
    if !config_file.exists() {
        return Err(LuaError::external("Configuration file does not exist"));
    }

    // let config_dir_path = config_file
    //     .parent()
    //     .expect("Could not get parent directory of the configuration file");

    let config_dir_path = config_file
        .parent()
        .unwrap()
        .canonicalize()
        .expect("Could not get canonical path of the configuration file");

    let lib_path = PathBuf::from(file!())
        .canonicalize()?
        .parent()
        .unwrap()
        .join("lib");

    // Add the current directory and lib location to the package path
    lua.load(format!(
        "package.path = '{}/?.lua;{}/?.lua;' .. package.path",
        config_dir_path.to_str().unwrap(),
        lib_path.to_str().unwrap()
    ))
    .exec()?;

    // Adding default libraries
    let default_libs = r#"
    list = require("utils").list
    map = require("utils").map
    If = require("utils").if_true
    IfElse = require("utils").if_else
    "#;

    lua.load(default_libs).exec()?;

    // Read the configuration file
    let contents = fs::read_to_string(config_file).expect("Problems reading the config file");
    // println!("{}", contents);

    // Load configuration
    let code = lua.load(contents);
    let conf = code.eval::<LuaTable>().expect("Evaluation problems");

    println!("Loaded configuration: {:?}", conf);
    // let repo_table = globals.get::<_, LuaTable>("repo");

    for pair in conf.clone().pairs::<String, mlua::Value>() {
        let (key, value) = pair?;
        println!("Key: {}, Value: {:?}", key, value);
    }

    // println!("-----------------------");
    // conf.get::<LuaTable>("locale")
    //     .clone()?
    //     .for_each(|k: String, v: mlua::Value| Ok(println!("K:{k} = V:{:?}", v)))?;

    // let pkgs = conf.get::<LuaTable>("packages").clone()?;

    // let mut vpkgs = Vec::new();
    // pkgs.sequence_values::<String>()
    //     .for_each(|s| vpkgs.push(s.unwrap()));

    // println!("{:?}", vpkgs);

    // println!("{:?}", conf);
    Ok(conf.clone())
}

pub fn create_partitions(conf: &LuaTable) -> (String, String, Vec<String>) {
    println!("In create_partition");
    let res = conf
        .clone()
        .get::<LuaTable>("boot")
        .clone()
        .unwrap()
        .for_each(|k: String, v: mlua::Value| Ok(println!("K:{k} = V:{:?}", v)));
    // println!("{:?}", conf);
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    ("".to_string(), "".to_string(), vec![])
}

pub fn create_filesystem_hierarchy(
    _boot_partition: String,
    _root_partition: String,
    _partition_list: Vec<String>,
    mount_point: &PathBuf,
) -> Vec<String> {
    println!("In create_filesytem_hierarchy");
    vec![]
}

pub fn set_base_distribution(_base_dist: mlua::Value) -> impl Distribution {
    return Arch {};
}

pub fn configure_system(conf: &LuaTable, partition_list: &Vec<String>, mount_point: &PathBuf) {
    println!("Configure system");
}

pub fn setup_bootloader(conf: &LuaTable, partition_list: &Vec<String>, dist: &impl Distribution) {
    println!("Setup bootloader")
}

pub fn create_kod_user(mount_point: &PathBuf) {
    println!("Create kod user")
}

pub fn manage_packages(
    mount_point: &PathBuf,
    repos: &Vec<String>,
    stage: &str,
    pending_to_install: &Vec<String>,
    chroot: bool,
) {
    println!("Manage packages");
}

pub fn get_pending_packages(packages_to_install: &Vec<String>) -> Vec<String> {
    println!("Get pending packages");
    vec![]
}

pub fn get_packages_to_install(conf: &LuaTable) -> (Vec<String>, Vec<String>) {
    println!("Get packages to install/remove");
    (vec![], vec![])
}

pub fn get_services_to_enable(ctx: &Context, conf: &LuaTable) -> Vec<String> {
    println!("Get services to enable");
    vec![]
}

pub fn enable_services(system_services_to_enable: &Vec<String>, use_chroot: bool) {
    println!("Enabling services");
}

pub fn proc_users(ctx: &Context, conf: &LuaTable) {
    println!("Processing users");
}

pub fn store_packages_services(
    gen_path: &PathBuf,
    packages_to_install: &Vec<String>,
    system_services_to_enable: &Vec<String>,
) {
    println!("Storing package and service information for the generation");
}
