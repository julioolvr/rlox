use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use rlox::callables::Callable;
use rlox::callables::LoxFunc;
use rlox::interpreter::Interpreter;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::{LoxInstance, LoxValue};

#[derive(Debug)]
pub struct LoxClass {
    internal: Rc<LoxClassInternal>,
}

#[derive(Debug)]
pub struct LoxClassInternal {
    pub name: String,
    pub methods: HashMap<String, LoxValue>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxValue>) -> LoxClass {
        LoxClass {
            internal: Rc::new(LoxClassInternal { name, methods }),
        }
    }

    pub fn instantiate(&self) -> Result<LoxInstance, RuntimeError> {
        Ok(LoxInstance::new(self.internal.clone()))
    }

    pub fn get_name(&self) -> &str {
        &self.internal.name
    }
}

impl Callable for LoxClass {
    fn as_any(&self) -> &Any {
        self
    }
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        let instance = Rc::new(RefCell::new(self.instantiate()?));

        let initializer = self.internal.methods.get("init");
        if let Some(init) = initializer {
            match init.clone() {
                LoxValue::Func(ref callable) => callable
                    .as_any()
                    .downcast_ref::<LoxFunc>()
                    .expect("Couldn't cast Callable to LoxFunc in LoxValue::Func")
                    .bind(instance.clone())
                    .call(interpreter, arguments)?,
                _ => panic!("Can't get non-func as method from an instance"),
            };
        }

        Ok(LoxValue::Instance(instance))
    }
}
