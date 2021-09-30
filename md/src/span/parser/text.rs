use super::super::span::Span;
use crate::tokenize::Token;
use s::{Result, ScannerError, S};

pub fn text(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    let reserved_tokens = vec![
        Token::BlockBracketStart,
        Token::UnderScore,
        Token::Asterisk,
        Token::BackQuote,
        Token::ExclamationMark,
    ];

    let (head, tokens) = if let Ok(_) = tokens.next_is_or_ignore(&vec![Token::EOF, Token::Newline])
    {
        // next value is end token
        Err(ScannerError::end())
    } else if let Ok((find_list, tokens)) = tokens.next_is_or_ignore(&reserved_tokens) {
        // next value is reserved token
        Ok((find_list, tokens))
    } else if let Some(h) = tokens.head() {
        Ok((h.clone(), tokens.tail()))
    } else {
        // next value is nothing
        Err(ScannerError::end())
    }?;

    let (src, tokens) = tokens.to_somewhere_leave(&reserved_tokens)?;

    let src = S::cons(head.clone(), src);
    let chmoped_text = src.chmop();
    if chmoped_text.length() == 0 {
        // only space
        Ok((Span::text(src.to_string()), tokens))
        // Ok((Span::text(src.string()), tokens))
    } else {
        Ok((Span::text(chmoped_text.string()), tokens))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ts_text1() {
        assert_eq!(
            text(&crate::tokenize::parse("hello2![]").unwrap(),).map(|v| v.0),
            Ok(Span::text("hello2")),
        );
    }
    #[test]
    fn ts_text2() {
        assert_eq!(
            text(&crate::tokenize::parse("hello").unwrap(),).map(|v| v.0),
            Ok(Span::text("hello")),
        );
    }
    #[test]
    fn ts_text3() {
        assert_eq!(
            text(&crate::tokenize::parse("hello or [Rust]").unwrap()),
            Ok((
                Span::text("hello or"),
                &S::from_vector(vec![
                    Token::BlockBracketStart,
                    Token::text("Rust"),
                    Token::BlockBracketEnd,
                    Token::EOF,
                ])
            )),
        );
    }
}
