#[allow(unused_imports)]
use std::io::{self, Write};

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
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}