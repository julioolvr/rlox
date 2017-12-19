use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use rlox::interpreter::errors::RuntimeError;
use rlox::callables::LoxFunc;
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

    pub fn get(&self, name: &str) -> Result<LoxValue, RuntimeError> {
        self.state
            .get(name)
            .map(|property| property.clone())
            .or_else(|| {
                self.class
                    .methods
                    .get(name)
                    .map(|method| method.clone())
                    .map(|method| match method {
                        LoxValue::Func(ref callable) => LoxValue::Func(Rc::new(
                            callable
                                .as_any()
                                .downcast_ref::<LoxFunc>()
                                .expect("Couldn't cast Callable to LoxFunc in LoxValue::Func")
                                .bind(Rc::new(RefCell::new(self.clone()))),
                        )),
                        _ => panic!("Can't get non-func as method from an instance"),
                    })
            })
            .ok_or(RuntimeError::UndefinedProperty(name.to_string()))
    }

    pub fn set(&mut self, name: &str, value: LoxValue) {
        self.state.insert(name.to_string(), value);
    }

    pub fn get_class_name(&self) -> &str {
        &self.class.name
    }
}
