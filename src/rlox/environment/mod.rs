use std::collections::hash_map::HashMap;
mod errors;

use self::errors::EnvironmentError;
use rlox::lox_value::LoxValue;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, key: String, val: LoxValue) {
        self.values.insert(key, val);
    }

    pub fn get(&self, key: &String) -> Result<&LoxValue, EnvironmentError> {
        match self.values.get(key) {
            Some(value) => Ok(value),
            None => Err(EnvironmentError::UndefinedVariable(key.clone()))
        }
    }
}