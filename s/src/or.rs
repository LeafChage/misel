use super::token::Token;
use super::{Parser, S};
use std::fmt;
use std::io::{Error, ErrorKind, Result};
use std::marker::PhantomData;

pub trait Or<T, P>
where
    T: Eq + fmt::Debug + Clone + Copy,
    P: Parser<T, T>,
{
    fn or(&self, p: P) -> OrParser<T, P>;
}

pub struct OrParser<T, P>
where
    T: Eq + fmt::Debug + Clone + Copy,
    P: Parser<T, T>,
{
    _marker: PhantomData<T>,
    values: Vec<P>,
}

impl<T, P> OrParser<T, P>
where
    T: Eq + fmt::Debug + Clone + Copy,
    P: Parser<T, T>,
{
    pub fn new(values: Vec<P>) -> Self {
        OrParser {
            _marker: PhantomData,
            values: values,
        }
    }

    pub fn or(mut self, v: P) -> Self {
        self.values.push(v);
        OrParser::new(self.values)
    }
}

impl<T, P> Parser<T, T> for OrParser<T, P>
where
    T: Eq + fmt::Debug + Clone + Copy,
    P: Parser<T, T>,
{
    fn parse<'a, 'b>(&'a self, s: &'b S<T>) -> Result<(Option<T>, &'b S<T>)> {
        for v in self.values.iter() {
            if let Ok(v) = v.parse(s) {
                return Ok(v);
            }
        }
        Err(Error::from(ErrorKind::UnexpectedEof))
    }
}

// #[test]
// fn ts_is_or() {
//     use super::token::Token;
//
//     assert_eq!(
//         Token::new('a')
//             .or(Token::new('b'))
//             .parse(&S::from(vec!['a', 'b', 'c'])),
//         Some('a')
//     );
//     assert_eq!(
//         Token::new('a')
//             .or(Token::new('b'))
//             .parse(&S::from(vec!['b', 'c'])),
//         Some('b')
//     );
//     assert_eq!(
//         Token::<char>::new('a')
//             .or(Token::new('b'))
//             .or(Token::new('c'))
//             .parse(&S::from(vec!['c', 'd'])),
//         Some('c')
//     );
//     assert_eq!(
//         Token::new('a')
//             .or(Token::new('b'))
//             .or(Token::new('c'))
//             .parse(&S::from(vec!['d', 'e'])),
//         None
//     );
// }
