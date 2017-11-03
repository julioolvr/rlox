use std;

use rlox::scanner::errors::ScannerError;
use rlox::parser::errors::ParsingError;
use rlox::interpreter::errors::RuntimeError;

#[derive(Debug)]
pub enum Error {
    Scanner(ScannerError),
    Parser(ParsingError),
    Runtime(RuntimeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Scanner(ref err) => write!(f, "{}", err),
            Error::Parser(ref err) => write!(f, "{}", err),
            Error::Runtime(ref err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Scanner(_) => "Error::Scanner",
            Error::Parser(_) => "Error::Parser",
            Error::Runtime(_) => "Error::Runtime",
        }
    }
}
