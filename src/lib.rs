use std::collections::HashMap;

pub trait CommandRunner {
    fn run(&mut self);
    fn usage(&self);
    fn help(&mut self, args: Vec<String>);
}

pub struct Command {
    name: String,
    description: String,
    run: fn(&mut Commands,Vec<String>),
}

pub struct Commands {
    args: Vec<String>,
    commands: HashMap<String, Command>,
}

impl CommandRunner for Commands{
    fn run(&mut self) {
        if self.args.len() < 2 {
            self.usage();
            return;
        }
        let args = self.args[2..].to_vec();
        let command_name = &self.args[1];
        let command = self.commands.get(command_name);
        match command {
            Some(command) => (command.run)(self, args),
            None => {
                println!("Command not found: {}", command_name);
                self.usage();
            }
        }
    }
    fn help(&mut self,_args: Vec<String>) {
        if self.args.len() <= 2 {
            self.usage();
            return;
        }
        for arg in &self.args[2..] {
            self.command_usage(arg);
        }
    }
    fn usage(&self) {
        let mut args_iter = self.args.iter();
        let program = args_iter.next().unwrap();
        let mut commands: Vec<_>  = self.commands.keys().collect();
        commands.sort();
        println!("Usage: {} <command> [options]", program);
        println!("Commands:");
        for command in commands {
            if let Some(cmd) = self.commands.get(command){
                println!("    {:<12} {}",cmd.name, cmd.description);
            }
        }
    }
}

impl Commands {
    pub fn new(args: Vec<String>) -> Self {
        let mut commands  = Commands {
            commands: HashMap::new(),
            args,
        }; 
        commands.create("help", "Print help", Commands::help);
        commands
    }

    pub fn create(&mut self,name: &str, description: &str, run: fn(&mut Commands, Vec<String>)) {
        let command = Command {
            name : name.to_string(),
            description : description.to_string(),
            run
        };
        self.commands.insert(name.to_string(), command);
    }

    pub fn command_usage(&self, command_name: &str) {
        let command = self.commands.get(command_name);
        match command {
            Some(command) => {
                println!("Usage: {} - {}", command.name, command.description);
            }
            None => {
                println!("Command not found: {}", command_name);
                self.usage();
            }
        }
    }

}
