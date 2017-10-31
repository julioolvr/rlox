use std;

#[derive(Debug)]
pub enum Error {
    ScannerError(usize, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ScannerError(ref line, ref message) => {
                write!(f, "[line {}] ScannerError: {}", line, message)
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ScannerError(_, _) => "ScannerError",
        }
    }
}
