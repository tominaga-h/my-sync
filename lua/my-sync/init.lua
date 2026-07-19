--- @meta
--- @diagnostic disable: unused-local

--- @class MySyncModule
local M = {}

--- Print message
--- @param msg string Message
function M.println(msg) end

--- @enum TaskType
local TaskType = {
	Install = "Install",
}
M.TaskType = TaskType

--- @class Task
local Task = {}

--- Add a task
--- @param task_type TaskType
--- @param name string
--- @param callback fun() A function to be called when the task is executed
function Task.add(task_type, name, callback) end

M.task = Task

return M
