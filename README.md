# pplanner
pplanner stands for Personal Planner.
It is a CLI/TUI tool to manage your time.
## build/install/usage
pplanner is made in rust.
To build it you need the rust tool chain.
In the directory pplanner you can run things like `cargo build --release`.
To install, run the script `install.sh`.
Help: `man pplanner`, `pplanner --help`.
Example: `pplanner ls points`
## features
- Points: Points in time like deadlines. Sorted on date with relative date for easy overview.
- Todos: Todo list, sorted on urgency, with 3 catogories: todo,longterm,idea.
### commands
- \_missing_help
- \_test_keys
- clean points
- clean slices
- clean todos
- edit plans
- edit points
- edit slices
- flush files
- help
- inspect point
- inspect slice
- license
- ls commands
- ls days
- ls months
- ls plans
- ls plans archive
- ls points
- ls points archive
- ls slices
- ls slices archive
- ls todos
- ls todos archive
- mk plan
- mk point
- mk slice
- mk todo
- mv plans
- now
- rm plans
- rm points
- rm slices
- rm todos
- status
- tick todos
