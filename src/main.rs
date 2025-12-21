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
    let trimmed_input = input.trim();
    let cmd_not_found = format!("{}: command not found", trimmed_input);
    cmd_not_found
}

fn repl() {
    loop {
        shell_prompt();
        read_input();
    }
}
