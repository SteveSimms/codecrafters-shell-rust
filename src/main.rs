use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

enum Command {
    Exit,
    Echo(String), //carries the message to print
    Type(String), // pass optional path param to the type enum variant
    // Path(String),
    Unknown(String), //Carries the unknown command name
}

//Implement the parsing logic
impl Command {
    fn from_input(input: &str) -> Self {
        let trimmed_input = input.trim();

        //1. Handle Exit
        if trimmed_input == "exit" || trimmed_input == "exit 0" {
            return Command::Exit;
        }

        // 2. Handle Echo (reusing our fix from Part 1)
        if let Some(arg) = trimmed_input.strip_prefix("echo ") {
            return Command::Echo(arg.to_string());
        } else if trimmed_input == "echo" {
            return Command::Echo("".to_string());
        }
        // 4.: handle type or any future commands here

        if let Some(arg) = trimmed_input.strip_prefix("type ") {
            return Command::Type(arg.to_string());
        }

        //3. Fallback
        Command::Unknown(trimmed_input.to_string())
    }
}
//Here we are following what I call the Enum Implementation Pattern
fn main() {
    loop {
        //1. Print Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // 2. Read Input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 3. Parse & Execute
        let command = Command::from_input(&input);

        match command {
            Command::Exit => std::process::exit(0),
            Command::Echo(msg) => println!("{}", msg),

            // Handle the type command
            Command::Type(cmd_name) => match cmd_name.as_str() {
                "echo" | "exit" | "type" => {
                    println!("{} is a shell builtin", cmd_name);
                }
                _ => {
                    let path_var = env::var("PATH").unwrap_or_default();
                    let mut found = false;
                    for dir in path_var.split(':') {
                        let full_path = std::path::Path::new(dir).join(&cmd_name);
                        if full_path.exists() {
                            if let Ok(metadata) = full_path.metadata() {
                                let mode = metadata.permissions().mode();
                                if mode & 0o111 != 0 {
                                    println!("{} is {}", cmd_name, full_path.display());
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !found {
                        println!("{}: not found", cmd_name);
                    }
                }
            },

            Command::Unknown(cmd_name) => {
                // Only print if the user actually typed something
                if !cmd_name.is_empty() {
                    println!("{}: command not found", cmd_name);
                }
            }
        }
    }
}
