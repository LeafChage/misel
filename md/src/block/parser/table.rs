use super::super::block::Block;
use crate::span;
use crate::span::Span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{And, Mono, Result, S};

fn table_header(tokens: &S<Token>) -> Result<(S<Span>, &S<Token>)> {
    let (_, tokens) = tokens.next(&Mono::new(Token::Pipe).ignore())?;
    let (spans, tokens) = tokens.until(&Mono::new(Token::Pipe).leave())?;
    let text = Span::text(spans.show());

    // will finish ?
    if let Ok((_, tokens)) = tokens.next(&And::from(vec![Token::Pipe, Token::Newline]).ignore()) {
        Ok((S::unit(text), tokens))
    } else {
        let (next_spans, tokens) = table_header(tokens)?;
        Ok((S::cons(text, next_spans), tokens))
    }
}

fn table_under_header(tokens: &S<Token>) -> Result<&S<Token>> {
    let (_, tokens) = tokens.next(&Mono::new(Token::Pipe).ignore())?;
    let (_, tokens) = tokens.until(&Mono::new(Token::Pipe).leave())?;

    // will finish ?
    if let Ok((_, tokens)) = tokens.next(&And::from(vec![Token::Pipe, Token::Newline]).ignore()) {
        Ok(tokens)
    } else {
        let tokens = table_under_header(tokens)?;
        Ok(tokens)
    }
}

fn table_body(tokens: &S<Token>) -> Result<(S<S<Span>>, &S<Token>)> {
    let (_, tokens) = tokens.next(&Mono::new(Token::Pipe).ignore())?;
    let (column, tokens) = tokens.until(&Mono::new(Token::Pipe).leave())?;
    let (spans, _) = span::parse(&column.push(Token::EOF))?;

    // will finish ?
    if let Ok((_, tokens)) = tokens.next(&And::from(vec![Token::Pipe, Token::Newline]).ignore()) {
        Ok((S::unit(spans), tokens))
    } else {
        let (next_spans, tokens) = table_body(tokens)?;
        Ok((S::cons(spans, next_spans), tokens))
    }
}

fn table_bodies(tokens: &S<Token>) -> Result<(S<S<S<Span>>>, &S<Token>)> {
    if let Ok((line, tokens)) = table_body(tokens) {
        let (next_line, tokens) = table_bodies(tokens)?;
        Ok((S::cons(line, next_line), tokens))
    } else {
        Ok((S::Nil, tokens))
    }
}

pub fn table(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let (headers, tokens) = table_header(tokens)?;
    let tokens = table_under_header(tokens)?;
    let (bodies, tokens) = table_bodies(tokens)?;
    Ok((Block::Table(headers, bodies), tokens))
}

#[test]
fn ts_parse_table() {
    use crate::span::Span;
    assert_eq!(
        table(
            &crate::tokenize::parse(
                r#"|header1|header2|header3|
|---|---|---|
|body11|body12|body13|
|body21|body22|body23|
"#
            )
            .unwrap()
        )
        .map(|v| v.0),
        Ok(Block::Table(
            S::from(vec![
                Span::text("header1"),
                Span::text("header2"),
                Span::text("header3"),
            ]),
            S::from(vec![
                S::from(vec![
                    S::unit(Span::text("body11")),
                    S::unit(Span::text("body12")),
                    S::unit(Span::text("body13")),
                ]),
                S::from(vec![
                    S::unit(Span::text("body21")),
                    S::unit(Span::text("body22")),
                    S::unit(Span::text("body23")),
                ])
            ])
        ))
    );
}
