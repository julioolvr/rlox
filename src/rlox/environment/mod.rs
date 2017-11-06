use std::collections::hash_map::HashMap;
mod errors;

use std::rc::Rc;
use self::errors::EnvironmentError;
use rlox::lox_value::LoxValue;
use rlox::callables::native;

pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Box<Option<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Box::new(None),
        }
    }

    pub fn global() -> Environment {
        let mut env = Environment::new();

        env.define("clock".to_string(),
                   LoxValue::Func(Rc::new(native::ClockFunc::new())));

        env
    }

    pub fn from_parent(parent: Environment) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Box::new(Some(parent)),
        }
    }

    pub fn define(&mut self, key: String, val: LoxValue) {
        self.values.insert(key, val);
    }

    pub fn assign(&mut self, key: &String, val: LoxValue) -> Result<(), EnvironmentError> {
        if self.values.contains_key(key) {
            self.values.insert(key.clone(), val);
            Ok(())
        } else {
            match *self.enclosing {
                Some(ref mut parent) => parent.assign(key, val),
                None => Err(EnvironmentError::UndefinedVariable(key.clone())),
            }
        }
    }

    pub fn get(&self, key: &String) -> Result<&LoxValue, EnvironmentError> {
        match self.values.get(key) {
            Some(value) => Ok(value),
            None => {
                match *self.enclosing {
                    Some(ref parent) => parent.get(key),
                    None => Err(EnvironmentError::UndefinedVariable(key.clone())),
                }
            }
        }
    }

    /// Drop this environment and return its parent
    pub fn pop(self) -> Option<Environment> {
        *self.enclosing
    }
}