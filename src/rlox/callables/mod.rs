use std;
use std::any::Any;
pub mod native;

use rlox::interpreter::Interpreter;
use rlox::lox_value::LoxValue;
use rlox::interpreter::errors::RuntimeError;

pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, &mut Interpreter, Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
    fn as_any(&self) -> &Any; // TODO: Read https://stackoverflow.com/a/33687996/275442
}
