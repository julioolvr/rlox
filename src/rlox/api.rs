use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

use rlox::scanner::Scanner;
use rlox::errors::InterpreterError;

pub fn run_file(path: &str) -> Result<(), InterpreterError> {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    run(contents)
}

pub fn run_repl() {
    println!("Welcome to the rlox prompt");
    println!("^C to exit\n");

    let user_input = ReplIterator {};

    for input in user_input {
        if let Err(err) = run(input) {
            println!("Error running command\n{}\n", err);
        }
    }
}

fn run(code: String) -> Result<(), InterpreterError> {
    let scanner = Scanner::new(code);

    for ref token in scanner.scan_tokens() {
        println!("{}", token);
    }

    Ok(())
    // Err(InterpreterError::new(4,
    //                           String::from("somefile.lox"),
    //                           String::from("Some parsing error")))
}

struct ReplIterator {}

impl Iterator for ReplIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!(">> ");
        io::stdout().flush().expect("Error flushing to stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input line");

        Some(String::from(input.trim()))
    }
}