pub mod errors;

use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use self::errors::RuntimeError;
use rlox::lox_value::{LoxValue, ValueError};
use rlox::parser::{Expr, Stmt};
use rlox::token::TokenType;
use rlox::environment::Environment;
use rlox::callables::{LoxFunc, LoxClass};

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
    writer: Rc<RefCell<io::Write>>,
}

impl Interpreter {
    pub fn new(writer: Rc<RefCell<io::Write>>) -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::global()));

        Interpreter {
            env: globals.clone(),
            globals: globals.clone(),
            writer,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Option<RuntimeError> {
        for stmt in stmts.iter() {
            if let Err(err) = self.interpret_stmt(stmt) {
                return Some(err);
            }
        }

        None
    }

    fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Option<LoxValue>, RuntimeError> {
        match *stmt {
            Stmt::Print(ref expr) => {
                match self.interpret_expr(expr) {
                    Ok(val) => {
                        self.writer
                            .borrow_mut()
                            .write_all(format!("{}\n", val).as_ref())
                            .expect("Error writing to stdout/writer");
                        Ok(None)
                    }
                    Err(err) => Err(err),
                }
            }
            Stmt::Expr(ref expr) => {
                match self.interpret_expr(expr) {
                    Ok(_) => Ok(None),
                    Err(err) => Err(err),
                }
            }
            Stmt::Var(ref token, ref expr) => {
                let value = match self.interpret_expr(expr) {
                    Ok(value) => value,
                    Err(err) => return Err(err),
                };

                self.env
                    .borrow_mut()
                    .define(token.lexeme.clone(), value);

                Ok(None)
            }
            Stmt::Block(ref statements) => {
                let env = Environment::from_parent(self.env.clone());

                self.interpret_block(statements, RefCell::new(env))
            }
            Stmt::If(ref condition, ref then_branch, ref else_branch) => {
                let condition_result = match self.interpret_expr(condition) {
                    Ok(value) => value,
                    Err(err) => return Err(err),
                };

                if condition_result.is_truthy() {
                    self.interpret_stmt(then_branch)
                } else if let Some(ref asd) = **else_branch {
                    self.interpret_stmt(asd)
                } else {
                    Ok(None)
                }
            }
            Stmt::While(ref condition, ref body) => {
                let mut keep_looping = match self.interpret_expr(condition) {
                    Ok(result) => result.is_truthy(),
                    Err(err) => return Err(err),
                };

                while keep_looping {
                    let result = self.interpret_stmt(body);

                    if result.is_err() {
                        return result;
                    }

                    keep_looping = match self.interpret_expr(condition) {
                        Ok(result) => result.is_truthy(),
                        Err(err) => return Err(err),
                    }
                }

                Ok(None)
            }
            Stmt::Func(ref name, _, _) => {
                let func = LoxValue::Func(Rc::new(LoxFunc::new(stmt.clone(), self.env.clone())));
                self.env.borrow_mut().define(name.lexeme.clone(), func);
                Ok(None)
            }
            Stmt::Return(_, ref expr) => Ok(Some(self.interpret_expr(expr)?)),
            Stmt::Class(ref token, _) => {
                let class = LoxValue::Class(Rc::new(LoxClass::new(token.lexeme.clone())));
                self.env
                    .borrow_mut()
                    .define(token.lexeme.clone(), class);
                Ok(None)
            }
        }
    }

    pub fn interpret_block(&mut self,
                           statements: &Vec<Stmt>,
                           environment: RefCell<Environment>)
                           -> Result<Option<LoxValue>, RuntimeError> {
        let mut return_value = None;
        let parent_env = self.env.clone();
        self.env = Rc::new(environment);

        for ref stmt in statements {
            return_value = self.interpret_stmt(stmt)?;

            if return_value.is_some() {
                break;
            }
        }

        self.env = parent_env;
        Ok(return_value)
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
            Expr::Var(ref token, ref distance) => {
                match distance {
                    &Some(distance) => {
                        match self.env.borrow().get_at(&token.lexeme, distance) {
                            Ok(value) => Ok(value.clone()),
                            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                        }
                    }
                    &None => {
                        match self.globals.borrow().get(&token.lexeme) {
                            Ok(value) => Ok(value.clone()),
                            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                        }
                    }
                }
            }
            Expr::Assign(ref token, ref expr, ref distance) => {
                let value = self.interpret_expr(expr)?;

                match distance {
                    &Some(distance) => {
                        match self.env
                                  .borrow_mut()
                                  .assign_at(&token.lexeme, value.clone(), distance) {
                            Ok(()) => Ok(value.clone()),
                            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                        }
                    }
                    &None => {
                        match self.globals
                                  .borrow_mut()
                                  .assign(&token.lexeme, value.clone()) {
                            Ok(()) => Ok(value.clone()),
                            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                        }
                    }
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
            Expr::Call(ref callee, ref arguments, ref token) => {
                let callable = self.interpret_expr(callee)?
                    .get_callable()
                    .ok_or_else(|| RuntimeError::CallOnNonCallable(token.clone()))?;

                let mut evaluated_args: Vec<LoxValue> = Vec::new();

                for arg in arguments {
                    evaluated_args.push(self.interpret_expr(arg)?);
                }

                if arguments.len() != callable.arity() {
                    return Err(RuntimeError::WrongArity(token.clone(),
                                                        arguments.len(),
                                                        callable.arity()));
                }

                callable.call(self, evaluated_args)
            }
        }
    }
}