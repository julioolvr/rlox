use std;

#[derive(Debug)]
pub enum ValueError {
    TypeError,
    DivideByZero,
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ValueError::TypeError => f.write_str("TypeError"),
            ValueError::DivideByZero => f.write_str("DivideByZero"),
        }
    }
}

impl std::error::Error for ValueError {
    fn description(&self) -> &str {
        match *self {
            ValueError::TypeError => "TypeError",
            ValueError::DivideByZero => "DivideByZero",
        }
    }
}
