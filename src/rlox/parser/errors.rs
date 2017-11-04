use std;
use rlox::token::Token;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedTokenError(Token, String),
    UnexpectedEofError,
    InvalidAssignmentError(Token),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParsingError::UnexpectedTokenError(ref token, ref message) => {
                write!(f,
                       "[line {}] UnexpectedTokenError: {} {}",
                       token.line,
                       message,
                       token.lexeme)
            }
            ParsingError::UnexpectedEofError => f.write_str("Unexpected end of input"),
            ParsingError::InvalidAssignmentError(ref token) => {
                write!(f, "[line {}] Invalid assignment target", token.line)
            }
        }
    }
}

impl std::error::Error for ParsingError {
    fn description(&self) -> &str {
        match *self {
            ParsingError::UnexpectedTokenError(_, _) => "UnexpectedTokenError",
            ParsingError::UnexpectedEofError => "UnexpectedEofError",
            ParsingError::InvalidAssignmentError(_) => "InvalidAssignmentError"
        }
    }
}