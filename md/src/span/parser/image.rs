use super::super::span::Span;
use crate::parser::error::parser::Result;
use crate::parser::s::S;
use crate::tokenize::Token;

pub fn image(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (_, tokens) = tokens.next_is_ignore(Token::ExclamationMark)?;
    let (_, tokens) = tokens.next_is_ignore(Token::BlockBracketStart)?;
    let (src, tokens) = tokens.until_ignore(Token::BlockBracketEnd)?;
    let (_, tokens) = tokens.next_is_ignore(Token::BracketStart)?;
    let (alt, tokens) = tokens.until_ignore(Token::BracketEnd)?;
    Ok((Span::image(src.string(), alt.string()), tokens))
}

#[test]
fn ts_image() {
    let src = "![https://example.com/example.jpg](image)";
    let tokens = crate::tokenize::parse(src).unwrap();
    let result = image(&tokens);
    assert_eq!(
        result.map(|v| v.0),
        Ok(Span::image("https://example.com/example.jpg", "image"))
    );
}
