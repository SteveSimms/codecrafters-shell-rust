#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // shell_prompt();
    repl()
}

fn shell_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
    println!("{}", read_input());
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let cmd = input.trim();
    let cmd_not_found = format!("{}: command not found", cmd);

    if cmd == "exit" {
        std::process::exit(0);
    }

    if cmd.starts_with("echo") {
        // echo the input
        let echo = cmd;
        //strips the echo prefix from command using pattern matching

        let stripped_echo = echo.strip_prefix("echo").unwrap_or(&echo);
        return format!("{}", stripped_echo);
    }

    if cmd != "echo" {
        return cmd_not_found;
    } else {
        return String::new();
    }
}

fn repl() {
    loop {
        shell_prompt();
    }
}
