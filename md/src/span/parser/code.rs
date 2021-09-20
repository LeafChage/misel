use super::super::span::Span;
use crate::parser::error::parser::Result;
use crate::parser::s::S;
use crate::tokenize::Token;

pub fn code(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (_, tokens) = tokens.next_is_ignore(Token::BackQuote)?;
    let (code, tokens) = tokens.until_ignore(Token::BackQuote)?;
    Ok((Span::code(code.string()), tokens))
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
