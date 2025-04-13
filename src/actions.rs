//! Actions that can be performed by the CLI
// #[warn(unused_variables)]
use mlua::prelude::*;

use crate::core::{
    chroot_exec, configure_system, create_filesystem_hierarchy, create_kod_user, create_partitions,
    enable_services, exec, get_packages_to_install, get_pending_packages, get_services_to_enable,
    load_config, manage_packages, proc_users, set_base_distribution, setup_bootloader,
    store_packages_services, Context,
};
use crate::distribution::Distribution;
use std::env;
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

// --------------------------------------------------------------------------

pub fn install(config: PathBuf, _use_verbose: bool) {
    let lua = Lua::new();
    println!("Installing KodOS using configuration file: {:?}", config);
    let conf = load_config(&lua, config).expect("Error loading configuration");

    let base_distribution = conf
        .clone()
        .get::<mlua::Value>("base_distribution")
        .unwrap();
    println!("Base distribution: {:?}", base_distribution);

    let dist = set_base_distribution(base_distribution);

    // # Prepare enviroment to install
    dist.prepare_for_installation();

    println!("-------------------------------");
    let (boot_partition, root_partition, partition_list) = create_partitions(&conf);

    let mount_point = PathBuf::from("/mnt");
    let partition_list =
        create_filesystem_hierarchy(boot_partition, root_partition, partition_list, &mount_point);

    // # Install base packages and configure system
    let base_packages = dist.get_base_packages(&conf); //# TODO: this function requires a wrapper
    dist.install_essentials_pkgs(&base_packages, &mount_point); //# TODO: this function requires a wrapper
    configure_system(&conf, &partition_list, &mount_point);
    setup_bootloader(&conf, &partition_list, &dist);
    create_kod_user(&mount_point);

    // === Proc packages
    let (repos, repo_packages) = dist.proc_repos(&conf, &mount_point); // TODO: this function requires a wrapper
    let (packages_to_install, packages_to_remove) = get_packages_to_install(&conf);
    let pending_to_install = get_pending_packages(&packages_to_install);
    // println!("packages\n{:?}", &packages_to_install);
    manage_packages(&mount_point, &repos, "install", &pending_to_install, true);

    // === Proc services
    let ctx = Context {
        user: env::var("USER").unwrap(),
        mount_point: mount_point.clone(),
        use_chroot: true,
        stage: "install".to_string(),
    };
    let system_services_to_enable = get_services_to_enable(&ctx, &conf);
    println!("Services to enable: {:?}", system_services_to_enable);
    enable_services(&system_services_to_enable, true);

    // === Proc users
    println!("\n====== Creating users ======");
    proc_users(&ctx, &conf);

    println!("\n==== Deploying generation ====");
    let gen_path = mount_point.join("kod/generations/0");
    store_packages_services(&gen_path, &packages_to_install, &system_services_to_enable);
    dist.generale_package_lock(&mount_point, &gen_path);

    // exec(f"umount -R {mount_point}");
    let res = exec("ls", vec!["-ls"]);
    println!("{}", res.unwrap());
    //
    let mp = PathBuf::from("/mnt");
    let res = chroot_exec("ls", vec!["-ls"], &mp);
    println!("{}", res.unwrap());

    println!("Done");
    println!("TODO: Copy kod to the new installtion");

    // let res = conf
    //     .clone()
    //     .get::<LuaTable>("locale")
    //     .clone()
    //     .unwrap()
    //     .for_each(|k: String, v: mlua::Value| Ok(println!("K:{k} = V:{:?}", v)));
    // // println!("{:?}", conf);
    // println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    // println!("In install: {:?}", res);
}

// =============================================================================
pub fn rebuild(config: PathBuf, new_generation: bool, update: bool, _use_verbose: bool) {
    let lua = Lua::new();
    println!(
        "Rebuilding KodOS using configuration file: {:?} new generation: {}, update packages: {}",
        config, new_generation, update
    );
    let _conf = load_config(&lua, config);
    // todo!()
}

// =============================================================================
pub fn rebuild_user(config: PathBuf, _use_verbose: bool) {
    println!(
        "Rebuilding KodOS using user configuration file: {:?}",
        config
    );
    // todo!()
}
