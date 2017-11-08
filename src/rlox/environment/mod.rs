use std::collections::hash_map::HashMap;
mod errors;

use std::rc::Rc;
use std::cell::RefCell;

use self::errors::EnvironmentError;
use rlox::lox_value::LoxValue;
use rlox::callables::native;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn global() -> Environment {
        let mut env = Environment::new();

        env.define("clock".to_string(),
                   LoxValue::Func(Rc::new(native::ClockFunc::new())));

        env
    }

    pub fn from_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(parent),
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
            match self.enclosing {
                Some(ref mut parent) => parent.borrow_mut().assign(key, val),
                None => Err(EnvironmentError::UndefinedVariable(key.clone())),
            }
        }
    }

    pub fn get(&self, key: &String) -> Result<LoxValue, EnvironmentError> {
        match self.values.get(key) {
            Some(value) => Ok(value.clone()),
            None => {
                match self.enclosing {
                    Some(ref parent) => parent.borrow().get(key),
                    None => Err(EnvironmentError::UndefinedVariable(key.clone())),
                }
            }
        }
    }
}