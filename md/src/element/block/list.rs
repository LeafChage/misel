use crate::element::{Span, S};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ListToken {
    Asterisk,         // *
    Hyphen,           // -
    Plus,             // +
    Numbering(isize), // 1
}

impl ListToken {
    pub fn token(&self) -> char {
        match self {
            ListToken::Asterisk => '*',
            ListToken::Hyphen => '-',
            ListToken::Plus => '+',
            ListToken::Numbering(n) => std::char::from_digit(*n as u32, 10).unwrap(),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ListToken::Asterisk => ListToken::Asterisk,
            ListToken::Hyphen => ListToken::Hyphen,
            ListToken::Plus => ListToken::Plus,
            ListToken::Numbering(n) => ListToken::Numbering(n + 1),
        }
    }
}

pub type ListUnit = (ListToken, S<Span>, Box<List>);

#[derive(Debug, Eq, PartialEq)]
pub struct List(S<ListUnit>);

impl List {
    pub fn cons(head: (ListToken, S<Span>, List), tail: S<ListUnit>) -> Self {
        let (t, spans, l) = head;
        List(S::cons((t, spans, Box::new(l)), tail))
    }
    pub fn unit(head: (ListToken, S<Span>, List)) -> Self {
        let (t, spans, l) = head;
        List(S::cons((t, spans, Box::new(l)), S::Nil))
    }
    pub fn nil() -> Self {
        List(S::Nil)
    }
}
