use rlox::callables::Callable;
use rlox::parser::Stmt;
use rlox::interpreter::Interpreter;
use rlox::environment::Environment;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::LoxValue;

#[derive(Debug)]
pub struct LoxFunc {
    declaration: Stmt,
}

impl LoxFunc {
    pub fn new(stmt: Stmt) -> LoxFunc {
        // TODO: Would be great to have a compile-time check for this instead of panicking
        match stmt {
            Stmt::Func(_, _, _) => LoxFunc { declaration: stmt },
            _ => panic!("Cannot build a LoxFunc with a Stmt other than Stmt::Func"),
        }
    }
}

impl Callable for LoxFunc {
    fn arity(&self) -> usize {
        match self.declaration {
            Stmt::Func(_, ref parameters, _) => parameters.len(),
            _ => panic!("Cannot build a LoxFunc with a Stmt other than Stmt::Func"),
        }
    }

    fn call(&self,
            interpreter: &mut Interpreter,
            arguments: Vec<LoxValue>)
            -> Result<LoxValue, RuntimeError> {
        let mut env = Environment::new();

        let (parameters, body) = match self.declaration {
            Stmt::Func(_, ref parameters, ref body) => (parameters, body),
            _ => panic!("Cannot build a LoxFunc with a Stmt other than Stmt::Func"),
        };

        let body = match **body {
            Stmt::Block(ref statements) => statements,
            _ => panic!("Cannot build a LoxFunc with a body Stmt other than Stmt::Block"),
        };

        for (i, param) in parameters.iter().enumerate() {
            env.define(param.lexeme.clone(),
                       arguments
                           .get(i)
                           .expect("Mismatched argument and parameter sizes")
                           .clone());
        }

        match interpreter.interpret_block(body, env)? {
            Some(result) => Ok(result),
            None => Ok(LoxValue::Nil)
        }
    }
}