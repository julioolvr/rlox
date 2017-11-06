use time;

use rlox::callables::Callable;
use rlox::interpreter::Interpreter;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::LoxValue;

#[derive(Debug)]
pub struct ClockFunc {}

impl ClockFunc {
    pub fn new() -> ClockFunc {
        ClockFunc {}
    }
}

impl Callable for ClockFunc {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError> {
        Ok(LoxValue::Number(time::get_time().sec as f64))
    }
}