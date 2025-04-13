use std::{collections::HashMap, path::PathBuf};

use mlua::prelude::*;

use crate::distribution::Distribution;

pub struct Arch {}

impl Distribution for Arch {
    fn prepare_for_installation(&self) {
        println!("Arch preparation");
    }

    fn get_base_packages(&self, conf: &LuaTable) -> Vec<String> {
        println!("Get base packages for Arch");
        vec![]
    }

    fn install_essentials_pkgs(&self, base_packages: &Vec<String>, mount_point: &PathBuf) {
        println!("Install_essentials_pkgs for Arch");
    }

    fn proc_repos(
        &self,
        conf: &LuaTable,
        mount_point: &PathBuf,
    ) -> (Vec<String>, HashMap<String, Vec<String>>) {
        println!("Proc repos");
        // HashMap::<String, Vec<String>>::new()
        (Vec::new(), HashMap::new())
    }

    fn generale_package_lock(&self, mount_point: &PathBuf, gen_path: &PathBuf) {
        println!("Generate package lock for Arch");
    }
}
