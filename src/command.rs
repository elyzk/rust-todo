use std::{collections::HashMap, io};

type CommandHandler = fn(Vec<MatchArg>) -> Result<(), io::Error>;

pub struct ValidCommands {
    commands: HashMap<String, Command>,
}

#[derive(Clone)]
pub struct Command {
    name: String,
    args: Vec<Arg>,
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
            args: Vec::new(),
            handler: None,
        }
    }

    pub fn add_arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn get_args(&self) -> &Vec<Arg> {
        return &self.args;
    }

    pub fn with_handler(mut self, handler: CommandHandler) -> Self {
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
    by_pos: Vec<MatchArg>,
    by_name: HashMap<String, usize>, // stores the index of a named arg in `by_pos`
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

        if args.len() != command.get_args().len() + 1 {
            return Err(CommandError::InvalidArgs);
        }

        // TODO: should we make any assumptions about the state of the input vector?
        args.remove(0);

        let by_pos: Vec<_> = args
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let arg = command.get_args().get(i).unwrap().to_owned();
                MatchArg::new(arg, val.to_owned())
            })
            .collect();

        let mut by_name = HashMap::new();
        by_pos.iter().enumerate().for_each(|(i, match_arg)| {
            // TODO: handle duplicate arg names here? or disallow having duplicate names in Command
            by_name.insert(match_arg.arg.name.clone(), i);
        });

        Ok(MatchCommand {
            command,
            by_pos,
            by_name,
        })
    }

    pub fn handle(&self) -> Result<(), io::Error> {
        let handler = self.command.handler.unwrap();
        handler(self.by_pos.clone())
    }
}

#[derive(Clone)]
pub struct MatchArg {
    arg: Arg,
    value: String,
}

impl MatchArg {
    pub fn new(arg: Arg, value: String) -> Self {
        Self { arg, value }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}
