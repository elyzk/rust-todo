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
            todos::create_todo(name, false).unwrap()
        }
        _ => (),
    }
}
