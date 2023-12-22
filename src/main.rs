use subcommand::{Commands, CommandRunner};
use std::env;

fn print_next(commands: &mut Commands, args: Vec<String>) {
    if args.len() < 1 {
        commands.command_usage("print");
    }
    for arg in args {
        println!("{}", arg);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut commands = Commands::new(args);
    commands.create("hello", "Prints hello world",|_commands, _args| {
        println!("Hello world");
    });
    commands.create("foo", "Prints foo",|_commands, _args| {
        println!("foo");
    });
    commands.create("print", "Print next word in new line",print_next);
    commands.run();
}