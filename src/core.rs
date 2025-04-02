use mlua::prelude::*;
use std::path::PathBuf;

fn local_config(config: PathBuf) -> LuaResult<()> {
    if !config.exists() {
        return Err(LuaError::external("Configuration file does not exist"));
    }
    // let contents = fs::read_to_string(config_file).expect("Problems reading the config file");

    // let mut config_file = File::open(&config)?;
    // let mut config_str = String::new();
    // config_file.read_to_string(&mut config_str)?;
    // let lua = Lua::new();
    // lua.context(|ctx| {
    //     ctx.load(&config_str).set_name("config")?.exec()?;
    //     let config: Table = ctx.globals().get("config")?;
    //     }

    // let contents = fs::read_to_string(config_file)
    //     .expect("Should have been able to read the file");

    // // Load configuration
    // let code = lua.load(contents);
    // code.exec()?;
    Ok(())
}
