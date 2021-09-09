use super::super::element::{Emphasis, EmphasisToken, Span, S};
use super::util::{devide_space, end, space, to};
use combine::parser::char::{newline, string_cmp};
use combine::parser::repeat::take_until;
use combine::{
    attempt, between, choice, eof, look_ahead, many, many1, not_followed_by, satisfy, token,
    EasyParser, Stream,
};
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
        Ok((Span::link("example", "https://example.com"), ""))
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
        Ok((Span::code(r#"console.log("hello world")"#), ""))
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
        Ok((Span::image("https://example.com/example.jpg", "image"), ""))
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
                    Emphasis::emphasis(txt)
                } else {
                    Emphasis::strong(txt)
                }))
    }
}

#[test]
fn ts_emphasis() {
    assert_eq!(
        emphasis(EmphasisToken::Asterisk).easy_parse("**hello**"),
        Ok((Span::Emphasis(Emphasis::emphasis("hello")), ""))
    );

    assert_eq!(
        emphasis(EmphasisToken::UnderScore).easy_parse("__hello__"),
        Ok((Span::Emphasis(Emphasis::emphasis("hello")), ""))
    );

    assert_eq!(
        emphasis(EmphasisToken::Asterisk).easy_parse("*world*"),
        Ok((Span::Emphasis(Emphasis::strong("world")), ""))
    );
}

parser! {
    fn reserved[Input]()(Input) -> char
        where [
            Input: Stream<Token = char>,
        ]
    {
        token('[')
            .or(token('*'))
            .or(token('_'))
            .or(token('`'))
            .or(token('!'))
            .or(space())
            .or(newline())
            .or(eof().map(|_| '0'))
            .map(|v|{
                println!("r{:?}", v);
                v
            })
    }
}

parser! {
    fn text[Input]()(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        take_until(reserved())
            .map(|d| Span::Text(d))
    }
}
#[test]
fn ts_text() {
    assert_eq!(
        text().easy_parse("hello2![]"),
        Ok((Span::text("hello2"), "![]"))
    );
    assert_eq!(text().easy_parse("hello"), Ok((Span::text("hello"), "")));
}

parser! {
    fn token_parse[Input]()(Input) -> Span
        where [
            Input: Stream<Token = char>,
        ]
    {
        devide_space(choice((
                    link(),
                    image(),
                    code(),
                    emphasis(EmphasisToken::Asterisk),
                    emphasis(EmphasisToken::UnderScore),
                    text()
        )))
            .map(|d| {
                println!("{:?}", d);
                d
            })
    }
}
parser! {
    pub fn parse[Input]()(Input) -> S::<Span>
        where [
        Input: Stream<Token = char>,
        ]
    {
        token_parse()
            .and(end().map(|_| S::Nil)
                .or(parse()))
            .map(|(car, cdr)| S::cons(car, cdr))
    }
}

#[test]
fn ts_parse() {
    assert_eq!(
        parse().easy_parse(
            r#"this is written [https://example.com](javascript) `console.log("hello world");` by LeafChage"#
        ),
        Ok((S::cons(
                    Span::text("this"),
                    S::cons(
                        Span::text("is"),
                        S::cons(
                            Span::text("written"),
                            S::cons(
                                Span::link("https://example.com", "javascript"),
                                S::cons(
                                    Span::code(r#"console.log("hello world");"#),
                                    S::cons(
                                        Span::text("by"),
                                        S::unit(Span::text("LeafChage"))
                                    )
                                )
                            )
                        )
                    )
        ), ""))
            );
}
#[test]
fn ts_parse2() {
    assert_eq!(
        parse().easy_parse("hhhello\nhi"),
        Ok((S::unit(Span::text("hhhello")), "hi"))
    );
}
