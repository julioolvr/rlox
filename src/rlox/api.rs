use std::io;
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Read;
use std::fs::File;

use rlox::scanner::Scanner;
use rlox::parser::Parser;
use rlox::errors::Error;
use rlox::interpreter::Interpreter;
use rlox::resolver::Resolver;

pub fn run_file(path: &str, writer: &mut io::Write) -> Result<(), Vec<Error>> {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let writer = Rc::new(RefCell::new(writer));
    let mut interpreter = Interpreter::new(writer);
    run(&mut interpreter, contents)
}

pub fn run_repl<R: io::BufRead>(reader: &mut R, writer: &mut io::Write) {
    println!("Welcome to the rlox prompt");
    println!("^D to exit\n");

    let writer = Rc::new(RefCell::new(writer));
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

/// Receive Lox code as a string, and return the contents of
/// stdout after executing it as another string:
pub fn run_string(code: String) -> String {
    let output: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(output);

    let result = {
        let mut writer = Rc::new(RefCell::new(&mut cursor as &mut io::Write));
        let mut interpreter = Interpreter::new(writer.clone());
        run(&mut interpreter, code)
    };

    match result {
        Ok(_) => {
            let output = cursor.get_ref().clone();
            String::from_utf8(output).unwrap()
        }
        Err(errors) => errors
            .iter()
            .map(|error| error.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
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
        Ok(mut ast) => {
            let mut resolver = Resolver::new();
            resolver.resolve_ast(&mut ast);

            match interpreter.interpret(ast) {
                Some(err) => Err(vec![Error::Runtime(err)]),
                None => Ok(()),
            }
        }
        Err(errors) => Err(errors.into_iter().map(|err| Error::Parser(err)).collect()),
    }
}

struct ReplIterator<'a, R: io::BufRead> {
    reader: R,
    writer: Rc<RefCell<&'a mut io::Write>>,
}

impl<'a, R: io::BufRead> ReplIterator<'a, R> {
    fn new(reader: R, writer: Rc<RefCell<&'a mut io::Write>>) -> ReplIterator<R> {
        ReplIterator { reader, writer }
    }
}

impl<'a, R: io::BufRead> Iterator for ReplIterator<'a, R> {
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
            Err(_) => panic!("Error reading input line"),
        }
    }
}
