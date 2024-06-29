use std::io::{self, Stdin, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin: Stdin = io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();
}
