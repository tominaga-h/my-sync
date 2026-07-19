use mlua::{Error, FromLua, Function, IntoLua, Lua, Result, Table, Value};
use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

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

// ----------------------------------------------

/// Struct of a task
#[allow(dead_code)]
pub struct Task {
    task_type: TaskType,
    name: String,
    callback: Function,
}

impl Task {
    pub fn new(task_type: TaskType, name: String, callback: Function) -> Self {
        Self {
            task_type,
            name,
            callback,
        }
    }
}

// ----------------------------------------------

/// Struct for management tasks
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task_type: TaskType, name: String, callback: Function) {
        self.tasks.push(Task::new(task_type, name, callback))
    }

    pub fn get_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------

fn setup_tasktype(lua: &Lua, module: &Table) -> Result<()> {
    let task_type = lua.create_table()?;

    task_type.set("Install", TaskType::Install)?;

    module.set("TaskType", task_type)?;

    Ok(())
}

fn setup_task_fns(lua: &Lua, module: &Table) -> Result<()> {
    // need RC & RefCell to mutably share the TaskManager across closures
    let task_manager = Rc::new(RefCell::new(TaskManager::new()));
    let task_capture = task_manager.clone();

    let lua_fn_add_task = lua.create_function(
        move |_, (task_type, name, callback): (TaskType, String, Function)| {
            task_capture
                .borrow_mut()
                .add_task(task_type, name, callback);
            Ok(())
        },
    )?;
    module.set("add_task", lua_fn_add_task)?;

    Ok(())
}

pub fn setup_task_module(lua: &Lua, module: &Table) -> Result<()> {
    setup_tasktype(lua, module)?;

    let task_module = lua.create_table()?;
    setup_task_fns(lua, &task_module)?;
    module.set("task", task_module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{Lua, Result};

    #[test]
    fn test_task_adding() {
        let lua = Lua::new();
        let test_fn = lua
            .create_function(|_, ()| -> Result<bool> { Ok(true) })
            .unwrap();

        let mut manager = TaskManager::new();
        manager.add_task(TaskType::Install, "test task".to_string(), test_fn);

        let task = manager.get_task(0).unwrap();
        assert_eq!(task.task_type, TaskType::Install);
        assert_eq!(task.name, "test task".to_string());
    }
}
