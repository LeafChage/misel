use super::super::span::Span;
use crate::parser::error::parser::Result;
use crate::parser::s::S;
use crate::tokenize::Token;

pub fn text(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let (src, tokens) = if let Ok((src, tokens)) = tokens.to_somewhere_leave(vec![
        Token::BlockBracketStart,
        Token::UnderScore,
        Token::Asterisk,
        Token::BackQuote,
        Token::ExclamationMark,
        Token::Newline,
        Token::EOF,
    ]) {
        (src, tokens)
    } else {
        let src = tokens.to_end();
        (src, &S::Nil)
    };

    let chmoped_text = src.chmop();
    if chmoped_text.length() == 0 {
        // only space
        Ok((Span::text(src.string()), tokens))
    } else {
        Ok((Span::text(chmoped_text.string()), tokens))
    }
}

#[test]
fn ts_text() {
    assert_eq!(
        text(&crate::tokenize::parse("hello2![]").unwrap(),).map(|v| v.0),
        Ok(Span::text("hello2")),
    );
    assert_eq!(
        text(&crate::tokenize::parse("hello").unwrap(),).map(|v| v.0),
        Ok(Span::text("hello")),
    );
}
