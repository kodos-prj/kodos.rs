// Distribution

use std::{collections::HashMap, path::PathBuf};

use mlua::prelude::*;

pub trait Distribution {
    fn prepare_for_installation(&self);
    fn get_base_packages(&self, conf: &LuaTable) -> Vec<String>;
    fn install_essentials_pkgs(&self, base_packages: &Vec<String>, mount_point: &PathBuf);
    fn proc_repos(
        &self,
        conf: &LuaTable,
        mount_point: &PathBuf,
    ) -> (Vec<String>, HashMap<String, Vec<String>>);
    fn generale_package_lock(&self, mount_point: &PathBuf, gen_path: &PathBuf);
}
