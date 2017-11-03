use std;
use rlox::errors::Error;

#[derive(Debug, PartialEq)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LoxValue::Number(number) => write!(f, "{}", number),
            LoxValue::String(ref string) => write!(f, "{}", string),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Nil => f.write_str("nil"),
        }
    }
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match *self {
            LoxValue::Number(_) => true,
            LoxValue::String(_) => true,
            LoxValue::Bool(b) => b,
            LoxValue::Nil => false,
        }
    }

    pub fn negate_number(&self) -> Result<LoxValue, Error> {
        if let LoxValue::Number(number) = *self {
            Ok(LoxValue::Number(-number))
        } else {
            Err(Error::TypeError)
        }
    }

    pub fn negate(&self) -> Result<LoxValue, Error> {
        Ok(LoxValue::Bool(!self.is_truthy()))
    }

    pub fn subtract(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number - right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn divide(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number / right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn multiply(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number * right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn plus(&self, other: LoxValue) -> Result<LoxValue, Error> {
        match *self {
            LoxValue::Number(left_number) => {
                if let LoxValue::Number(right_number) = other {
                    Ok(LoxValue::Number(left_number + right_number))
                } else {
                    Err(Error::TypeError)
                }
            }
            LoxValue::String(ref left_string) => {
                if let LoxValue::String(right_string) = other {
                    Ok(LoxValue::String(format!("{}{}", left_string, right_string)))
                } else {
                    Err(Error::TypeError)
                }
            }
            _ => Err(Error::TypeError),
        }
    }

    pub fn is_greater(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number > right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn is_greater_equal(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number >= right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn is_less(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number < right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn is_less_equal(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Bool(left_number <= right_number));
            }
        }

        Err(Error::TypeError)
    }

    pub fn is_not_equal(&self, other: &LoxValue) -> Result<LoxValue, Error> {
        Ok(LoxValue::Bool(self != other))
    }

    pub fn is_equal(&self, other: &LoxValue) -> Result<LoxValue, Error> {
        Ok(LoxValue::Bool(self == other))
    }
}