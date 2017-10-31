mod rlox;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let args: Vec<String> = args.collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if let Some(filename) = args.first() {
        if let Err(errors) = rlox::run_file(filename) {
            println!("Error running file {}\n", filename);

            for err in errors {
                println!("{}", err);
            }
        }
    } else {
        rlox::run_repl();
    }
}
