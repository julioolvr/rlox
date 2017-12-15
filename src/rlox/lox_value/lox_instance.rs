use std::collections::hash_map::HashMap;
use std::rc::Rc;

use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::LoxValue;
use rlox::lox_value::lox_class::LoxClassInternal;

#[derive(Debug)]
pub struct LoxInstance {
    class: Rc<LoxClassInternal>,
    state: HashMap<String, LoxValue>,
}

impl LoxInstance {
    pub fn new(class: Rc<LoxClassInternal>) -> LoxInstance {
        LoxInstance {
            class,
            state: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Result<&LoxValue, RuntimeError> {
        self.state
            .get(name)
            .ok_or(RuntimeError::UndefinedProperty(name.to_string()))
    }

    pub fn set(&mut self, name: &str, value: LoxValue) {
        self.state.insert(name.to_string(), value);
    }

    pub fn get_class_name(&self) -> &str {
        &self.class.name
    }
}