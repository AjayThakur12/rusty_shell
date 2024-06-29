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
        let input = input.trim();
        let mut words: Vec<&str> = input.split_whitespace().collect();
        match words[0] {
            "ECHO" => {
                words.remove(0);
                for word in words {
                    print!("{} ", word);
                }
                println!()
            },
            "LS" => {

            },
            "MKDIR" => {

            },
            "EXIT" => process::exit(0),
            _ => println!("{} : command not found", input),
        }
    }
}
