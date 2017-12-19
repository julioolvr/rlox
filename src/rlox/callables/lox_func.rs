use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

use rlox::callables::Callable;
use rlox::parser::Stmt;
use rlox::interpreter::Interpreter;
use rlox::environment::Environment;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::{LoxInstance, LoxValue};

// TODO: move this outside of rlox::callables
#[derive(Debug)]
pub struct LoxFunc {
    declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl LoxFunc {
    pub fn new(stmt: Stmt, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> LoxFunc {
        // TODO: Would be great to have a compile-time check for this instead of panicking
        match stmt {
            Stmt::Func(_, _, _) => LoxFunc {
                declaration: stmt,
                closure,
                is_initializer,
            },
            _ => panic!("Cannot build a LoxFunc with a Stmt other than Stmt::Func"),
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunc {
        let mut env = Environment::from_parent(self.closure.clone());
        env.define("this".to_string(), LoxValue::Instance(instance.clone()));

        LoxFunc {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(env)),
            is_initializer: self.is_initializer,
        }
    }
}

impl Callable for LoxFunc {
    fn as_any(&self) -> &Any {
        self
    }
    fn arity(&self) -> usize {
        match self.declaration {
            Stmt::Func(_, ref parameters, _) => parameters.len(),
            _ => panic!("Cannot build a LoxFunc with a Stmt other than Stmt::Func"),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        let mut env = Environment::from_parent(self.closure.clone());

        let (parameters, body) = match self.declaration {
            Stmt::Func(_, ref parameters, ref body) => (parameters, body),
            _ => panic!("Cannot build a LoxFunc with a Stmt other than Stmt::Func"),
        };

        let body = match **body {
            Stmt::Block(ref statements) => statements,
            _ => panic!("Cannot build a LoxFunc with a body Stmt other than Stmt::Block"),
        };

        for (i, param) in parameters.iter().enumerate() {
            env.define(
                param.lexeme.clone(),
                arguments
                    .get(i)
                    .expect("Mismatched argument and parameter sizes")
                    .clone(),
            );
        }

        let result = match interpreter.interpret_block(body, RefCell::new(env))? {
            Some(result) => Ok(result),
            None => Ok(LoxValue::Nil),
        };

        if self.is_initializer {
            return Ok(self.closure
                .borrow()
                .get_at(&"this".to_string(), 0)
                .expect("Couldn't find reference to `this` in initializer"));
        }

        return result;
    }
}
