use super::S;
use std::fmt;

pub type Result<T> = std::result::Result<T, ScannerError>;

#[derive(Eq, PartialEq)]
pub struct ScannerError {
    msg: String,
}

impl ScannerError {
    pub fn unexpected(expected: &impl fmt::Debug, unexpected: &impl fmt::Debug) -> Self {
        ScannerError {
            msg: format!("expected: {:?}, unexpected: {:?}", expected, unexpected),
        }
    }

    pub fn not_found<T>(expected: &S<T>) -> Self
    where
        T: fmt::Debug + Eq + Clone,
    {
        ScannerError {
            msg: format!("not found {:?}", expected),
        }
    }

    pub fn end() -> Self {
        ScannerError {
            msg: String::from("What? list is finished"),
        }
    }

    pub fn message(m: impl Into<String>) -> Self {
        ScannerError { msg: m.into() }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl fmt::Debug for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScannerError {:?}", self.msg)

        // file!(),
        // line!()
        // programmer-facing output
    }
}
