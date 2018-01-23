pub mod errors;

use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use self::errors::RuntimeError;
use rlox::lox_value::{LoxClass, LoxValue, ValueError};
use rlox::parser::{Expr, Stmt};
use rlox::token::TokenType;
use rlox::environment::Environment;
use rlox::callables::LoxFunc;

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
            Stmt::Print(ref expr) => self.interpret_expr(expr).map(|val| {
                self.writer
                    .borrow_mut()
                    .write_all(format!("{}\n", val).as_ref())
                    .expect("Error writing to stdout/writer");
                None
            }),
            Stmt::Expr(ref expr) => self.interpret_expr(expr).map(|_| None),
            Stmt::Var(ref token, ref expr) => self.interpret_expr(expr).map(|value| {
                self.env.borrow_mut().define(token.lexeme.clone(), value);
                None
            }),
            Stmt::Block(ref statements) => {
                let env = Environment::from_parent(self.env.clone());

                self.interpret_block(statements, RefCell::new(env))
            }
            Stmt::If(ref condition, ref then_branch, ref else_branch) => {
                self.interpret_expr(condition).and_then(|condition_result| {
                    if condition_result.is_truthy() {
                        self.interpret_stmt(then_branch)
                    } else if let Some(ref else_branch) = **else_branch {
                        self.interpret_stmt(else_branch)
                    } else {
                        Ok(None)
                    }
                })
            }
            Stmt::While(ref condition, ref body) => {
                while self.interpret_expr(condition)?.is_truthy() {
                    self.interpret_stmt(body)?;
                }

                Ok(None)
            }
            Stmt::Func(ref name, _, _) => {
                let func =
                    LoxValue::Func(Rc::new(LoxFunc::new(stmt.clone(), self.env.clone(), false)));
                self.env.borrow_mut().define(name.lexeme.clone(), func);
                Ok(None)
            }
            Stmt::Return(_, ref expr) => Ok(Some(self.interpret_expr(expr)?)),
            Stmt::Class(ref token, ref superclass, ref method_statements) => {
                let mut methods = HashMap::new();
                let mut parent_env = None;

                let resolved_superclass = if let &Some(ref superclass) = superclass {
                    let superclass = match self.interpret_expr(superclass)? {
                        LoxValue::Class(ref class) => class.clone(),
                        _ => return Err(RuntimeError::InvalidSuperclass(token.clone())),
                    };

                    parent_env = Some(self.env.clone());
                    let mut env = Environment::from_parent(self.env.clone());
                    env.define("super".to_string(), LoxValue::Class(superclass.clone()));
                    self.env = Rc::new(RefCell::new(env));

                    Some(superclass)
                } else {
                    None
                };

                for method_statement in method_statements {
                    match method_statement {
                        &Stmt::Func(ref name, _, _) => {
                            let method = LoxValue::Func(Rc::new(LoxFunc::new(
                                method_statement.clone(),
                                self.env.clone(),
                                name.lexeme == "init",
                            )));
                            methods.insert(name.lexeme.clone(), method);
                        }
                        _ => return Err(RuntimeError::InternalError("TODO: Change me".to_string())),
                    };
                }

                let class = LoxValue::Class(Rc::new(LoxClass::new(
                    token.lexeme.clone(),
                    resolved_superclass,
                    methods,
                )));

                if superclass.is_some() {
                    self.env = parent_env.expect("When interpreting a subclass, a parent environment should always be present");
                }

                self.env.borrow_mut().define(token.lexeme.clone(), class);

                Ok(None)
            }
        }
    }

    pub fn interpret_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: RefCell<Environment>,
    ) -> Result<Option<LoxValue>, RuntimeError> {
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
                    Err(RuntimeError::InternalError(
                        "Invalid literal - no value".to_string(),
                    ))
                }
            }
            Expr::Grouping(ref expr) => self.interpret_expr(expr),
            Expr::Unary(ref token, ref expr) => {
                let value = self.interpret_expr(expr)?;

                match token.token_type {
                    TokenType::Minus => value
                        .negate_number()
                        .map_err(|_| RuntimeError::NegateNonNumberError(token.clone())),
                    TokenType::Bang => value
                        .negate()
                        .map_err(|_| RuntimeError::InternalError("Can't negate value".to_string())),
                    _ => Err(RuntimeError::InternalError(format!(
                        "Invalid unary operator: {:?}",
                        token
                    ))),
                }
            }
            Expr::Binary(ref left, ref operator, ref right) => {
                let left_value = self.interpret_expr(left)?;
                let right_value = self.interpret_expr(right)?;

                match operator.token_type {
                    TokenType::Minus => left_value
                        .subtract(right_value)
                        .map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
                    TokenType::Slash => left_value.divide(right_value).map_err(|err| match err {
                        ValueError::DivideByZero => {
                            RuntimeError::DivideByZeroError(operator.clone())
                        }
                        _ => RuntimeError::DivideNonNumbers(operator.clone()),
                    }),
                    TokenType::Star => left_value
                        .multiply(right_value)
                        .map_err(|_| RuntimeError::MultiplyNonNumbers(operator.clone())),
                    TokenType::Plus => left_value
                        .plus(right_value)
                        .map_err(|_| RuntimeError::PlusTypeError(operator.clone())),
                    TokenType::Greater => left_value
                        .is_greater(right_value)
                        .map_err(|_| RuntimeError::GreaterNonNumbers(operator.clone())),
                    TokenType::GreaterEqual => left_value
                        .is_greater_equal(right_value)
                        .map_err(|_| RuntimeError::GreaterEqualNonNumbers(operator.clone())),
                    TokenType::Less => left_value
                        .is_less(right_value)
                        .map_err(|_| RuntimeError::LessNonNumbers(operator.clone())),
                    TokenType::LessEqual => left_value
                        .is_less_equal(right_value)
                        .map_err(|_| RuntimeError::LessEqualNonNumbers(operator.clone())),
                    TokenType::BangEqual => left_value.is_not_equal(&right_value).map_err(|_| {
                        RuntimeError::InternalError("Can't check non-equality".to_string())
                    }),
                    TokenType::EqualEqual => left_value.is_equal(&right_value).map_err(|_| {
                        RuntimeError::InternalError("Can't check equality".to_string())
                    }),
                    _ => Err(RuntimeError::InternalError(format!(
                        "Invalid binary operator: {:?}",
                        operator
                    ))),
                }
            }
            Expr::Var(ref token, ref distance) => match distance {
                &Some(distance) => match self.env.borrow().get_at(&token.lexeme, distance) {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                },
                &None => match self.globals.borrow().get(&token.lexeme) {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                },
            },
            Expr::Assign(ref token, ref expr, ref distance) => {
                let value = self.interpret_expr(expr)?;

                match distance {
                    &Some(distance) => match self.env.borrow_mut().assign_at(
                        &token.lexeme,
                        value.clone(),
                        distance,
                    ) {
                        Ok(()) => Ok(value.clone()),
                        Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                    },
                    &None => match self.globals
                        .borrow_mut()
                        .assign(&token.lexeme, value.clone())
                    {
                        Ok(()) => Ok(value.clone()),
                        Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                    },
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
                    return Err(RuntimeError::WrongArity(
                        token.clone(),
                        arguments.len(),
                        callable.arity(),
                    ));
                }

                callable.call(self, evaluated_args)
            }
            Expr::Get(ref target, ref token) => {
                let resolved_target = self.interpret_expr(target)?;

                match resolved_target {
                    // TODO: Don't clone!
                    LoxValue::Instance(ref instance) => {
                        Ok(instance.borrow().get(&token.lexeme)?.clone())
                    }
                    _ => Err(RuntimeError::InvalidGetTarget(token.clone())),
                }
            }
            Expr::Set(ref target, ref token, ref expr) => {
                let resolved_target = self.interpret_expr(target)?;

                let value = match resolved_target {
                    LoxValue::Instance(instance) => {
                        let resolved_value = self.interpret_expr(expr)?;

                        // TODO: Don't clone!
                        instance
                            .borrow_mut()
                            .set(&token.lexeme, resolved_value.clone());
                        resolved_value.clone()
                    }
                    _ => return Err(RuntimeError::InvalidGetTarget(token.clone())),
                };

                Ok(value)
            }
            Expr::This(ref token, ref distance) => match distance {
                &Some(distance) => match self.env.borrow().get_at(&token.lexeme, distance) {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                },
                &None => match self.globals.borrow().get(&token.lexeme) {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
                },
            },
            Expr::Super(_, ref method, ref distance) => match distance {
                &Some(distance) => {
                    let superclass = self.env
                        .borrow()
                        .get_at(&"super".to_string(), distance)
                        .expect("Couldn't find `super` when interpreting");
                    let instance = self.env
                        .borrow()
                        .get_at(&"this".to_string(), distance - 1)
                        .expect("Couldn't find `this` when interpreting `super` call");

                    let superclass = match superclass {
                        LoxValue::Class(ref class) => class,
                        _ => {
                            return Err(RuntimeError::InternalError(
                                "Couldn't extract LoxClass from LoxValue::Class".to_string(),
                            ))
                        }
                    };

                    let instance = match instance {
                        LoxValue::Instance(ref instance) => instance,
                        _ => {
                            return Err(RuntimeError::InternalError(
                                "Couldn't extract LoxInstance from LoxValue::Instance".to_string(),
                            ))
                        }
                    };

                    let resolved_method = superclass.find_method(&method.lexeme, instance.clone());

                    match resolved_method {
                        Some(method) => Ok(LoxValue::Func(Rc::new(method))),
                        None => Err(RuntimeError::UndefinedProperty(method.lexeme.clone())),
                    }
                }
                &None => Err(RuntimeError::InternalError(
                    "Couldn't find distance to super reference".to_string(),
                )),
            },
        }
    }
}
