use super::super::block::Block;
use crate::span;
use crate::tokenize::Token;
use s::{Mono, Result, ScannerError, S};

pub fn vanilla(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let target = Mono::new(Token::EOF).ignore();
    if let Ok(_) = tokens.next(&target) {
        return Err(ScannerError::not_found(&target));
    }

    let newline = tokens.until(&Mono::new(Token::Newline).include());
    let eof = tokens.until(&Mono::new(Token::EOF).include());
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
            Block::Vanilla(S::from(vec![Span::text("hello or"), Span::text("[Rust]"),])),
            &S::Nil
        ))
    );
}
