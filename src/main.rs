use std::io::{self, Stdin, Write};

#[allow(unused)]
enum Commands {
    ECHO(String),
    LS(String),
    MKDIR(String),
}
fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin: Stdin = io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();
    // print!("{}", input);
    match input.as_str() {
        "ECHO" => println!("This is an ECHO."),
        "LS" => println!("This is an LS."),
        "MKDIR" => println!("This is a MKDIR."),
        _ => println!("{} : command not found", input.trim()),
    }
}
