use std;

#[derive(Debug)]
pub enum ScannerError {
    ScannerError(usize, String),
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ScannerError::ScannerError(ref line, ref message) => {
                write!(f, "[line {}] ScannerError: {}", line, message)
            }
        }
    }
}

impl std::error::Error for ScannerError {
    fn description(&self) -> &str {
        match *self {
            ScannerError::ScannerError(_, _) => "ScannerError",
        }
    }
}