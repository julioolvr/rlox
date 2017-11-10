extern crate rlox;

use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;

pub fn execute(code: &str) -> Vec<String> {

    let mut input = Cursor::new(code.to_string().replace("\n", "").into_bytes());
    let output: Vec<u8> = Vec::new();
    let repl_writer = Rc::new(RefCell::new(Cursor::new(output)));

    rlox::run_repl(&mut input, repl_writer.clone());

    let output = repl_writer.borrow().get_ref().clone();
    let output = String::from_utf8(output).unwrap().replace(">> ", "");
    let output = output
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim());

    output.map(|s| String::from(s)).collect::<Vec<String>>()
}