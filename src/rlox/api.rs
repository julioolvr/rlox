use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

use rlox::scanner::Scanner;
use rlox::errors::Error;

pub fn run_file(path: &str) -> Result<(), Vec<Error>> {
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
        if let Err(errors) = run(input) {
            println!("");

            for err in errors {
                println!("{}", err);
            }
        }
    }
}

fn run(code: String) -> Result<(), Vec<Error>> {
    let scanner = Scanner::new(code);
    let (tokens, mut errors) = scanner.scan_tokens();

    for ref token in tokens {
        println!("{:?}", token);
    }

    if errors.len() == 0 {
        Ok(())
    } else {
        Err(errors)
    }
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