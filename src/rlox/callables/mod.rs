mod lox_func;

use rlox::interpreter::Interpreter;
use rlox::lox_value::LoxValue;
use rlox::interpreter::errors::RuntimeError;
pub use self::lox_func::LoxFunc;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, &Interpreter, Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
}
