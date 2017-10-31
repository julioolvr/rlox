use std::fs::File;
use std::io::Read;

pub fn run_file(path: &str) {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    run(contents);
}

fn run(code: String) {
    println!("Running code\n{}", code);
}