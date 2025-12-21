#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    shell_prompt();
}

fn shell_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
