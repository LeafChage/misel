use super::how_to_handle::HowToHandle;
use super::or::{Or, OrParser};
use super::stream::Stream;
use super::Parser;
use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Token<T> {
    value: T,
    how_to_handle: HowToHandle,
}

impl<T> Parser<T, T> for Token<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
{
    fn parse<'a, 'b>(&self, s: &'a mut Stream<T>) -> Result<(Option<&'a T>, &'a Stream<T>)> {
        if let Some(v) = s.look() {
            if &self.value == v {
                let t = match self.how_to_handle {
                    HowToHandle::Leave => None,
                    HowToHandle::Include => s.next(),
                    HowToHandle::Ignore => {
                        let _ = s.next();
                        None
                    }
                };
                return Ok((t, s));
            }
        }
        Err(Error::from(ErrorKind::UnexpectedEof))
    }
}

impl<T, P> Or<T, P> for Token<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
    P: Parser<T, T>,
{
    fn or(&self, p: P) -> OrParser<T, P> {
        OrParser::new(vec![Token::new(self.value), p])
    }
}

impl<T> Token<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
{
    pub fn new(v: T) -> Self {
        Token {
            value: v,
            how_to_handle: HowToHandle::Include,
        }
    }
    pub fn ignore(&self) -> Self {
        Token {
            value: self.value,
            how_to_handle: HowToHandle::Ignore,
        }
    }
    pub fn leave(&self) -> Self {
        Token {
            value: self.value,
            how_to_handle: HowToHandle::Leave,
        }
    }
    pub fn include(&self) -> Self {
        Token {
            value: self.value,
            how_to_handle: HowToHandle::Include,
        }
    }
}

#[test]
fn ts_is_token() {
    assert_eq!(
        Token::new('a').parse(&S::from(vec!['a', 'b'])).unwrap(),
        (Some('a'), &S::from(vec!['b']))
    );
}
#[test]
fn ts_is_token_ignore() {
    assert_eq!(
        Token::new('a')
            .ignore()
            .parse(&S::from(vec!['a', 'b']))
            .unwrap(),
        (None, &S::from(vec!['b']))
    );
}
#[test]
fn ts_is_token_leave() {
    assert_eq!(
        Token::new('a')
            .leave()
            .parse(&S::from(vec!['a', 'b']))
            .unwrap(),
        (None, &S::from(vec!['a', 'b']))
    );
}
#[test]
fn ts_is_token_include() {
    assert_eq!(
        Token::new('a')
            .include()
            .parse(&S::from(vec!['a', 'b']))
            .unwrap(),
        (Some('a'), &S::from(vec!['b']))
    );
}
