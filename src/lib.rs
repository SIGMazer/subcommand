use std::collections::HashMap;

pub mod command;

use command::{Command, CommandFn, CommandRunner, Commands};

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                matrix[i][j] = j;
            } else if j == 0 {
                matrix[i][j] = i;
            } else {
                let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                    0
                } else {
                    1
                };

                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost)
            }
        }
    }
    matrix[len1][len2]
}

impl CommandRunner for Commands {
    fn run(&mut self) {
        if self.args.len() < 2 {
            self.usage();
            return;
        }
        let args = self.args[2..].to_vec();
        let command_name = &self.args[1];
        let command = self.commands.get(command_name);
        match command {
            Some(command) => {
                if command.name.eq("help") {
                    self.help();
                    return;
                }
                match command.run {
                    Some(run) => {
                        let result = run(args);
                        match result {
                            Ok(_) => {}
                            Err(err) => {
                                self.command_usage(command_name);
                                println!("Error: {}", err);
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {
                if !self.suggest_closest_subcommand_if_exist(command_name.to_string()) {
                    println!("Command not found: {}", command_name);
                    self.usage();
                }
            }
        }
    }
    fn usage(&self) {
        let mut args_iter = self.args.iter();
        let program = args_iter.next().unwrap();
        let mut commands: Vec<_> = self.commands.keys().collect();
        commands.sort();
        println!("Usage: {} <command> [options]", program);
        println!("Commands:");
        for command in commands {
            if let Some(cmd) = self.commands.get(command) {
                println!("    {:<12} {}", cmd.name, cmd.description);
            }
        }
    }
    fn help(&self) {
        if self.args.len() <= 2 {
            self.usage();
            return;
        }
        for arg in &self.args[2..] {
            self.command_usage(arg);
        }
    }
}

impl Commands {
    pub fn new(args: Vec<String>) -> Self {
        let mut commands = Commands {
            commands: HashMap::new(),
            args,
        };
        let command = Command {
            name: "help".to_string(),
            description: "Print help".to_string(),
            run: None,
        };
        commands.commands.insert(command.name.to_string(), command);
        commands
    }

    pub fn create(&mut self, name: &str, description: &str, run: CommandFn) {
        let command = Command {
            name: name.to_string(),
            description: description.to_string(),
            run: Some(run),
        };
        self.commands.insert(name.to_string(), command);
    }

    fn suggest_closest_subcommand_if_exist(&self, command: String) -> bool {
        let commands: Vec<_> = self.commands.keys().collect();
        let mut closest_commands: Vec<String> = Vec::new();
        for c in commands {
            let d = levenshtein_distance(&command, c);
            if d < 2 {
                closest_commands.push(c.to_owned());
            }
        }
        if closest_commands.len() > 0 {
            println!("May you mean:");
            for c in closest_commands {
                println!("    {}", c)
            }
            return true;
        }
        false
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commands() {
        let mut commands = Commands::new(vec!["program".to_string()]);

        commands.create("cmd1", "Description 1", |args| {
            println!("Command 1 executed with args: {:?}", args);
            Ok(0)
        });

        commands.create("cmd2", "Description 2", |args| {
            println!("Command 2 executed with args: {:?}", args);
            Ok(0)
        });

        // Test usage
        commands.usage();

        // Test help
        commands.args.push("help".to_string());
        commands.help();

        // Test running a command
        commands.args = vec![
            "program".to_string(),
            "cmd1".to_string(),
            "arg1".to_string(),
        ];
        commands.run();

        // Test running an unknown command
        commands.args = vec!["program".to_string(), "unknown_cmd".to_string()];
        commands.run();

        // Test running a command with insufficient arguments
        commands.args = vec!["program".to_string(), "cmd1".to_string()];
        commands.run();
    }
}
