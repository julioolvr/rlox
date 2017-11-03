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
                    TokenType::Minus => {
                        if let LoxValue::Number(number) = value {
                            Ok(LoxValue::Number(-number))
                        } else {
                            Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                        }
                    }
                    TokenType::Bang => Ok(LoxValue::Bool(!value.is_truthy())),
                    _ => Err(Error::UnexpectedEofError), // TODO: Change for some InterpreterError
                }
            }
            Expr::Binary(ref left, ref operator, ref right) => {
                let left_value = left.value()?;
                let right_value = right.value()?;

                match operator.token_type {
                    TokenType::Minus => {
                        if let LoxValue::Number(left_number) = left_value {
                            if let LoxValue::Number(right_number) = right_value {
                                return Ok(LoxValue::Number(left_number - right_number));
                            }
                        }

                        Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                    }
                    TokenType::Slash => {
                        if let LoxValue::Number(left_number) = left_value {
                            if let LoxValue::Number(right_number) = right_value {
                                return Ok(LoxValue::Number(left_number / right_number));
                            }
                        }

                        Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                    }
                    TokenType::Star => {
                        if let LoxValue::Number(left_number) = left_value {
                            if let LoxValue::Number(right_number) = right_value {
                                return Ok(LoxValue::Number(left_number * right_number));
                            }
                        }

                        Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                    }
                    TokenType::Plus => {
                        match left_value {
                            LoxValue::Number(left_number) => {
                                if let LoxValue::Number(right_number) = right_value {
                                    Ok(LoxValue::Number(left_number + right_number))
                                } else {
                                    Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                                }
                            }
                            LoxValue::String(left_string) => {
                                if let LoxValue::String(right_string) = right_value {
                                    Ok(LoxValue::String(format!("{}{}", left_string, right_string)))
                                } else {
                                    Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                                }
                            }
                            _ => Err(Error::UnexpectedEofError), // TODO: Change for some InterpreterError
                        }
                    }
                    _ => Err(Error::UnexpectedEofError), // TODO: Change for some InterpreterError
                }
            }
        }
    }
}