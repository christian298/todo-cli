mod todo;
mod todo_store;
mod ui;

use clap::arg_enum;
use structopt::StructOpt;
use todo_store::TodoStore;
use ui::draw_todo_list;

arg_enum! {
    #[derive(Debug, PartialEq)]
    pub enum Filter {
        Done,
        Open,
        All
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Todo CLI", about = "Todo CLI")]
struct Opt {
    #[structopt(short = "a", long = "add", default_value = "")]
    todo: String,

    #[structopt(short = "d", long = "done", default_value = "0")]
    done: u32,

    #[structopt(short = "f", long = "filter", possible_values = &Filter::variants(), case_insensitive = true, default_value = "All")]
    filter: Filter,

    #[structopt(short, long)]
    list: bool,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let todo_store = TodoStore::new();

    if opt.list {
        let todos = todo_store.read_todos().unwrap();
        draw_todo_list(todos, opt.filter).expect("Failed to draw todo list");
    }

    if !opt.todo.is_empty() {
        todo_store.add_todo(opt.todo, false);
    }

    if opt.done > 0 {
        todo_store.toggle_status(true, opt.done);
    }
}
