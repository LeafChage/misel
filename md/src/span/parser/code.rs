use super::super::span::Span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{Mono, Result, S};

pub fn code(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (_, tokens) = tokens.next(&Mono::new(Token::BackQuote).ignore())?;
    let (code, tokens) = tokens.until(&Mono::new(Token::BackQuote).ignore())?;
    Ok((Span::code(code.show()), tokens))
}

#[test]
fn ts_code() {
    let src = r#"`console.log("hello world")`"#;
    let tokens = crate::tokenize::parse(src).unwrap();
    let result = code(&tokens);
    assert_eq!(
        result.map(|v| v.0),
        Ok(Span::code(r#"console.log("hello world")"#))
    );
}
