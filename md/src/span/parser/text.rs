use super::super::span::Span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{Or, Result, S};

/// if first token is reserved , this is text
pub fn text(tokens: &S<Token>) -> Result<(Span, &S<Token>)> {
    if let Ok((Some(head), tokens)) = tokens.next(
        &Or::from(vec![
            Token::BlockBracketStart,
            Token::UnderScore,
            Token::Asterisk,
            Token::BackQuote,
            Token::ExclamationMark,
            Token::EOF,
            Token::Newline,
        ])
        .include(),
    ) {
        let (src, tokens) = tokens.until_or_end(
            &Or::from(vec![
                Token::BlockBracketStart,
                Token::UnderScore,
                Token::Asterisk,
                Token::BackQuote,
                Token::ExclamationMark,
                Token::EOF,
                Token::Newline,
            ])
            .leave(),
        )?;

        let c = S::cons(head, src).chmop();
        Ok((Span::text(c.show()), tokens))
    } else {
        let (src, tokens) = tokens.until_or_end(
            &Or::from(vec![
                Token::BlockBracketStart,
                Token::UnderScore,
                Token::Asterisk,
                Token::BackQuote,
                Token::ExclamationMark,
                Token::EOF,
                Token::Newline,
            ])
            .leave(),
        )?;
        let c = src.chmop();
        Ok((Span::text(c.show()), tokens))
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
                &S::from(vec![
                    Token::BlockBracketStart,
                    Token::text("Rust"),
                    Token::BlockBracketEnd,
                    Token::EOF,
                ])
            )),
        );
    }
}
