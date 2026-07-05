use std::env;

use crate::todos::Todo;

mod todos;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 4 {
        panic!("Incorrect number of arguments");
    }

    match args[1].as_str() {
        "list" => todos::list_todos(),
        "create" => {
            let name = &args[2];
            todos::create_todo(name, false).unwrap();
        }
        "delete" => {
            let name = &args[2];
            todos::delete_todo(name).unwrap();
        }
        "update" => {
            let name = &args[2];
            let new_name = &args[3];
            todos::update_todo(name, new_name).unwrap();
        }
        "done" => {
            let name = &args[2];
            todos::done_todo(name).unwrap();
        }
        "undone" => {
            let name = &args[2];
            todos::undone_todo(name).unwrap();
        }
        "clear" => todos::clear_todos().unwrap(),
        _ => (),
    }
}
