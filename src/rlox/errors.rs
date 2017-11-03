use std;
use rlox::token::Token;

#[derive(Debug)]
pub enum Error {
    ScannerError(usize, String),
    UnexpectedTokenError(Token, String),
    UnexpectedEofError,
    InternalError(String),
    NegateNonNumberError(Token),
    TypeError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ScannerError(ref line, ref message) => {
                write!(f, "[line {}] ScannerError: {}", line, message)
            }
            Error::UnexpectedTokenError(ref token, ref message) => {
                write!(f,
                       "[line {}] UnexpectedTokenError: {} {}",
                       token.line,
                       message,
                       token.lexeme)
            }
            Error::UnexpectedEofError => f.write_str("Unexpected end of input"),
            Error::InternalError(ref message) => {
                write!(f, "Internal interpreter error: {}", message)
            }
            Error::NegateNonNumberError(ref token) => {
                write!(f,
                       "[line {}] Cannot negate a non-numerical value",
                       token.line)
            }
            Error::TypeError => f.write_str("TypeError"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ScannerError(_, _) => "ScannerError",
            Error::UnexpectedTokenError(_, _) => "UnexpectedTokenError",
            Error::UnexpectedEofError => "UnexpectedEofError",
            Error::InternalError(_) => "InternalError",
            Error::NegateNonNumberError(_) => "NegateNonNumberError",
            Error::TypeError => "TypeError",
        }
    }
}
