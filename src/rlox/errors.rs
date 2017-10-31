use std;

#[derive(Debug)]
pub struct InterpreterError {
    line: usize,
    filename: String,
    message: String,
}

impl InterpreterError {
    pub fn new(line: usize, filename: String, message: String) -> InterpreterError {
        InterpreterError {
            line,
            filename,
            message,
        }
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "[line {}] Error {}: {}",
               self.line,
               self.filename,
               self.message)
    }
}

impl std::error::Error for InterpreterError {
    fn description(&self) -> &str {
        "InterpreterError"
    }
}