extern crate rlox;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let args: Vec<String> = args.collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if let Some(filename) = args.first() {
        rlox::run_file(filename);
    } else {
        rlox::run_repl();
    }
}
