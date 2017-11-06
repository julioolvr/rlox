use rlox::callables::Callable;
use rlox::interpreter::Interpreter;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::LoxValue;

pub struct LoxFunc {}
impl Callable for LoxFunc {
    fn arity(&self) -> usize {
        8
    }

    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError> {
        Ok(LoxValue::Nil)
    }
}