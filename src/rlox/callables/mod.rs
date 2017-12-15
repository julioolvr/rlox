use std;
mod lox_func;
mod lox_class;
pub mod native;

use rlox::interpreter::Interpreter;
use rlox::lox_value::LoxValue;
use rlox::interpreter::errors::RuntimeError;
pub use self::lox_func::LoxFunc;
pub use self::lox_class::LoxClass;
pub use self::lox_class::LoxClassInternal;

pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, &mut Interpreter, Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
}
