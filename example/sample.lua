local sync = require("my-sync")
local task = sync.task
sync.println("Hello lua World!")
sync.println("What is Next?")
task.add(sync.TaskType.Install, "test install", function() end)
