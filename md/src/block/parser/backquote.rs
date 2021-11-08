use super::super::block::Block;
use crate::span;
use crate::tokenize::Token;
use s::{And, Mono, Result, S};

pub fn backquote(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let (_, tokens) =
        tokens.next(&And::from(vec![Token::AngleBracketEnd, Token::Space]).ignore())?;

    let (depth, _, tokens) =
        tokens.many(&And::from(vec![Token::AngleBracketEnd, Token::Space]).ignore())?;

    let newline = tokens.until(&Mono::new(Token::Newline).include());
    let eof = tokens.until(&Mono::new(Token::EOF).include());
    let (src, tokens) = match (&newline, &eof) {
        (Ok(_), _) => newline,
        (Err(_), Ok(_)) => eof,
        (Err(_), Err(_)) => newline,
    }?;
    let (spans, _) = span::parse(&src.push(Token::EOF))?;

    Ok((Block::Backquote((depth + 1) as u32, spans), tokens))
}

#[test]
fn ts_backquote() {
    use crate::span::Span;
    assert_eq!(
        backquote(&crate::tokenize::parse(r#"> > hello world"#).unwrap()).map(|v| v.0),
        Ok(Block::Backquote(2, S::unit(Span::text("hello world"))))
    );
}
