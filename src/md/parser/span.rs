use super::super::element::{Emphasis, EmphasisToken, Span, SpanChain};
use super::util::end;
use combine::parser::char::newline;
use combine::parser::repeat::take_until;
use combine::{between, choice, many, many1, satisfy, token, EasyParser, Stream};
use std::vec::*;

parser! {
    fn link[Input]()(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        between(
            token('['),
            token(']'),
            many(satisfy(|c| c != '[' && c != ']')),
        )
            .and(between(
                    token('('),
                    token(')'),
                    many(satisfy(|c| c != '(' && c != ')')),
            )).map(|d| Span::Link(d.0, d.1))
    }
}
#[test]
fn ts_link() {
    let src = "[example](https://example.com)";
    let result = link().easy_parse(src);
    assert_eq!(
        result,
        Ok((
            Span::Link("example".to_owned(), "https://example.com".to_owned()),
            ""
        ))
    );
}

parser! {
    fn code[Input]()(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        between(token('`'), token('`'), many(satisfy(|c| c != '`'))).map(|v| Span::Code(v))
    }
}
#[test]
fn ts_code() {
    assert_eq!(
        code().easy_parse(r#"`console.log("hello world")`"#),
        Ok((Span::Code(r#"console.log("hello world")"#.to_owned()), ""))
    );
}

parser! {
    fn image[Input]()(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        token('!')
            .with(
                between(
                    token('['),
                    token(']'),
                    many(satisfy(|c| c != '[' && c != ']')),
                )
                .and(between(
                        token('('),
                        token(')'),
                        many(satisfy(|c| c != '(' && c != ')')),
                )).map(|d| Span::Image(d.0, d.1))
            )
    }
}
#[test]
fn ts_image() {
    assert_eq!(
        image().easy_parse("![https://example.com/example.jpg](image)"),
        Ok((
            Span::Image(
                "https://example.com/example.jpg".to_owned(),
                "image".to_owned()
            ),
            ""
        ))
    );
}

parser! {
    fn emphasis[Input](t: EmphasisToken)(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        let v: char = EmphasisToken::char(*t);
        many1(token(v))
            .and(many(satisfy(|c| c != EmphasisToken::char(*t))))
            .and(many1(token(v)))
            .map(|((tokens, txt), _): ((Vec<char>, String), Vec<char>)|
                Span::Emphasis(if tokens.len() % 2 == 0 {
                    Emphasis::Emphasis(txt)
                } else {
                    Emphasis::Strong(txt)
                }))
    }
}

#[test]
fn ts_emphasis() {
    assert_eq!(
        emphasis(EmphasisToken::Asterisk).easy_parse("**hello**"),
        Ok((Span::Emphasis(Emphasis::Emphasis("hello".to_owned())), ""))
    );

    assert_eq!(
        emphasis(EmphasisToken::UnderScore).easy_parse("__hello__"),
        Ok((Span::Emphasis(Emphasis::Emphasis("hello".to_owned())), ""))
    );

    assert_eq!(
        emphasis(EmphasisToken::Asterisk).easy_parse("*world*"),
        Ok((Span::Emphasis(Emphasis::Strong("world".to_owned())), ""))
    );
}

parser! {
    fn text[Input]()(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        take_until(
            token('[')
            .or(token('*'))
            .or(token('_'))
            .or(token('`'))
            .or(token('!'))
            .or(newline())
        )
            .or(take_until(end()))
            .map(|d| Span::Text(d))
    }
}
#[test]
fn ts_text() {
    assert_eq!(
        text().easy_parse("hello![]"),
        Ok((Span::Text("hello".to_owned()), "![]"))
    );
    assert_eq!(
        text().easy_parse("hello"),
        Ok((Span::Text("hello".to_owned()), ""))
    );
}

parser! {
    pub fn parse[Input]()(Input) -> SpanChain
        where [
        Input: Stream<Token = char>,
        ]
    {
        choice((
                link(),
                image(),
                code(),
                emphasis(EmphasisToken::Asterisk),
                emphasis(EmphasisToken::UnderScore),
                text()
        ))
            .and(end().map(|_| None)
                .or(parse().map(| v | Some(Box::new(v)))))
            .map(|(v, v2)| SpanChain::new(v, v2))
    }
}

#[test]
fn ts_parse() {
    assert_eq!(
        parse().easy_parse(
            r#"this is written [https://example.com](javascript) `console.log("hello world");` by LeafChage"#
        ),
        Ok((
                SpanChain::some(Span::Text("this is written ".to_owned()),
                SpanChain::some(Span::Link("https://example.com".to_owned(), "javascript".to_owned()),
                SpanChain::some(Span::Text(" ".to_owned()),
                SpanChain::some(Span::Code(r#"console.log("hello world");"#.to_owned()),
                SpanChain::none(Span::Text(" by LeafChage".to_owned())))))),
                "")
        ))
}
