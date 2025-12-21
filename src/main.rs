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

        //3. Fallback
        Command::Unknown(trimmed_input.to_string())
    }
}

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
// fn main() {
//     // shell_prompt();
//     repl()
// }

// fn shell_prompt() {
//     print!("$ ");
//     io::stdout().flush().unwrap();
//     println!("{}", read_input());
// }

// fn read_input() -> String {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//     let cmd = input.trim();
//     // let cmd_not_found = format!("{}: command not found", cmd);

//     if cmd == "exit" {
//         std::process::exit(0);
//     }

//     if cmd.starts_with("echo") {
//         // echo the input
//         //strips the echo prefix from command using pattern matching
//         //todo: find out why there is an extra space in the pattern
//         let stripped_echo = cmd.strip_prefix("echo").unwrap_or(&cmd); //why is there an extra space

//         return format!("{}", stripped_echo);
//     }

//     // let stripped_echo = cmd.strip_prefix("echo").unwrap_or(&cmd);
//     // return match cmd {
//     //     "echo" => format!("{}", cmd.strip_prefix("echo").unwrap_or(&cmd)),
//     //     _ => cmd_not_found,
//     // };

//     // return match cmd {
//     //     "echo" => String::new(),
//     //     _ if cmd.starts_with("echo ") => cmd
//     //         .strip_prefix("echo ")
//     //         .map(|s| s.to_string())
//     //         .unwrap_or_else(|| String::new()),
//     //     _ => cmd_not_found,
//     // };
//     //

//     // if cmd != "echo" {
//     //     return cmd_not_found;
//     // } else if cmd.starts_with("echo ") {
//     //     let stripped_echo = cmd.strip_prefix("echo").unwrap_or(&cmd); //why is there an extra space

//     //     return format!("{}", stripped_echo);
//     // } else {
//     //     let stripped_empty_str = String::new();
//     //     return stripped_empty_str
//     //         .to_string()
//     //         .split_off(0)
//     //         .strip_prefix("")
//     //         .unwrap_or(&stripped_empty_str)
//     //         .to_string()
//     //         .to_string();
//     // }
//     //
//     //

//     // REFACTOR NEW CODE INCOMING
//     if let Some(arg) = cmd.strip_prefix("echo ") {
//         //Matches "echo value", returns "value"
//         format!("{}", arg)
//     } else if cmd == "echo" {
//         String::new()
//     } else {
//         return format!("{}: command not found: ", cmd);
//     }

//     // return format!("{}: command not found: ", cmd);
// }

// fn repl() {
//     loop {
//         shell_prompt();
//     }
// }
