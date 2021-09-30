use super::super::span::Span;
use crate::tokenize::Token;
use s::{Result, S};

pub fn link(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (_, tokens) = tokens.next_is_ignore(&Token::BlockBracketStart)?;
    let (label, tokens) = tokens.until_ignore(&Token::BlockBracketEnd)?;
    let (_, tokens) = tokens.next_is_ignore(&Token::BracketStart)?;
    let (href, tokens) = tokens.until_ignore(&Token::BracketEnd)?;
    Ok((Span::link(label.to_string(), href.to_string()), tokens))
}

#[test]
fn ts_link() {
    let src = "[example](https://example.com)";

    let tokens = crate::tokenize::parse(src).unwrap();
    let result = link(&tokens);
    assert_eq!(
        result.map(|v| v.0),
        Ok(Span::link("example", "https://example.com"))
    );
}
