use super::{Parser, S};
use std::fmt;
use std::marker::PhantomData;

pub struct And<T, S>
where
    T: Eq + fmt::Debug + Clone + Copy,
    S: Parser<T, Vec<T>>,
{
    _marker: PhantomData<T>,
    values: Vec<S>,
}

impl<T, S> And<T, S>
where
    T: Eq + fmt::Debug + Clone + Copy,
    S: Parser<T, Vec<T>>,
{
    pub fn new(values: Vec<S>) -> Self {
        And {
            _marker: PhantomData,
            values: values,
        }
    }

    // fn or(mut self, v: S) -> Self {
    //     self.values.push(v);
    //     And::new(self.values)
    // }
}

impl<T, P> Parser<T, Vec<T>> for And<T, P>
where
    T: Eq + fmt::Debug + Clone + Copy,
    P: Parser<T, Vec<T>>,
{
    fn parse(&self, s: &S<T>) -> Option<Vec<T>> {
        let v = vec![];
        let mut tail = s;
        for value in self.values.iter() {
            if let Some(head) = tail.head() {
                if let Some(result) = value.parse(s) {
                    v.push(result.clone())
                } else {
                    return None;
                }
            }
        }
        Some(v)
    }
}

#[test]
fn ts_is_and() {
    use super::token::Token;
    assert_eq!(
        Token::new('a')
            .or(Token::new('b'))
            .parse(&S::from(vec!['a', 'b', 'c'])),
        Some('a')
    );
    assert_eq!(
        Token::new('a')
            .or(Token::new('b'))
            .parse(&S::from(vec!['b', 'c'])),
        Some('b')
    );
    assert_eq!(
        Token::<char>::new('a')
            .or(Token::new('b'))
            .or(Token::new('c'))
            .parse(&S::from(vec!['c', 'd'])),
        Some('c')
    );
    assert_eq!(
        Token::new('a')
            .or(Token::new('b'))
            .or(Token::new('c'))
            .parse(&S::from(vec!['d', 'e'])),
        None
    );
}
