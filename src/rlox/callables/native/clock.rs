use std::any::Any;
use std::time::{SystemTime, UNIX_EPOCH};

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
    fn as_any(&self) -> &Any {
        self
    }
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        Ok(LoxValue::Number(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64))
    }
}
