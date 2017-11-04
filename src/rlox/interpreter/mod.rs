pub mod errors;

use self::errors::RuntimeError;
use rlox::lox_value::{LoxValue, ValueError};
use rlox::parser::{Expr, Stmt};
use rlox::token::TokenType;
use rlox::environment::Environment;

pub struct Interpreter {
    env: Option<Environment>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { env: Some(Environment::new()) }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Option<RuntimeError> {
        for stmt in stmts.iter() {
            if let Some(err) = self.interpret_stmt(stmt) {
                return Some(err);
            }
        }

        None
    }

    fn interpret_stmt(&mut self, stmt: &Stmt) -> Option<RuntimeError> {
        match *stmt {
            Stmt::Print(ref expr) => {
                match self.interpret_expr(expr) {
                    Ok(val) => {
                        println!("{}", val);
                        None
                    }
                    Err(err) => Some(err),
                }
            }
            Stmt::Expr(ref expr) => {
                match self.interpret_expr(expr) {
                    Ok(_) => None,
                    Err(err) => Some(err),
                }
            }
            Stmt::Var(ref token, ref expr) => {
                let value = match self.interpret_expr(expr) {
                    Ok(value) => value,
                    Err(err) => return Some(err),
                };

                match self.env.as_mut() {
                    Some(env) => env.define(token.lexeme.clone(), value),
                    None => return Some(RuntimeError::InternalError("Missing environment when setting variable".to_string())),
                }

                None
            }
            Stmt::Block(ref statements) => {
                let parent_env = self.env.take();

                if parent_env.is_none() {
                    return Some(RuntimeError::InternalError("Missing environment when creating block".to_string()));
                }

                let parent_env = parent_env.unwrap();
                let new_env = Environment::from_parent(parent_env);
                self.env = Some(new_env);

                for stmt in statements {
                    if let Some(err) = self.interpret_stmt(stmt) {
                        return Some(err);
                    }
                }

                if self.env.is_none() {
                    return Some(RuntimeError::InternalError("Missing environment after leaving block".to_string()));
                }

                self.env = self.env.take().unwrap().pop();
                None
            }
            Stmt::If(ref condition, ref then_branch, ref else_branch) => {
                let condition_result = match self.interpret_expr(condition) {
                    Ok(value) => value,
                    Err(err) => return Some(err),
                };

                if condition_result.is_truthy() {
                    self.interpret_stmt(then_branch)
                } else if let Some(ref asd) = **else_branch {
                    self.interpret_stmt(asd)
                } else {
                    None
                }
            }
        }
    }

    fn interpret_expr(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        match *expr {
            Expr::Literal(ref literal) => {
                if let Some(value) = literal.value() {
                    Ok(value)
                } else {
                    Err(RuntimeError::InternalError("Invalid literal - no value".to_string()))
                }
            }
            Expr::Grouping(ref expr) => self.interpret_expr(expr),
            Expr::Unary(ref token, ref expr) => {
                let value = self.interpret_expr(expr)?;

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
                let left_value = self.interpret_expr(left)?;
                let right_value = self.interpret_expr(right)?;

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
            Expr::Var(ref token) => {
                if self.env.is_none() {
                    return Err(RuntimeError::InternalError("Missing environment when retrieving variable".to_string()));
                }

                match self.env.as_mut().unwrap().get(&token.lexeme) {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                }
            }
            Expr::Assign(ref token, ref expr) => {
                let value = self.interpret_expr(expr)?;

                if self.env.is_none() {
                    return Err(RuntimeError::InternalError("Missing environment when assigning variable".to_string()));
                }

                match self.env
                          .as_mut()
                          .unwrap()
                          .assign(&token.lexeme, value.clone()) {
                    Ok(_) => Ok(value),
                    Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                }
            }
            Expr::Logical(ref left, ref operator, ref right) => {
                let left_value = self.interpret_expr(left)?;

                if operator.token_type == TokenType::Or {
                    if left_value.is_truthy() {
                        return Ok(left_value);
                    }
                } else {
                    if !left_value.is_truthy() {
                        return Ok(left_value);
                    }
                }

                self.interpret_expr(right)
            }
        }
    }
}