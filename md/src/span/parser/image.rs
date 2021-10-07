use super::super::span::Span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{Result, S};

pub fn image(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (_, tokens) = tokens.next_is_ignore(Token::ExclamationMark)?;
    let (_, tokens) = tokens.next_is_ignore(Token::BlockBracketStart)?;
    let (alt, tokens) = tokens.until_ignore(Token::BlockBracketEnd)?;
    let (_, tokens) = tokens.next_is_ignore(Token::BracketStart)?;
    let (link, tokens) = tokens.until_ignore(Token::BracketEnd)?;
    Ok((Span::image(alt.show(), link.show()), tokens))
}

#[test]
fn ts_image() {
    let src = "![image](https://example.com/example.jpg)";
    let tokens = crate::tokenize::parse(src).unwrap();
    let result = image(&tokens);
    assert_eq!(
        result.map(|v| v.0),
        Ok(Span::image("image", "https://example.com/example.jpg",))
    );
}
