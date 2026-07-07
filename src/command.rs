use std::{collections::HashMap, io};

type CommandHandler = fn(Vec<String>) -> Result<(), io::Error>;

pub struct ValidCommands {
    commands: HashMap<String, Command>,
}

#[derive(Clone)]
pub struct Command {
    name: String,
    num_args: usize,
    handler: Option<CommandHandler>,
}

#[derive(Clone)]
pub struct Arg {
    name: String,
}

impl ValidCommands {
    pub fn new() -> Self {
        ValidCommands {
            commands: HashMap::new(),
        }
    }

    pub fn add(&mut self, command: Command) -> &mut Self {
        self.commands.insert(command.name.clone(), command.clone());
        self
    }

    pub fn get(&self, name: String) -> Option<Command> {
        self.commands.get(&name).cloned()
    }

    pub fn exists(&self, command: Command) -> bool {
        self.commands.contains_key(&command.name)
    }
}

impl Command {
    pub fn new(name: &str) -> Self {
        Command {
            name: String::from(name),
            num_args: 0,
            handler: None,
        }
    }

    pub fn set_args(mut self, num_args: usize) -> Self {
        self.num_args = num_args;
        self
    }

    pub fn num_args(&self) -> usize {
        self.num_args
    }

    pub fn with_handler(mut self, handler: fn(Vec<String>) -> Result<(), io::Error>) -> Self {
        self.handler = Some(handler);
        self
    }
}

impl Arg {
    pub fn new(name: &str) -> Self {
        Arg {
            name: String::from(name),
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    NoCommand,
    InvalidCommand,
    InvalidArgs,
}

pub struct MatchCommand {
    command: Command,
    args: Vec<String>,
}

impl MatchCommand {
    // TODO: should this go here?
    pub fn match_command(
        valid_commands: &ValidCommands,
        args: &mut Vec<String>,
    ) -> Result<Self, CommandError> {
        if args.len() == 0 {
            return Err(CommandError::NoCommand);
        }
        let command = args[0].clone();
        let command = match valid_commands.get(command) {
            Some(command) => command,
            None => return Err(CommandError::InvalidCommand),
        };

        if args.len() != command.num_args() + 1 {
            return Err(CommandError::InvalidArgs);
        }

        args.remove(0);

        Ok(MatchCommand {
            command,
            args: args.clone(),
        })
    }

    pub fn handle(&self) -> Result<(), io::Error> {
        let handler = self.command.handler.unwrap();
        handler(self.args.clone())
    }
}
//
// struct MatchArg {
//     arg: Arg,
//     value: String,
// }
