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
    SubtractNonNumbers(Token),
    DivideNonNumbers(Token),
    MultiplyNonNumbers(Token),
    PlusTypeError(Token),
    GreaterNonNumbers(Token),
    GreaterEqualNonNumbers(Token),
    LessNonNumbers(Token),
    LessEqualNonNumbers(Token),
    DivideByZero,
    DivideByZeroError(Token),
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
            Error::SubtractNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a subtraction must be numbers",
                       token.line)
            }
            Error::DivideNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a division must be numbers",
                       token.line)
            }
            Error::MultiplyNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a multiplication must be numbers",
                       token.line)
            }
            Error::PlusTypeError(ref token) => {
                write!(f,
                       "[line {}] Both sides of an addition must be either strings or numbers",
                       token.line)
            }
            Error::GreaterNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a greater than comparison must be numbers",
                       token.line)
            }
            Error::GreaterEqualNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a greater or equal comparison must be numbers",
                       token.line)
            }
            Error::LessNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a less than comparison must be numbers",
                       token.line)
            }
            Error::LessEqualNonNumbers(ref token) => {
                write!(f,
                       "[line {}] Both sides of a less or equal comparison must be numbers",
                       token.line)
            }
            Error::DivideByZero => f.write_str("DivideByZero"),
            Error::DivideByZeroError(ref token) => {
                write!(f, "[line {}] Cannot divide by zero", token.line)
            }
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
            Error::SubtractNonNumbers(_) => "SubtractNonNumbers",
            Error::DivideNonNumbers(_) => "DivideNonNumbers",
            Error::MultiplyNonNumbers(_) => "MultiplyNonNumbers",
            Error::PlusTypeError(_) => "PlusTypeError",
            Error::GreaterNonNumbers(_) => "GreaterNonNumbers",
            Error::GreaterEqualNonNumbers(_) => "GreaterEqualNonNumbers",
            Error::LessNonNumbers(_) => "LessNonNumbers",
            Error::LessEqualNonNumbers(_) => "LessEqualNonNumbers",
            Error::DivideByZero => "DivideByZero",
            Error::DivideByZeroError(_) => "DivideByZeroError",
        }
    }
}
