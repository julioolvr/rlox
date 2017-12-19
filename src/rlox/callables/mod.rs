use std;
use std::any::Any;
mod lox_func;
pub mod native;

use rlox::interpreter::Interpreter;
use rlox::lox_value::LoxValue;
use rlox::interpreter::errors::RuntimeError;
pub use self::lox_func::LoxFunc;

pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, &mut Interpreter, Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
    fn as_any(&self) -> &Any; // TODO: Read https://stackoverflow.com/a/33687996/275442
}
