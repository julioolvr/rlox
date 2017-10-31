use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let args: Vec<String> = args.collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]")
    } else if args.len() == 1 {
        println!("Run")
    } else {
        println!("REPL")
    }
}
