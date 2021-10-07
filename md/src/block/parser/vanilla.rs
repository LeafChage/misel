use super::super::block::Block;
use crate::span;
use crate::tokenize::Token;
use s::{Result, ScannerError, S};

pub fn vanilla(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    if let Ok(_) = tokens.next_is_ignore(Token::EOF) {
        return Err(ScannerError::unexpected(&Token::EOF, &tokens.head()));
    }

    let newline = tokens.until_include(Token::Newline);
    let eof = tokens.until_include(Token::EOF);
    let (src, tokens) = match (&newline, &eof) {
        (Ok(_), _) => newline,
        (Err(_), Ok(_)) => eof,
        (Err(_), Err(_)) => newline,
    }?;

    let (spans, _) = span::parse(&src)?;
    Ok((Block::Vanilla(spans), tokens))
}

#[test]
fn ts_vanilla1() {
    use crate::span::Span;
    assert_eq!(
        vanilla(&crate::tokenize::parse("hello or [Rust]").unwrap()),
        Ok((
            Block::Vanilla(S::from_vector(vec![
                Span::text("hello or"),
                Span::text("[Rust]"),
            ])),
            &S::Nil
        ))
    );
}
