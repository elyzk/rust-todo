use std::{
    fs::{self, File},
    io::{self, BufWriter},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    name: String,
    done: bool,
}

impl Todo {
    fn new(name: &str, done: bool) -> Self {
        Self {
            name: name.to_string(),
            done,
        }
    }
}

static TODOS_FILE: &str = "todos.json";

pub fn list_todos() {
    let todos = read_todos_file();
    for todo in todos {
        println!("todo: {}, done: {}", todo.name, todo.done);
    }
}

pub fn create_todo(name: &str, done: bool) -> Result<(), io::Error> {
    let mut todos = read_todos_file();
    let todo = Todo::new(name, done);

    todos.push(todo);

    write_todos(todos)
}
pub fn delete_todo(name: &str) -> Result<(), io::Error> {
    let mut todos = read_todos_file();
    todos.retain(|todo| todo.name != name);

    write_todos(todos)
}
pub fn update_todo(name: &str, new_name: &str) -> Result<(), io::Error> {
    let mut todos = read_todos_file();
    todos.iter_mut().for_each(|todo| {
        if todo.name == name {
            (*todo).name = new_name.to_string();
        }
    });

    write_todos(todos)
}
pub fn done_todo(name: &str) -> Result<(), io::Error> {
    let mut todos = read_todos_file();
    todos.iter_mut().for_each(|todo| {
        if todo.name == name {
            (*todo).done = true;
        }
    });

    write_todos(todos)
}
pub fn undone_todo(name: &str) -> Result<(), io::Error> {
    let mut todos = read_todos_file();
    todos.iter_mut().for_each(|todo| {
        if todo.name == name {
            (*todo).done = false;
        }
    });

    write_todos(todos)
}
pub fn clear_todos() -> Result<(), io::Error> {
    let mut todos = read_todos_file();
    todos.retain(|todo| todo.done == false);
    write_todos(todos)
}

fn read_todos_file() -> Vec<Todo> {
    // Read the list of todos, or an empty list if the file does not exist
    let todos = match fs::read_to_string(TODOS_FILE) {
        Ok(todos) => todos,
        Err(_) => String::from("[]"),
    };
    // TODO: handle this error
    let todos: Vec<Todo> = serde_json::from_str(&todos).unwrap();
    todos
}

fn write_todos(todos: Vec<Todo>) -> Result<(), io::Error> {
    let todos_file = File::create(TODOS_FILE)?;
    let writer = BufWriter::new(todos_file);

    serde_json::to_writer(writer, &todos).unwrap();
    Ok(())
}
