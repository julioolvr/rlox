use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use rlox::interpreter::errors::RuntimeError;
use rlox::token::Token;
use rlox::lox_value::LoxValue;
use rlox::lox_value::lox_class::LoxClassInternal;

#[derive(Debug, Clone)]
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

    pub fn get(&self, name: &Token) -> Result<LoxValue, RuntimeError> {
        self.state
            .get(&name.lexeme)
            .map(|property| property.clone())
            .or_else(|| {
                self.class
                    .find_method(&name.lexeme, Rc::new(RefCell::new(self.clone())))
                    .map(|method| LoxValue::Func(Rc::new(method)))
            })
            .ok_or(RuntimeError::UndefinedProperty(name.clone()))
    }

    pub fn set(&mut self, name: &str, value: LoxValue) {
        self.state.insert(name.to_string(), value);
    }

    pub fn get_class_name(&self) -> &str {
        &self.class.name
    }
}
