use rlox::errors::Error;

#[derive(Debug)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
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
            Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
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

        Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
    }

    pub fn divide(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number / right_number));
            }
        }

        Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
    }

    pub fn multiply(&self, other: LoxValue) -> Result<LoxValue, Error> {
        if let LoxValue::Number(left_number) = *self {
            if let LoxValue::Number(right_number) = other {
                return Ok(LoxValue::Number(left_number * right_number));
            }
        }

        Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
    }

    pub fn plus(&self, other: LoxValue) -> Result<LoxValue, Error> {
        match *self {
            LoxValue::Number(left_number) => {
                if let LoxValue::Number(right_number) = other {
                    Ok(LoxValue::Number(left_number + right_number))
                } else {
                    Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                }
            }
            LoxValue::String(ref left_string) => {
                if let LoxValue::String(right_string) = other {
                    Ok(LoxValue::String(format!("{}{}", left_string, right_string)))
                } else {
                    Err(Error::UnexpectedEofError) // TODO: Change for some InterpreterError
                }
            }
            _ => Err(Error::UnexpectedEofError), // TODO: Change for some InterpreterError
        }
    }
}