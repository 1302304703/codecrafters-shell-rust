#[allow(unused_imports)]
use std::io::{self, Write};

const VALID_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.starts_with("exit") {
            std::process::exit(0);
        } else if command.starts_with("echo") {
            println!("{}", command.replace("echo ", "").trim());
        } else if command.starts_with("type") {
            let cmd = &command[4..].trim();
            if VALID_COMMANDS.contains(&cmd) {
                println!("{} is a shell builtin", cmd);
            } else {
                println!("{}: not found", cmd);
            }
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}