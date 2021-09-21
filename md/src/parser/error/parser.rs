use crate::tokenize::Token;
use std::fmt;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Eq, PartialEq)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(expected: Token, unexptected: &Token) -> Self {
        ParseError {
            msg: format!(
                "expected: {:?} / unexptected: {:?}",
                vec![expected.clone()],
                vec![unexptected]
            ),
        }
    }

    pub fn message(msg: impl Into<String>) -> Self {
        ParseError { msg: msg.into() }
    }

    pub fn eof(expected: Vec<Token>) -> Self {
        ParseError {
            msg: format!(
                "expected: {:?} / unexptected: {:?}",
                expected,
                vec![Token::EOF]
            ),
        }
    }

    pub fn not_found(expected: Vec<&Token>) -> Self {
        // unimplemented!();
        ParseError {
            msg: format!("expected: {:?}", expected),
        }
    }

    pub fn unexpected() -> Self {
        // unimplemented!();
        ParseError {
            msg: format!("unexpected"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError {:?}", self.msg)

        // file!(),
        // line!()
        // programmer-facing output
    }
}
