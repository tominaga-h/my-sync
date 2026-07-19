use mlua::{Lua, Result, Table};

fn fn_println(_: &Lua, msg: String) -> Result<()> {
    println!("{}", msg);
    Ok(())
}

fn fn_add(_: &Lua, (a, b): (u32, u32)) -> Result<u32> {
    Ok(a + b)
}

/// set lua functions to module
///
/// note:
/// There is no need to return the module because `mlua::Table` behaves like a reference,
///  so just setting it will change its contents.
pub fn set_functions(lua: &Lua, module: &Table) -> Result<()> {
    // lua function: print lines
    let lua_fn_println = lua.create_function(fn_println)?;
    module.set("println", lua_fn_println)?;

    // lua function: add
    let lua_fn_add = lua.create_function(fn_add)?;
    module.set("add", lua_fn_add)?;

    Ok(())
}

pub fn setup_module(lua: &Lua) -> Result<Table> {
    let module: Table = lua.create_table()?;

    set_functions(lua, &module)?;

    Ok(module)
}

pub fn inject_package(lua: &Lua) -> Result<()> {
    // Use package.preload to inject lua package
    let package: Table = lua.globals().get("package")?;
    let preload: Table = package.get("preload")?;

    let module: Table = setup_module(lua)?;

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

    #[test]
    fn test_fn_add() {
        let lua = Lua::new();
        let result = fn_add(&lua, (1, 1));
        assert!(result.is_ok());
        assert!(matches!(result, Ok(2)));
    }

    #[test]
    fn test_fn_add_integration() {
        let lua = Lua::new();
        inject_package(&lua).unwrap();

        let lua_script = r#"
            local sync = require("my-sync")
            return sync.add(1, 2)
        "#;

        let result: u32 = lua.load(lua_script).eval().unwrap();

        assert_eq!(result, 3);
    }
}
