use mlua::{Lua, Result};

pub fn set_functions(lua: &Lua) -> Result<()> {
    // lua func: print lines
    let fn_println = lua.create_function(|_, msg: String| {
        println!("{}", msg);
        Ok(())
    })?;

    lua.globals().set("println", fn_println)?;

    Ok(())
}
