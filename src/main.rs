#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Exit,
    Echo(String),    //carries the message to print
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
        // todo: handle type or any future commands here
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
            Command::Unknown(cmd_name) => {
                // Only print if the user actually typed something
                if !cmd_name.is_empty() {
                    println!("{}: command not found", cmd_name);
                }
            }
        }
    }
}
