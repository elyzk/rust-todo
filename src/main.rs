use std::env;

use crate::command::{Command, MatchCommand, ValidCommands};
mod command;
mod todos;

#[derive(Debug)]
enum ProgramError {
    BadCommand,
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
                .with_handler(todos::create_todo)
                .set_args(1),
        )
        .add(
            Command::new("delete")
                .set_args(1)
                .with_handler(todos::delete_todo),
        )
        .add(
            Command::new("update")
                .set_args(2)
                .with_handler(todos::update_todo),
        )
        .add(
            Command::new("done")
                .set_args(1)
                .with_handler(todos::done_todo),
        )
        .add(
            Command::new("undone")
                .set_args(1)
                .with_handler(todos::undone_todo),
        )
        .add(Command::new("clear").with_handler(todos::clear_todos));
}
