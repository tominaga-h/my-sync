use mlua::{Error, FromLua, IntoLua, Lua, Result, Table, Value};
use std::fmt::{self, Display, Formatter};

/// Type of the task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    Install,
}

impl FromLua for TaskType {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::String(task_type) => match &*task_type.to_str()? {
                "Install" => Ok(TaskType::Install),
                unknown => Err(Error::FromLuaConversionError {
                    from: "String",
                    to: "TaskType".to_string(),
                    message: Some(format!("Unknown task type: {}", unknown)),
                }),
            },
            _ => Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: "TaskType".to_string(),
                message: Some("Expected a string for TaskType".to_string()),
            }),
        }
    }
}

impl IntoLua for TaskType {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let task_type_str = match self {
            TaskType::Install => "Install",
        };
        Ok(Value::String(lua.create_string(task_type_str)?))
    }
}

impl Display for TaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let task_type_str = match self {
            TaskType::Install => "Install",
        };
        write!(f, "{}", task_type_str)
    }
}

// pub struct Task {}

fn fn_task_add(_: &Lua, task_type: TaskType) -> Result<TaskType> {
    println!("{}", task_type);
    Ok(task_type)
}

pub fn setup_task_module(lua: &Lua, module: &Table) -> Result<()> {
    let task_module = lua.create_table()?;

    let lua_fn_task_add = lua.create_function(fn_task_add)?;
    task_module.set("add", lua_fn_task_add)?;

    module.set("task", task_module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_fn_task_add() {
        let lua = Lua::new();
        let result = fn_task_add(&lua, TaskType::Install);
        if let Ok(task_type) = result {
            assert_eq!(task_type, TaskType::Install);
        }
    }
}
