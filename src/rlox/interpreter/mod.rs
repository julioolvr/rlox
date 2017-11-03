pub mod errors;

use self::errors::RuntimeError;
use rlox::lox_value::{LoxValue, ValueError};
use rlox::parser::{Expr, Stmt};
use rlox::token::TokenType;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(stmts: Vec<Stmt>) -> Option<RuntimeError> {
        for stmt in stmts.iter() {
            if let Some(err) = Interpreter::interpret_stmt(stmt) {
                return Some(err);
            }
        }

        None
    }

    fn interpret_stmt(stmt: &Stmt) -> Option<RuntimeError> {
        match *stmt {
            Stmt::Print(ref expr) => {
                match Interpreter::interpret_expr(expr) {
                    Ok(val) => {
                        println!("{}", val);
                        None
                    }
                    Err(err) => Some(err),
                }
            }
            Stmt::Expr(ref expr) => {
                match Interpreter::interpret_expr(expr) {
                    Ok(_) => None,
                    Err(err) => Some(err),
                }
            }
        }
    }

    fn interpret_expr(expr: &Expr) -> Result<LoxValue, RuntimeError> {
        match *expr {
            Expr::Literal(ref literal) => {
                if let Some(value) = literal.value() {
                    Ok(value)
                } else {
                    Err(RuntimeError::InternalError("Invalid literal - no value".to_string()))
                }
            }
            Expr::Grouping(ref expr) => Interpreter::interpret_expr(expr),
            Expr::Unary(ref token, ref expr) => {
                let value = Interpreter::interpret_expr(expr)?;

                match token.token_type {
                    TokenType::Minus => {
                        value
                            .negate_number()
                            .map_err(|_| RuntimeError::NegateNonNumberError(token.clone()))
                    }
                    TokenType::Bang => {
                        value
                            .negate()
                            .map_err(|_| {
                                         RuntimeError::InternalError("Can't negate value"
                                                                         .to_string())
                                     })
                    }
                    _ => {
                        Err(RuntimeError::InternalError(format!("Invalid unary operator: {:?}",
                                                                token)))
                    }
                }
            }
            Expr::Binary(ref left, ref operator, ref right) => {
                let left_value = Interpreter::interpret_expr(left)?;
                let right_value = Interpreter::interpret_expr(right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        left_value
                            .subtract(right_value)
                            .map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone()))
                    }
                    TokenType::Slash => {
                        left_value
                            .divide(right_value)
                            .map_err(|err| match err {
                                         ValueError::DivideByZero => {
                                             RuntimeError::DivideByZeroError(operator.clone())
                                         }
                                         _ => RuntimeError::DivideNonNumbers(operator.clone()),
                                     })
                    }
                    TokenType::Star => {
                        left_value
                            .multiply(right_value)
                            .map_err(|_| RuntimeError::MultiplyNonNumbers(operator.clone()))
                    }
                    TokenType::Plus => {
                        left_value
                            .plus(right_value)
                            .map_err(|_| RuntimeError::PlusTypeError(operator.clone()))
                    }
                    TokenType::Greater => {
                        left_value
                            .is_greater(right_value)
                            .map_err(|_| RuntimeError::GreaterNonNumbers(operator.clone()))
                    }
                    TokenType::GreaterEqual => {
                        left_value
                            .is_greater_equal(right_value)
                            .map_err(|_| RuntimeError::GreaterEqualNonNumbers(operator.clone()))
                    }
                    TokenType::Less => {
                        left_value
                            .is_less(right_value)
                            .map_err(|_| RuntimeError::LessNonNumbers(operator.clone()))
                    }
                    TokenType::LessEqual => {
                        left_value
                            .is_less_equal(right_value)
                            .map_err(|_| RuntimeError::LessEqualNonNumbers(operator.clone()))
                    }
                    TokenType::BangEqual => {
                        left_value
                            .is_not_equal(&right_value)
                            .map_err(|_| {
                                         RuntimeError::InternalError("Can't check non-equality"
                                                                         .to_string())
                                     })
                    }
                    TokenType::EqualEqual => {
                        left_value
                            .is_equal(&right_value)
                            .map_err(|_| {
                                         RuntimeError::InternalError("Can't check equality"
                                                                         .to_string())
                                     })
                    }
                    _ => {
                        Err(RuntimeError::InternalError(format!("Invalid binary operator: {:?}",
                                                                operator)))
                    }
                }
            }
        }
    }
}