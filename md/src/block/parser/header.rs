use super::super::block::Block;
use crate::span;
use crate::tokenize::Token;
use s::{Result, S};

pub fn header(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let (level, tokens) = tokens.many1_ignore(&S::from_vector(vec![Token::Sharp]))?;
    let (_, tokens) = tokens.many1_ignore(&S::from_vector(vec![Token::Space]))?;
    let newline = tokens.until_include(&Token::Newline);
    let eof = tokens.until_include(&Token::EOF);
    let (src, tokens) = match (&newline, &eof) {
        (Ok(_), _) => newline,
        (Err(_), Ok(_)) => eof,
        (Err(_), Err(_)) => newline,
    }?;

    let added_eof = src.push(Token::EOF);
    let (spans, _) = span::parse(added_eof)?;
    Ok((Block::Header(level as u32, spans), tokens))
}

#[test]
fn ts_parse_header() {
    use crate::span::Span;
    assert_eq!(
        header(
            &crate::tokenize::parse(
                r#"# hello world
hello"#
            )
            .unwrap()
        )
        .map(|v| v.0),
        Ok(Block::Header(1, S::unit(Span::text("hello world"))))
    );
    assert_eq!(
        header(&crate::tokenize::parse("## hi [https://example.com](LeafChage)").unwrap())
            .map(|v| v.0),
        Ok(Block::Header(
            2,
            S::cons(
                Span::text("hi"),
                S::unit(Span::link("https://example.com", "LeafChage"),)
            )
        ))
    );
}
