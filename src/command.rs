use std::collections::HashMap;
use std::error::Error;

pub type ExitCode = i32;

pub type CommandResult = Result<ExitCode, Box<dyn Error>>;

pub type CommandFn = fn(Vec<String>) -> CommandResult;

pub trait CommandRunner {
    fn run(&mut self);
    fn usage(&self);
    fn help(&self);
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub run: Option<CommandFn>,
}

pub struct Commands {
    pub args: Vec<String>,
    pub commands: HashMap<String, Command>,
}
