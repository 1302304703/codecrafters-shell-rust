use std::io::{self, Write};
use std::path::Path;

const BUILTIN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let (cmd, args) = parse_command(input);
        execute_command(cmd, args);
    }
}

fn parse_command(input: &str) -> (&str, &str) {
    match input.split_once(' ') {
        Some((cmd, args)) => (cmd, args.trim()),
        None => (input, ""),
    }
}

fn execute_command(cmd: &str, args: &str) {
    match cmd {
        "exit" => std::process::exit(0),
        "echo" => println!("{}", args),
        "type" => handle_type(args),
        _ => println!("{}: command not found", cmd),
    }
}

fn handle_type(cmd: &str) {
    if BUILTIN_COMMANDS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    if let Some(path) = find_executable(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd);
    }
}

fn find_executable(cmd: &str) -> Option<String> {
    let path_env = std::env::var("PATH").ok()?;

    for dir in std::env::split_paths(&path_env) {
        let full_path = dir.join(cmd);
        if is_executable(&full_path) {
            return Some(full_path.to_string_lossy().to_string());
        }
    }

    None
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        metadata.is_file() && (permissions.mode() & 0o111 != 0)
    } else {
        false
    }
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}