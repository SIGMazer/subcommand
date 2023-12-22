# Subcommand

Subcommand it's library taht lets you easily add subcommand.

## Usage

```rust
use std::env;
use subcommands::command::{ Commands, CommandRunner,CommandResult};

fn print_next(args: Vec<String>) -> CommandResult {
    if args.len() < 1 {
        return Err("Not enough arguments".into());
    }
    for arg in args {
        println!("{}", arg);
    }
    Ok(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut commands = Commands::new(args);
    commands.create("cmd1", "Description 1", |args| {
            println!("Command 1 executed with args: {:?}", args);
            Ok(0)
      });

    commands.create("cmd2", "Description 2", |args| {
            println!("Command 2 executed with args: {:?}", args);
            Ok(0)
      });
    commands.create("print", "Print next word in new line",print_next);
    commands.run();
}

```
