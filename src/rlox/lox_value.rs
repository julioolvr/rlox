#[derive(Debug)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil
}

impl LoxValue {
  pub fn is_truthy(&self) -> bool {
    match *self {
      LoxValue::Number(_) => true,
      LoxValue::String(_) => true,
      LoxValue::Bool(b) => b,
      LoxValue::Nil => false
    }
  }
}