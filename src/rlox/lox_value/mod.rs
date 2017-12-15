mod errors;

use std;
use std::rc::Rc;
use rlox::callables::Callable;
pub use self::errors::ValueError;

#[derive(Debug)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Func(Rc<Callable>),
    Class(String),
    Nil,
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LoxValue::Number(number) => write!(f, "{}", number),
            LoxValue::String(ref string) => write!(f, "{}", string),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Func(_) => f.write_str("func"),
            LoxValue::Class(ref name) => write!(f, "class <{}>", name),
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
            LoxValue::Class(ref name) => LoxValue::Class(name.clone()),
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
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number - right_number));
            }
        }

        Err(ValueError::TypeError)
    }

    pub fn divide(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                if right_number != 0.0 {
                    return Ok(LoxValue::Number(left_number / right_number));
                } else {
                    return Err(ValueError::DivideByZero);
                }
            }
        }

        Err(ValueError::TypeError)
    }

    pub fn multiply(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number * right_number));
            }
        }

        Err(ValueError::TypeError)
    }

    pub fn plus(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        match *self {
            LoxValue::Number(left_number) => {
                if let LoxValue::Number(right_number) = other {
                    Ok(LoxValue::Number(left_number + right_number))
                } else {
                    Err(ValueError::TypeError)
                }
            }
            LoxValue::String(ref left_string) => {
                if let LoxValue::String(right_string) = other {
                    Ok(LoxValue::String(format!("{}{}", left_string, right_string)))
                } else {
                    Err(ValueError::TypeError)
                }
            }
            _ => Err(ValueError::TypeError),
        }
    }

    pub fn is_greater(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number > right_number));
            }
        }

        Err(ValueError::TypeError)
    }

    pub fn is_greater_equal(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number >= right_number));
            }
        }

        Err(ValueError::TypeError)
    }

    pub fn is_less(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number < right_number));
            }
        }

        Err(ValueError::TypeError)
    }

    pub fn is_less_equal(&self, other: LoxValue) -> Result<LoxValue, ValueError> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number <= right_number));
            }
        }

        Err(ValueError::TypeError)
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
        let result = match *self {
            LoxValue::Number(number) => {
                match *other {
                    LoxValue::Number(other) => number == other,
                    _ => false,
                }
            }
            LoxValue::String(ref string) => {
                match *other {
                    LoxValue::String(ref other) => string == other,
                    _ => false,
                }
            }
            LoxValue::Bool(b) => {
                match *other {
                    LoxValue::Bool(other) => b == other,
                    _ => false,
                }
            }
            LoxValue::Nil => {
                match *other {
                    LoxValue::Nil => true,
                    _ => false,
                }
            }
            // TODO: Figure out how to check if two `Rc`s reference the same value
            LoxValue::Func(_) => false,
            LoxValue::Class(_) => false, // TODO: Or is it?
        };

        Ok(LoxValue::Bool(result))
    }

    pub fn get_callable(&self) -> Option<Rc<Callable>> {
        match *self {
            LoxValue::Func(ref func) => Some(func.clone()),
            _ => None,
        }
    }
}