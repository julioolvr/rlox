use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Read;
use std::fs::File;

use rlox::scanner::Scanner;
use rlox::parser::Parser;
use rlox::errors::Error;
use rlox::interpreter::Interpreter;
use rlox::resolver::Resolver;

// TODO: The api for the writer is kind of ugly, I feel like implementation details
// are leaking from it. Revisit at some point.
pub fn run_file(path: &str, writer: Rc<RefCell<io::Write>>) -> Result<(), Vec<Error>> {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut interpreter = Interpreter::new(writer.clone());
    run(&mut interpreter, contents)
}

pub fn run_repl<R: io::BufRead>(reader: &mut R, writer: Rc<RefCell<io::Write>>) {
    println!("Welcome to the rlox prompt");
    println!("^D to exit\n");

    let user_input = ReplIterator::new(reader, writer.clone());
    let mut interpreter = Interpreter::new(writer.clone());

    for input in user_input {
        if let Err(errors) = run(&mut interpreter, input) {
            writer
                .borrow_mut()
                .write_all(b"\n")
                .expect("Error writing to stdout/writer");

            for err in errors {
                writer
                    .borrow_mut()
                    .write_all(format!("{}", err).as_ref())
                    .expect("Error writing to stdout/writer");
            }

            writer
                .borrow_mut()
                .write_all(b"\n")
                .expect("Error writing to stdout/writer");
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
            let mut resolver = Resolver::new();
            resolver.resolve_ast(&ast, interpreter);

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

struct ReplIterator<R: io::BufRead> {
    reader: R,
    writer: Rc<RefCell<io::Write>>
}

impl<R: io::BufRead> ReplIterator<R> {
    fn new(reader: R, writer: Rc<RefCell<io::Write>>) -> ReplIterator<R> {
        ReplIterator { reader, writer }
    }
}

impl<R: io::BufRead> Iterator for ReplIterator<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut input = String::new();

        self.writer
            .borrow_mut()
            .write_all(b">> ")
            .expect("Error writing to stdout/writer");
        self.writer
            .borrow_mut()
            .flush()
            .expect("Error flushing stdout/writer");

        match self.reader.read_line(&mut input) {
            Ok(0) => None,
            Ok(_) => Some(String::from(input.trim())),
            Err(_) => panic!("Error reading input line")
        }
    }
}