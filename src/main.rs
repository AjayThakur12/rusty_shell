use std::io::{self, Stdin, Write};
use std::process;

#[allow(unused)]
enum Commands {
    ECHO(String),
    LS(String),
    MKDIR(String),
}
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin: Stdin = io::stdin();
        let mut input: String = String::new();
        stdin.read_line(&mut input).unwrap();
        match input.trim() {
            "ECHO" => println!("This is an ECHO."),
            "LS" => println!("This is an LS."),
            "MKDIR" => println!("This is a MKDIR."),
            "EXIT" => process::exit(0),
            _ => println!("{} : command not found", input),
        }
    }
}
