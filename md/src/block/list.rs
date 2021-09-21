use crate::parser::s::S;
use crate::span::Span;
use crate::tokenize::Token;

#[derive(Debug, Eq, PartialEq)]
pub enum ListLine {
    Unordered(S<Span>, Box<S<ListLine>>),
    Ordered(usize, S<Span>, Box<S<ListLine>>),
}

impl ListLine {
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
