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
                    Err(Error::InternalError("Invalid literal - no value".to_string()))
                }
            }
            Expr::Grouping(ref expr) => expr.value(),
            Expr::Unary(ref token, ref expr) => {
                let value = expr.value()?;

                match token.token_type {
                    TokenType::Minus => {
                        value
                            .negate_number()
                            .map_err(|_| Error::NegateNonNumberError(token.clone()))
                    }
                    TokenType::Bang => value.negate(),
                    _ => Err(Error::InternalError(format!("Invalid unary operator: {:?}", token))),
                }
            }
            Expr::Binary(ref left, ref operator, ref right) => {
                let left_value = left.value()?;
                let right_value = right.value()?;

                match operator.token_type {
                    TokenType::Minus => {
                        left_value
                            .subtract(right_value)
                            .map_err(|_| Error::SubtractNonNumbers(operator.clone()))
                    }
                    TokenType::Slash => {
                        left_value
                            .divide(right_value)
                            .map_err(|_| Error::DivideNonNumbers(operator.clone()))
                    }
                    TokenType::Star => {
                        left_value
                            .multiply(right_value)
                            .map_err(|_| Error::MultiplyNonNumbers(operator.clone()))
                    }
                    TokenType::Plus => {
                        left_value
                            .plus(right_value)
                            .map_err(|_| Error::PlusTypeError(operator.clone()))
                    }
                    TokenType::Greater => {
                        left_value
                            .is_greater(right_value)
                            .map_err(|_| Error::GreaterNonNumbers(operator.clone()))
                    }
                    TokenType::GreaterEqual => {
                        left_value
                            .is_greater_equal(right_value)
                            .map_err(|_| Error::GreaterEqualNonNumbers(operator.clone()))
                    }
                    TokenType::Less => {
                        left_value
                            .is_less(right_value)
                            .map_err(|_| Error::LessNonNumbers(operator.clone()))
                    }
                    TokenType::LessEqual => {
                        left_value
                            .is_less_equal(right_value)
                            .map_err(|_| Error::LessEqualNonNumbers(operator.clone()))
                    }
                    TokenType::BangEqual => left_value.is_not_equal(&right_value),
                    TokenType::EqualEqual => left_value.is_equal(&right_value),
                    _ => {
                        Err(Error::InternalError(format!("Invalid binary operator: {:?}",
                                                         operator)))
                    }
                }
            }
        }
    }
}