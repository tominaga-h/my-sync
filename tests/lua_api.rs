use mlua::Lua;
use my_sync::lua::{TaskType, inject_package};

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

#[test]
fn test_fn_task_add_integration() {
    let lua = Lua::new();
    inject_package(&lua).unwrap();

    let lua_script = r#"
        local sync = require("my-sync")
        return sync.task.add("Install")
    "#;

    let result: TaskType = lua.load(lua_script).eval().unwrap();

    assert_eq!(result, TaskType::Install);
}
