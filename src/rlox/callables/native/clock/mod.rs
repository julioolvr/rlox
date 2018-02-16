use std::any::Any;

use rlox::callables::Callable;
use rlox::interpreter::Interpreter;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::LoxValue;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        use self::wasm::get_current_time;
    } else {
        mod default;
        use self::default::get_current_time;
    }
}

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
        Ok(LoxValue::Number(get_current_time() as f64))
    }
}
