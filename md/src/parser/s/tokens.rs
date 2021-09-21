#[warn(unused_variables)]
use super::s::S;
use crate::parser::error::parser::{ParseError, Result};
use crate::tokenize::Token;
use std::iter::Iterator;

#[derive(Clone, Debug)]
enum HowToHandle {
    /// abcd until b return (a, cd)
    Ignore,

    /// abcd until b return (a, bcd)
    Leave,

    /// abcd until b return (ab, cd)
    Include,
}

impl S<Token> {
    pub fn string(&self) -> String {
        if let Some(head) = self.head() {
            format!("{}{}", head.v(), self.tail().string())
        } else {
            String::new()
        }
    }

    pub fn chmop(&self) -> S<Token> {
        let head = self.head();
        match head {
            Some(head) => match head {
                Token::Space | Token::Newline => {
                    let s = self.tail().chmop();
                    if s == S::Nil {
                        S::Nil
                    } else {
                        S::cons(head.clone(), s)
                    }
                }
                _ => S::cons(head.clone(), self.tail().chmop()),
            },
            None => S::Nil,
        }
    }

    pub fn next_is_ignore(&self, target: Token) -> Result<(&Token, &S<Token>)> {
        self.next_is(target, HowToHandle::Ignore)
    }

    pub fn next_is_leave(&self, target: Token) -> Result<(&Token, &S<Token>)> {
        self.next_is(target, HowToHandle::Leave)
    }

    fn next_is(&self, target: Token, how_to_handle: HowToHandle) -> Result<(&Token, &S<Token>)> {
        if let Some(head) = self.head() {
            if head == &target {
                match how_to_handle {
                    HowToHandle::Ignore => Ok((head, self.tail())),
                    HowToHandle::Leave => Ok((head, self)),
                    HowToHandle::Include => panic!("unexpected"),
                }
            } else {
                Err(ParseError::new(target, head))
            }
        } else {
            Err(ParseError::eof(vec![target]))
        }
    }

    pub fn next_are_ignore<'a>(&'a self, targets: S<Token>) -> Result<(S<Token>, &'a S<Token>)> {
        self.next_are(targets, HowToHandle::Ignore)
    }

    pub fn next_are_leave<'a>(&'a self, targets: S<Token>) -> Result<(S<Token>, &'a S<Token>)> {
        self.next_are(targets, HowToHandle::Leave)
    }

    fn next_are<'a>(
        &'a self,
        targets: S<Token>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<Token>, &'a S<Token>)> {
        let ok = self
            .zip_with(&targets, |t, target| t == target)
            .fold(true, |a, b| a && *b);

        if ok {
            match how_to_handle {
                HowToHandle::Ignore => {
                    let length = targets.length();
                    Ok((targets, self.tail_after(length)))
                }
                HowToHandle::Leave => Ok((targets, self)),
                HowToHandle::Include => panic!("unexpected"),
            }
        } else {
            Err(ParseError::not_found(targets.to_vector()))
        }
    }

    pub fn to_skip_ignore(&self, target: Token) -> Result<&S<Token>> {
        self.to_skip(target, HowToHandle::Ignore)
    }

    pub fn to_skip_leave(&self, target: Token) -> Result<&S<Token>> {
        self.to_skip(target, HowToHandle::Leave)
    }

    fn to_skip(&self, target: Token, how_to_handle: HowToHandle) -> Result<&S<Token>> {
        if let Some(head) = self.head() {
            Ok(if head == &target {
                match how_to_handle {
                    HowToHandle::Ignore => self.tail(),
                    HowToHandle::Leave => self,
                    HowToHandle::Include => panic!("unexpected"),
                }
            } else {
                self.tail().to_skip(target, how_to_handle)?
            })
        } else {
            Err(ParseError::eof(vec![target]))
        }
    }

    pub fn until_ignore(&self, target: Token) -> Result<(S<Token>, &S<Token>)> {
        self.until(target, HowToHandle::Ignore)
    }

    pub fn until_leave(&self, target: Token) -> Result<(S<Token>, &S<Token>)> {
        self.until(target, HowToHandle::Leave)
    }

    pub fn until_include(&self, target: Token) -> Result<(S<Token>, &S<Token>)> {
        self.until(target, HowToHandle::Include)
    }

    fn until(&self, target: Token, how_to_handle: HowToHandle) -> Result<(S<Token>, &S<Token>)> {
        if let Some(head) = self.head() {
            Ok(if head == &target {
                match how_to_handle {
                    HowToHandle::Ignore => (S::Nil, self.tail()),
                    HowToHandle::Leave => (S::Nil, self),
                    HowToHandle::Include => (S::unit(head.clone()), self.tail()),
                }
            } else {
                let (car, cdr) = self.tail().until(target, how_to_handle)?;
                (S::cons(head.clone(), car), cdr)
            })
        } else {
            Err(ParseError::eof(vec![target]))
        }
    }

    pub fn until_targets_ignore(&self, targets: S<Token>) -> Result<(S<Token>, &S<Token>)> {
        self.until_targets(targets, HowToHandle::Ignore)
    }
    pub fn until_targets_leave(&self, targets: S<Token>) -> Result<(S<Token>, &S<Token>)> {
        self.until_targets(targets, HowToHandle::Leave)
    }

    pub fn until_targets_include(&self, targets: S<Token>) -> Result<(S<Token>, &S<Token>)> {
        self.until_targets(targets, HowToHandle::Include)
    }

    fn until_targets(
        &self,
        targets: S<Token>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<Token>, &S<Token>)> {
        let ok = self
            .zip_with(&targets, |t, target| t == target)
            .fold(true, |a, b| a && *b);

        if let Some(head) = self.head() {
            if ok {
                match how_to_handle {
                    HowToHandle::Ignore => {
                        let length = targets.length();
                        Ok((S::Nil, self.tail_after(length)))
                    }
                    HowToHandle::Leave => Ok((S::Nil, self)),
                    HowToHandle::Include => {
                        let length = targets.length();
                        Ok((targets, self.tail_after(length)))
                    }
                }
            } else {
                let (car, cdr) = self.tail().until_targets(targets, how_to_handle)?;
                Ok((S::cons(head.clone(), car), cdr))
            }
        } else {
            Err(ParseError::eof(vec![]))
        }
    }

    pub fn to_somewhere_ignore(&self, targets: Vec<Token>) -> Result<(S<Token>, &S<Token>)> {
        self.to_somewhere(targets, HowToHandle::Ignore)
    }

    pub fn to_somewhere_leave(&self, targets: Vec<Token>) -> Result<(S<Token>, &S<Token>)> {
        self.to_somewhere(targets, HowToHandle::Leave)
    }

    pub fn to_somewhere_include(&self, targets: Vec<Token>) -> Result<(S<Token>, &S<Token>)> {
        self.to_somewhere(targets, HowToHandle::Include)
    }

    fn to_somewhere(
        &self,
        targets: Vec<Token>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<Token>, &S<Token>)> {
        if let Some(head) = self.head() {
            let include = targets.iter().fold(None, |result, current| match result {
                Some(_) => result,
                None => {
                    if current == head {
                        Some(current)
                    } else {
                        None
                    }
                }
            });

            Ok(match include {
                Some(_target) => match how_to_handle {
                    HowToHandle::Ignore => (S::Nil, self.tail()),
                    HowToHandle::Leave => (S::Nil, self),
                    HowToHandle::Include => (S::unit(head.clone()), self.tail()),
                },
                None => {
                    let (car, cdr) = self.tail().to_somewhere(targets, how_to_handle)?;
                    (S::cons(head.clone(), car), cdr)
                }
            })
        } else {
            Err(ParseError::eof(targets))
        }
    }

    pub fn many_ignore<'a>(&'a self, targets: S<Token>) -> Result<(usize, &'a S<Token>)> {
        self.many(targets, HowToHandle::Ignore)
    }
    pub fn many_leave<'a>(&'a self, targets: S<Token>) -> Result<(usize, &'a S<Token>)> {
        self.many(targets, HowToHandle::Leave)
    }

    fn many<'a>(
        &'a self,
        targets: S<Token>,
        how_to_handle: HowToHandle,
    ) -> Result<(usize, &'a S<Token>)> {
        let mut tail = self;
        let mut count = 0;
        while tail
            .zip_with(&targets, |t, target| t == target)
            .fold(true, |a, b| a && *b)
        {
            count += 1;
            let length = targets.length();
            tail = tail.tail_after(length);
        }

        Ok(match how_to_handle {
            HowToHandle::Ignore => (count, tail),
            HowToHandle::Leave => (count, self),
            HowToHandle::Include => unimplemented!(),
        })
    }

    pub fn many1_ignore<'a>(&'a self, targets: S<Token>) -> Result<(usize, &'a S<Token>)> {
        self.many1(targets, HowToHandle::Ignore)
    }
    pub fn many1_leave<'a>(&'a self, targets: S<Token>) -> Result<(usize, &'a S<Token>)> {
        self.many1(targets, HowToHandle::Leave)
    }

    fn many1<'a>(
        &'a self,
        targets: S<Token>,
        how_to_handle: HowToHandle,
    ) -> Result<(usize, &'a S<Token>)> {
        let (count, tokens) = self.many(targets, how_to_handle)?;
        if count == 0 {
            Err(ParseError::unexpected())
        } else {
            Ok((count, tokens))
        }
    }

    pub fn next_are_or_ignore<'a>(
        &'a self,
        someone: Vec<S<Token>>,
    ) -> Result<(S<Token>, &'a S<Token>)> {
        self.next_are_or(someone, HowToHandle::Ignore)
    }

    pub fn next_are_or_leave<'a>(
        &'a self,
        someone: Vec<S<Token>>,
    ) -> Result<(S<Token>, &'a S<Token>)> {
        self.next_are_or(someone, HowToHandle::Leave)
    }

    fn next_are_or<'a>(
        &'a self,
        someone: Vec<S<Token>>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<Token>, &'a S<Token>)> {
        for t in someone.into_iter() {
            let result = self.next_are(t, how_to_handle.clone());
            if let Ok(_) = result {
                return result;
            }
        }

        Err(ParseError::message(format!("next_are_or")))
    }
}

#[test]
fn ts_chmop() {
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Asterisk, Token::Space,]).chmop(),
        S::from_vector(vec![Token::Sharp, Token::Asterisk,])
    );
}

#[test]
fn ts_next_is() {
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Asterisk,]).next_is_ignore(Token::Sharp),
        Ok((&Token::Sharp, &S::from_vector(vec![Token::Asterisk,])))
    );
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Asterisk,]).next_is_leave(Token::Sharp),
        Ok((
            &Token::Sharp,
            &S::from_vector(vec![Token::Sharp, Token::Asterisk,])
        ))
    );
}

#[test]
fn ts_next_are() {
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Sharp, Token::Asterisk])
            .next_are_ignore(S::from_vector(vec![Token::Sharp, Token::Sharp])),
        Ok((
            S::from_vector(vec![Token::Sharp, Token::Sharp]),
            &S::from_vector(vec![Token::Asterisk])
        ))
    );
}

#[test]
fn ts_to_skip() {
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Asterisk, Token::Dot,])
            .to_skip_ignore(Token::Asterisk),
        Ok(&S::from_vector(vec![Token::Dot]))
    );
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Asterisk, Token::Dot,])
            .to_skip_leave(Token::Asterisk),
        Ok(&S::from_vector(vec![Token::Asterisk, Token::Dot]))
    );
}

#[test]
fn ts_until() {
    assert_eq!(
        S::from_vector(vec![
            Token::Sharp,
            Token::Asterisk,
            Token::Dot,
            Token::Asterisk,
        ])
        .until_ignore(Token::Asterisk),
        Ok((
            S::from_vector(vec![Token::Sharp]),
            &S::from_vector(vec![Token::Dot, Token::Asterisk,])
        ))
    );
    assert_eq!(
        S::from_vector(vec![
            Token::Sharp,
            Token::Asterisk,
            Token::Dot,
            Token::Asterisk,
        ])
        .until_leave(Token::Asterisk),
        Ok((
            S::from_vector(vec![Token::Sharp]),
            &S::from_vector(vec![Token::Asterisk, Token::Dot, Token::Asterisk,])
        ))
    );
    assert_eq!(
        S::from_vector(vec![
            Token::Sharp,
            Token::Asterisk,
            Token::Dot,
            Token::Asterisk,
        ])
        .until_include(Token::Asterisk),
        Ok((
            S::from_vector(vec![Token::Sharp, Token::Asterisk]),
            &S::from_vector(vec![Token::Dot, Token::Asterisk])
        ))
    );
}

#[test]
fn ts_until_targets() {
    assert_eq!(
        S::from_vector(vec![
            Token::Sharp,
            Token::Asterisk,
            Token::Dot,
            Token::Asterisk,
        ])
        .until_targets_ignore(S::from_vector(vec![Token::Dot, Token::Asterisk])),
        Ok((
            S::from_vector(vec![Token::Sharp, Token::Asterisk]),
            &S::from_vector(vec![])
        ))
    );
    assert_eq!(
        S::from_vector(vec![
            Token::Sharp,
            Token::Asterisk,
            Token::Dot,
            Token::Asterisk,
        ])
        .until_targets_leave(S::from_vector(vec![Token::Dot, Token::Asterisk])),
        Ok((
            S::from_vector(vec![Token::Sharp, Token::Asterisk]),
            &S::from_vector(vec![Token::Dot, Token::Asterisk])
        ))
    );
    assert_eq!(
        S::from_vector(vec![
            Token::Sharp,
            Token::Asterisk,
            Token::Dot,
            Token::Asterisk,
        ])
        .until_targets_include(S::from_vector(vec![Token::Dot, Token::Asterisk])),
        Ok((
            S::from_vector(vec![
                Token::Sharp,
                Token::Asterisk,
                Token::Dot,
                Token::Asterisk
            ]),
            &S::from_vector(vec![])
        ))
    );
}

#[test]
fn ts_many() {
    assert_eq!(
        S::from_vector(vec![
            Token::Space,
            Token::Sharp,
            Token::Space,
            Token::Sharp,
            Token::Space,
            Token::Sharp,
            Token::text("chage"),
        ])
        .many_ignore(S::from_vector(vec![Token::Space, Token::Sharp])),
        Ok((3, &S::from_vector(vec![Token::text("chage"),])))
    );
    assert_eq!(
        S::from_vector(vec![
            Token::Space,
            Token::Sharp,
            Token::Space,
            Token::Sharp,
            Token::Space,
            Token::Sharp,
            Token::text("chage"),
        ])
        .many_leave(S::from_vector(vec![Token::Space, Token::Sharp])),
        Ok((
            3,
            &S::from_vector(vec![
                Token::Space,
                Token::Sharp,
                Token::Space,
                Token::Sharp,
                Token::Space,
                Token::Sharp,
                Token::text("chage"),
            ])
        ))
    );
}

#[test]
fn ts_next_are_or() {
    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Sharp, Token::Asterisk]).next_are_or_ignore(vec![
            S::from_vector(vec![Token::Space, Token::Space]),
            S::from_vector(vec![Token::Sharp, Token::Sharp])
        ]),
        Ok((
            S::from_vector(vec![Token::Sharp, Token::Sharp]),
            &S::from_vector(vec![Token::Asterisk])
        ))
    );

    assert_eq!(
        S::from_vector(vec![Token::Sharp, Token::Sharp, Token::Asterisk]).next_are_or_ignore(vec![
            S::from_vector(vec![Token::Sharp]),
            S::from_vector(vec![Token::Sharp, Token::Sharp])
        ]),
        Ok((
            S::from_vector(vec![Token::Sharp]),
            &S::from_vector(vec![Token::Sharp, Token::Asterisk])
        ))
    );
}
