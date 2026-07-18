use mlua::{Lua, Result, Table};

fn fn_println(_: &Lua, msg: String) -> Result<()> {
    println!("{}", msg);
    Ok(())
}

pub fn set_functions(lua: &Lua) -> Result<Table> {
    let module: Table = lua.create_table()?;

    // lua func: print lines
    let lua_fn_println = lua.create_function(fn_println)?;
    module.set("println", lua_fn_println)?;

    Ok(module)
}

pub fn inject_package(lua: &Lua) -> Result<()> {
    // Use package.preload to inject lua package
    let package: Table = lua.globals().get("package")?;
    let preload: Table = package.get("preload")?;

    let module: Table = set_functions(lua)?;

    // Lua needs a loader function for injection
    let module_capture = module.clone();
    let loader = lua.create_function(move |_, ()| Ok(module_capture.clone()))?;

    preload.set("my-sync", loader)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_fn_println() {
        let lua = Lua::new();
        let result = fn_println(&lua, "test".to_string());
        assert!(result.is_ok());
    }
}
