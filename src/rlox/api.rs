use std::io;
use std::io::Read;
use std::fs::File;

use rlox::scanner::Scanner;
use rlox::parser::Parser;
use rlox::errors::Error;
use rlox::interpreter::Interpreter;

pub fn run_file(path: &str) -> Result<(), Vec<Error>> {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut interpreter = Interpreter::new();
    run(&mut interpreter, contents)
}

pub fn run_repl<R: io::BufRead, W: io::Write>(reader: R, writer: W) {
    println!("Welcome to the rlox prompt");
    println!("^D to exit\n");

    let user_input = ReplIterator::new(reader, writer);
    let mut interpreter = Interpreter::new();

    for input in user_input {
        if let Err(errors) = run(&mut interpreter, input) {
            println!();

            for err in errors {
                println!("{}", err);
            }

            println!();
        }
    }
}

fn run(interpreter: &mut Interpreter, code: String) -> Result<(), Vec<Error>> {
    let scanner = Scanner::new(code);
    let (tokens, scanner_errors) = scanner.scan_tokens();
    let parser = Parser::new(tokens);
    let ast = parser.ast();

    if scanner_errors.len() > 0 {
        return Err(scanner_errors
                       .into_iter()
                       .map(|err| Error::Scanner(err))
                       .collect());
    }

    match ast {
        Ok(ast) => {
            match interpreter.interpret(ast) {
                Some(err) => Err(vec![Error::Runtime(err)]),
                None => Ok(()),
            }
        }
        Err(errors) => {
            Err(errors
                    .into_iter()
                    .map(|err| Error::Parser(err))
                    .collect())
        }
    }
}

struct ReplIterator<R: io::BufRead, W: io::Write> {
    reader: R,
    writer: W,
}

impl<R: io::BufRead, W: io::Write> ReplIterator<R, W> {
    fn new(reader: R, writer: W) -> ReplIterator<R, W> {
        ReplIterator { reader, writer }
    }
}

impl<R: io::BufRead, W: io::Write> Iterator for ReplIterator<R, W> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.writer
            .write_all(b">> ")
            .expect("Error writing to stdout/writer");
        self.writer
            .flush()
            .expect("Error flushing stdout/writer");

        let mut input = String::new();

        match self.reader.read_line(&mut input) {
            Ok(0) => None,
            Ok(_) => Some(String::from(input.trim())),
            Err(_) => panic!("Error reading input line")
        }
    }
}