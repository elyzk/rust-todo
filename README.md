This is a toy project for learning purposes (my first application written in Rust). I intend it to simply be a CLI that the user can use to list, create, and update todos, using proper/idiomatic error handling and module organization. Todos shall be written to and read from a JSON file local to the project.

# Commands:
- `todo list`: Lists all todos, with an indicator as to whether they are complete
- `todo create <todo>`: Creates a new todo (must be unique)
- `todo delete <todo>`
- `todo update <todo> <new_todo>`
- `todo done <todo>`: Marks a todo as done
- `todo undone <todo>`: Marks a todo as undone
- `todo clear`: Deletes all completed todos

# Goals:
- A fully-featured command / CLI handling package written from scratch, inspired by the [clap](https://github.com/clap-rs/clap) library API.
- Handling raw input to implement a terminal-native UI: enable users to navigate among a list of todos, selecting and modifying them as they please.

Disclaimer: No generative AI was used in the creation of this project. ;)
