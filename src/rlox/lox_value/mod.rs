mod errors;
mod lox_class;
mod lox_instance;
mod lox_func;

use std;
use std::rc::Rc;
use std::cell::RefCell;

use rlox::callables::Callable;
pub use self::lox_class::{LoxClass, LoxClassInternal};
pub use self::lox_instance::LoxInstance;
pub use self::lox_func::LoxFunc;
pub use self::errors::ValueError;

#[derive(Debug)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Func(Rc<Callable>),
    Class(Rc<LoxClass>),
    Instance(Rc<RefCell<LoxInstance>>),
    Nil,
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LoxValue::Number(number) => write!(f, "{}", number),
            LoxValue::String(ref string) => write!(f, "{}", string),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Func(_) => f.write_str("func"),
            LoxValue::Class(ref class) => write!(f, "class <{}>", class.get_name()),
            LoxValue::Instance(ref instance) => {
                write!(f, "instance of <{}>", instance.borrow().get_class_name())
            }
            LoxValue::Nil => f.write_str("nil"),
        }
    }
}

impl std::clone::Clone for LoxValue {
    fn clone(&self) -> LoxValue {
        match *self {
            LoxValue::Number(number) => LoxValue::Number(number),
            LoxValue::String(ref string) => LoxValue::String(string.clone()),
            LoxValue::Bool(b) => LoxValue::Bool(b),
            LoxValue::Nil => LoxValue::Nil,
            LoxValue::Func(ref func) => LoxValue::Func(func.clone()),
            LoxValue::Class(ref class) => LoxValue::Class(class.clone()),
            LoxValue::Instance(ref instance) => LoxValue::Instance(instance.clone()),
        }
    }
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match *self {
            LoxValue::Bool(b) => b,
            LoxValue::Nil => false,
            _ => true,
        }
    }

    pub fn negate_number(&self) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(number) = *self {
            Ok(LoxValue::Number(-number))
        } else {
            Err(ValueError::TypeError)
        }
    }

    pub fn negate(&self) -> Result<LoxValue, ValueError> {
        Ok(LoxValue::Bool(!self.is_truthy()))
    }

    pub fn subtract(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Number(left_number - right_number))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn divide(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                if right_number != 0.0 {
                    Ok(LoxValue::Number(left_number / right_number))
                } else {
                    Err(ValueError::DivideByZero)
                }
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn multiply(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Number(left_number * right_number))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn plus(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Number(left_number + right_number))
            }
            (&LoxValue::String(ref left_string), LoxValue::String(ref right_string)) => {
                Ok(LoxValue::String(format!("{}{}", left_string, right_string)))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn is_greater(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Bool(left_number > right_number))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn is_greater_equal(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Bool(left_number >= right_number))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn is_less(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Bool(left_number < right_number))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn is_less_equal(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match (self, other) {
            (&LoxValue::Number(left_number), LoxValue::Number(right_number)) => {
                Ok(LoxValue::Bool(left_number <= right_number))
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn is_not_equal(&self, other: &LoxValue) -> Result<LoxValue, ValueError> {
        let is_equal = self.is_equal(other)?;

        if let LoxValue::Bool(is_equal) = is_equal {
            Ok(LoxValue::Bool(!is_equal))
        } else {
            unreachable!()
        }
    }

    pub fn is_equal(&self, other: &LoxValue) -> Result<LoxValue, ValueError> {
        let result = match (self, other) {
            (&LoxValue::Number(number), &LoxValue::Number(other)) => number == other,
            (&LoxValue::String(ref string), &LoxValue::String(ref other)) => string == other,
            (&LoxValue::Bool(b), &LoxValue::Bool(other)) => b == other,
            (&LoxValue::Nil, &LoxValue::Nil) => true,
            (&LoxValue::Func(ref f), &LoxValue::Func(ref other)) => Rc::ptr_eq(f, other),
            (&LoxValue::Class(ref c), &LoxValue::Class(ref other)) => Rc::ptr_eq(c, other),
            (&LoxValue::Instance(ref i), &LoxValue::Instance(ref other)) => Rc::ptr_eq(i, other),
            _ => false,
        };

        Ok(LoxValue::Bool(result))
    }

    pub fn get_callable(&self) -> Option<Rc<Callable>> {
        match *self {
            LoxValue::Func(ref func) => Some(func.clone()),
            LoxValue::Class(ref class) => Some(class.clone()),
            _ => None,
        }
    }
}
