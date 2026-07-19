--- @meta
--- @diagnostic disable: unused-local

--- @class MySyncModule
local M = {}

--- Print message
--- @param msg string Message
function M.println(msg)
	--- Rust側で実装
end

--- @enum TaskType
local TaskType = {
	Install = "Install",
}
M.TaskType = TaskType

--- @class Task
local Task = {}

--- Add a task
--- @param task_type TaskType
function Task.add(task_type) end

M.task = Task

return M
