use super::super::block::Block;
use crate::parser::error::parser::{ParseError, Result};
use crate::parser::s::S;
use crate::span;
use crate::tokenize::Token;

pub fn backquote(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let (_, tokens) =
        tokens.until_targets_ignore(S::from_vector(vec![Token::AngleBracketEnd, Token::Space]))?;
    let (depth, tokens) =
        tokens.many_ignore(S::from_vector(vec![Token::AngleBracketEnd, Token::Space]))?;

    let newline = tokens.until_include(Token::Newline);
    let eof = tokens.until_include(Token::EOF);
    let (src, tokens) = match (&newline, &eof) {
        (Ok(_), _) => newline,
        (Err(_), Ok(_)) => eof,
        (Err(_), Err(_)) => newline,
    }?;
    let (spans, _) = span::parse(&src)?;

    Ok((Block::Backquote((depth + 1) as u32, spans), tokens))
}

#[test]
fn ts_backquote() {
    use crate::span::Span;
    assert_eq!(
        backquote(&crate::tokenize::parse(r#"> > hello world"#).unwrap()).map(|v| v.0),
        Ok(Block::Backquote(2, S::unit(Span::text("hello world"))))
    );
}
