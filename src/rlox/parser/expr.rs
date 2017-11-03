use std;
use rlox::token::{Token, Literal, TokenType};
use rlox::errors::Error;
use rlox::lox_value::LoxValue;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Expr::Binary(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(ref expr) => write!(f, "(group {})", expr),
            Expr::Literal(ref literal) => write!(f, "{}", literal),
            Expr::Unary(ref operator, ref expr) => write!(f, "({} {})", operator.lexeme, expr),
        }
    }
}

impl Expr {
    pub fn value(&self) -> Result<LoxValue, Error> {
        match *self {
            Expr::Literal(ref literal) => {
                if let Some(value) = literal.value() {
                    Ok(value)
                } else {
                    Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                }
            }
            Expr::Grouping(ref expr) => expr.value(),
            Expr::Unary(ref token, ref expr) => {
                let value = expr.value()?;

                match token.token_type {
                    TokenType::Minus => value.negate_number(),
                    TokenType::Bang => value.negate(),
                    _ => Err(Error::UnexpectedEofError), // TODO: Change for some InterpreterError
                }
            }
            Expr::Binary(ref left, ref operator, ref right) => {
                let left_value = left.value()?;
                let right_value = right.value()?;

                match operator.token_type {
                    TokenType::Minus => left_value.subtract(right_value),
                    TokenType::Slash => left_value.divide(right_value),
                    TokenType::Star => left_value.multiply(right_value),
                    TokenType::Plus => left_value.plus(right_value),
                    _ => Err(Error::UnexpectedEofError), // TODO: Change for some InterpreterError
                }
            }
        }
    }
}