use crate::span::Span;
use crate::tokenize::Token;
use s::S;

#[derive(Debug, Eq, PartialEq)]
pub enum ListKind {
    Unordered,
    Ordered,
    Nil,
}

impl ListKind {
    pub fn from_token(token: &Token) -> Self {
        match token {
            &Token::Asterisk | &Token::Hyphen | &Token::Plus => ListKind::Unordered,
            &Token::Index(_n) => ListKind::Ordered,
            _ => ListKind::Nil,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ListLine {
    pub line: S<Span>,
    pub child_list: Box<List>,
}

impl ListLine {
    pub fn new(spans: S<Span>, child: List) -> Self {
        ListLine {
            line: spans,
            child_list: Box::new(child),
        }
    }

    pub fn next_list_target(token: &Token) -> Token {
        match token {
            &Token::Asterisk | &Token::Hyphen | &Token::Plus => token.clone(),
            &Token::Index(n) => Token::Index(n + 1),
            _ => unimplemented!(),
        }
    }

    pub fn need_parsed_targets(target: &Token) -> S<Token> {
        match target {
            &Token::Asterisk | &Token::Hyphen | &Token::Plus => {
                S::from_vector(vec![target.clone(), Token::Space])
            }
            &Token::Index(_) => S::from_vector(vec![target.clone(), Token::Dot, Token::Space]),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct List {
    pub kind: ListKind,
    pub list: S<ListLine>,
}

impl List {
    pub fn new(kind: ListKind, lines: S<ListLine>) -> Self {
        List {
            kind: kind,
            list: lines,
        }
    }

    pub fn nil() -> Self {
        List {
            kind: ListKind::Nil,
            list: S::Nil,
        }
    }
}
