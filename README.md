## todo-cli (WIP)

### Build
`cargo build`

### Run
`./target/debug/todo-cli`

#### Options
`todo-cli -a TODO` Add a new todo

`todo-cli -l` List all Todos

`todo-cli -l -f STATUS` List Todos filtered by `STATUS` (done, open)

`todo-cli -d ID` Mark todo with `ID` as done