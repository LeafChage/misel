use super::super::emphasis::{EmphasisToken, EmphasisType};
use super::super::span::Span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{Result, ScannerError, S};

fn emphasis_with_target(
    tokens: &S<Token>,
    target: EmphasisToken,
    expect: EmphasisType,
) -> Result<(Span, &S<Token>)> {
    let v = target.token();
    let (_, tokens) = tokens.next_is_ignore(v.clone())?;
    if let Ok((span, tokens)) = emphasis_with_target(tokens, target, expect.other()) {
        let (_, tokens) = tokens.until_ignore(v)?;
        Ok((span, tokens))
    } else {
        let (value, tokens) = tokens.until_ignore(v.clone())?;
        Ok((Span::emphasis(expect, value.show()), tokens))
    }
}

pub fn emphasis(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let head = tokens.head();
    match head {
        Some(Token::Asterisk) => {
            emphasis_with_target(tokens, EmphasisToken::Asterisk, EmphasisType::Strong)
        }
        Some(Token::UnderScore) => {
            emphasis_with_target(tokens, EmphasisToken::UnderScore, EmphasisType::Strong)
        }
        Some(n) => Err(ScannerError::unexpected(&Token::Asterisk, n)),
        None => Err(ScannerError::end()),
    }
}

#[test]
fn ts_emphasis() {
    assert_eq!(
        emphasis(&crate::tokenize::parse("**hello**").unwrap(),).map(|v| v.0),
        Ok(Span::emphasis(EmphasisType::Emphasis, "hello"))
    );

    assert_eq!(
        emphasis(&crate::tokenize::parse("__hello__").unwrap(),).map(|v| v.0),
        Ok(Span::emphasis(EmphasisType::Emphasis, "hello"))
    );

    assert_eq!(
        emphasis(&crate::tokenize::parse("*world*").unwrap(),).map(|v| v.0),
        Ok(Span::emphasis(EmphasisType::Strong, "world"))
    );
}
