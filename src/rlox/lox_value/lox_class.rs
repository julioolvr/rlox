use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use rlox::callables::Callable;
use rlox::interpreter::Interpreter;
use rlox::interpreter::errors::RuntimeError;
use rlox::lox_value::{LoxFunc, LoxInstance, LoxValue};

#[derive(Debug)]
pub struct LoxClass {
    internal: Rc<LoxClassInternal>,
}

#[derive(Debug)]
pub struct LoxClassInternal {
    pub name: String,
    pub superclass: Option<Rc<LoxClass>>,
    pub methods: HashMap<String, LoxValue>,
}

impl LoxClassInternal {
    pub fn find_method(&self, name: &str, instance: Rc<RefCell<LoxInstance>>) -> Option<LoxFunc> {
        self.methods
            .get(name)
            .map(|method| method.clone())
            .map(|method| match method {
                LoxValue::Func(ref callable) => callable
                    .as_any()
                    .downcast_ref::<LoxFunc>()
                    .expect("Couldn't cast Callable to LoxFunc in LoxValue::Func")
                    .bind(instance.clone()),
                _ => panic!("Can't get non-func as method from an instance"),
            })
            .or_else(|| {
                if let Some(superclass) = self.superclass.clone() {
                    superclass.find_method(name, instance)
                } else {
                    None
                }
            })
    }
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, LoxValue>,
    ) -> LoxClass {
        LoxClass {
            internal: Rc::new(LoxClassInternal {
                name,
                superclass,
                methods,
            }),
        }
    }

    pub fn instantiate(&self) -> Result<LoxInstance, RuntimeError> {
        Ok(LoxInstance::new(self.internal.clone()))
    }

    pub fn get_name(&self) -> &str {
        &self.internal.name
    }

    pub fn find_method(&self, name: &str, instance: Rc<RefCell<LoxInstance>>) -> Option<LoxFunc> {
        self.internal.find_method(name, instance)
    }
}

impl Callable for LoxClass {
    fn as_any(&self) -> &Any {
        self
    }

    fn arity(&self) -> usize {
        let initializer = self.internal.methods.get("init");
        if let Some(init) = initializer {
            match init {
                &LoxValue::Func(ref callable) => callable.arity(),
                _ => panic!("Can't get non-func as method from an instance"),
            }
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        let instance = Rc::new(RefCell::new(self.instantiate()?));

        if let Some(init) = self.internal.find_method("init", instance.clone()) {
            init.call(interpreter, arguments)?;
        };

        Ok(LoxValue::Instance(instance))
    }
}
