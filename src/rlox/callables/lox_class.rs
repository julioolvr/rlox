use std::rc::Rc;

use rlox::callables::Callable;
use rlox::interpreter::Interpreter;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::LoxValue;

#[derive(Debug)]
pub struct LoxClass {
    internal: Rc<LoxClassInternal>,
}

#[derive(Debug)]
pub struct LoxClassInternal {
    pub name: String,
}

impl LoxClass {
    pub fn new(name: String) -> LoxClass {
        LoxClass { internal: Rc::new(LoxClassInternal { name }) }
    }

    pub fn instantiate(&self) -> Result<LoxValue, RuntimeError> {
        Ok(LoxValue::Instance(self.internal.clone()))
    }

    pub fn get_name(&self) -> &str {
        &self.internal.name
    }
}

impl Callable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self,
            _interpreter: &mut Interpreter,
            _arguments: Vec<LoxValue>)
            -> Result<LoxValue, RuntimeError> {
        self.instantiate()
    }
}