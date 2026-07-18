use mlua::{Lua, Result};

fn fn_println(_: &Lua, msg: String) -> Result<()> {
    println!("{}", msg);
    Ok(())
}

pub fn set_functions(lua: &Lua) -> Result<()> {
    // lua func: print lines
    let lua_fn_println = lua.create_function(fn_println)?;

    lua.globals().set("println", lua_fn_println)?;

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
