mod code;
mod emphasis;
mod image;
mod link;
mod text;

pub use code::code;
pub use emphasis::emphasis;
pub use image::image;
pub use link::link;
pub use text::text;

use super::super::span::Span;
use crate::tokenize::Token;
use s::{Result, ScannerError, S};

pub fn parse(tokens: &S<Token>) -> Result<(S<Span>, &S<Token>)> {
    println!("\t[SPAN]>>");
    println!("\t[SPAN]>> spannext {:?}", tokens.head());
    if let Ok((_, tokens)) = tokens.next_is_ignore(&Token::EOF) {
        Ok((S::Nil, tokens))
    } else if let Ok((_, tokens)) = tokens.next_is_ignore(&Token::Newline) {
        Ok((S::Nil, tokens))
    } else if let Ok((span, tokens)) = link(tokens) {
        println!("\t[SPAN] {:?}", span);
        let (spans, tokens) = parse(tokens)?;
        Ok((S::cons(span, spans), tokens))
    } else if let Ok((span, tokens)) = code(tokens) {
        println!("\t[SPAN] {:?}", span);
        let (spans, tokens) = parse(tokens)?;
        Ok((S::cons(span, spans), tokens))
    } else if let Ok((span, tokens)) = image(tokens) {
        println!("\t[SPAN] {:?}", span);
        let (spans, tokens) = parse(tokens)?;
        Ok((S::cons(span, spans), tokens))
    } else if let Ok((span, tokens)) = emphasis(tokens) {
        println!("\t[SPAN] {:?}", span);
        let (spans, tokens) = parse(tokens)?;
        Ok((S::cons(span, spans), tokens))
    } else if let Ok((span, tokens)) = text(tokens) {
        println!("\t[SPAN] {:?}", span);
        let (spans, tokens) = parse(tokens)?;
        Ok((S::cons(span, spans), tokens))
    } else {
        println!("unexpected : {:?}", tokens.head());
        Err(ScannerError::end())
    }
}

#[test]
fn ts_parse() {
    assert_eq!(
        parse(&crate::tokenize::parse(r#"this is written [https://example.com](javascript) `console.log("hello world");` by LeafChage"#).unwrap(),).map(|v| v.0),
        Ok(S::cons(
                Span::text("this is written"),
                S::cons(
                    Span::link("https://example.com", "javascript"),
                    S::cons(
                        Span::text(" "),
                        S::cons(
                            Span::code(r#"console.log("hello world");"#),
                            S::unit(
                                Span::text(" by LeafChage"),
                            )
                        )
                    )
                )
        ))
    );
}
