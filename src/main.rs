use std::env;

use crate::command::{Arg, Command, MatchCommand, ValidCommands};
mod command;
mod todos;

#[derive(Debug)]
enum ProgramError {
    BadCommand,
    // TODO: more fine-grained error handling
    FileIOError,
    JSONError,
}

fn main() -> Result<(), ProgramError> {
    let mut valid_commands = ValidCommands::new();
    init_commands(&mut valid_commands);

    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 4 {
        return Err(ProgramError::BadCommand);
    }

    args.remove(0);

    let match_command = MatchCommand::match_command(&valid_commands, &mut args)
        .map_err(|_| ProgramError::BadCommand)?;

    match_command.handle().map_err(|_| ProgramError::BadCommand);

    Ok(())
}

fn init_commands(valid_commands: &mut ValidCommands) {
    valid_commands
        .add(Command::new("list").with_handler(todos::list_todos))
        .add(
            Command::new("create")
                .add_arg(Arg::new("name"))
                .with_handler(todos::create_todo),
        )
        .add(
            Command::new("delete")
                .add_arg(Arg::new("name"))
                .with_handler(todos::delete_todo),
        )
        .add(
            Command::new("update")
                .add_arg(Arg::new("old"))
                .add_arg(Arg::new("new"))
                .with_handler(todos::update_todo),
        )
        .add(
            Command::new("done")
                .add_arg(Arg::new("name"))
                .with_handler(todos::done_todo),
        )
        .add(
            Command::new("undone")
                .add_arg(Arg::new("name"))
                .with_handler(todos::undone_todo),
        )
        .add(Command::new("clear").with_handler(todos::clear_todos));
}
