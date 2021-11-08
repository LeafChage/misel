use super::super::span::Span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{Mono, Result, S};

pub fn image(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (_, tokens) = tokens.next(&Mono::new(Token::ExclamationMark).ignore())?;
    let (_, tokens) = tokens.next(&Mono::new(Token::BlockBracketStart).ignore())?;
    let (alt, tokens) = tokens.until(&Mono::new(Token::BlockBracketEnd).ignore())?;
    let (_, tokens) = tokens.next(&Mono::new(Token::BracketStart).ignore())?;
    let (link, tokens) = tokens.until(&Mono::new(Token::BracketEnd).ignore())?;
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
