use super::super::element::block::{List, ListToken, ListUnit};
use super::super::element::{Block, Span, S};
use super::span;
use super::util::{end, newline, space, to};
use combine::parser::char::{string, tab};
use combine::parser::repeat::{repeat_skip_until, take_until};
use combine::{
    attempt, between, choice, eof, look_ahead, many, many1, optional, produce, satisfy, sep_by,
    sep_by1, sep_end_by, sep_end_by1, token, value, EasyParser, Parser, Stream,
};

parser! {
    fn code_block[Input]()(Input) -> Block
        where [
            Input: Stream<Token = char>
        ]
    {
        string("```")
            .with(
                take_until(newline())
                .skip(newline())
                .and(take_until(string("```")))
                .skip(string("```"))
                .skip(optional(newline()))
                .map(|(lang, code)| Block::CodeBlock(lang, Span::Text(code))))
    }
}
#[test]
fn ts_parse_code_block() {
    assert_eq!(
        code_block().easy_parse(
            r#"```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```"#
        ),
        Ok((
            Block::CodeBlock(
                "javascript".to_owned(),
                Span::Text(
                    r#"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"#
                    .to_owned()
                )
            ),
            ""
        ))
    );
}

parser! {
    fn backquotes[Input]()(Input) -> Block
        where [
            Input: Stream<Token = char>
        ]
    {
        many1::<Vec<_>, _, _>(token('>').skip(space()))
            .map(|backquotes| backquotes.len() as u32)
            .and(to(
                    span::parse() ,
                    newline().or(eof())
            ))
            .map(|(depth, spans)| Block::Backquote(depth, spans))
    }
}

#[test]
fn ts_backquote() {
    assert_eq!(
        backquotes().easy_parse("> > hello world"),
        Ok((
            Block::Backquote(
                2,
                S::cons(Span::text("hello"), S::unit(Span::text("world")))
            ),
            ""
        ))
    );
}

parser! {
    fn horizontal_rules[Input]()(Input) -> Block
        where [
            Input: Stream<Token = char>,
        ]
    {
        attempt(string("- - -").skip(many::<Vec<_>, _, _>(string(" -"))))
            .or(attempt(string("* * *").skip(many::<Vec<_>, _, _>(string(" *")))))
            .or(attempt(string("_ _ _").skip(many::<Vec<_>, _, _>(string(" _")))))
            .or(string("---").skip(many::<Vec<_>, _, _>(token('-'))))
            .or(string("***").skip(many::<Vec<_>, _, _>(token('*'))))
            .or(string("___").skip(many::<Vec<_>, _, _>(token('_'))))
            .map(|_| Block::HorizontalRules)
            .skip(newline())
    }
}
#[test]
fn ts_horizontal_rules1() {
    assert_eq!(
        horizontal_rules().easy_parse("- - -\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_eq!(
        horizontal_rules().easy_parse("* * *\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_eq!(
        horizontal_rules().easy_parse("_ _ _\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
}
#[test]
fn ts_horizontal_rules2() {
    assert_ne!(
        horizontal_rules().easy_parse("- -  -\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_ne!(
        horizontal_rules().easy_parse("* *  *\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_ne!(
        horizontal_rules().easy_parse("_ _  _\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
}
#[test]
fn ts_horizontal_rules3() {
    assert_eq!(
        horizontal_rules().easy_parse("---\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_eq!(
        horizontal_rules().easy_parse("***\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_eq!(
        horizontal_rules().easy_parse("___\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
}
#[test]
fn ts_horizontal_rules4() {
    assert_eq!(
        horizontal_rules().easy_parse("--------------------\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_eq!(
        horizontal_rules().easy_parse("********************\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
    assert_eq!(
        horizontal_rules().easy_parse("____________________\nhello"),
        Ok((Block::HorizontalRules, "hello"))
    );
}

parser! {
    fn header[Input]()(Input) -> Block
        where [
            Input: Stream<Token = char>
        ]
    {
        many1::<Vec<char>, _, _>(token('#'))
            .map(|sharps| sharps.len() as u32)
            .skip(many::<Vec<_>, _, _>(space()))
            .and(span::parse())
            .map(|(level, spans)| Block::Header(level, spans))
    }
}

#[test]
fn ts_parse_header() {
    assert_eq!(
        header().easy_parse(
            r#"# hello world
hello"#
        ),
        Ok((
            Block::Header(
                1,
                S::cons(Span::text("hello"), S::unit(Span::text("world")))
            ),
            "hello"
        ))
    );
    assert_eq!(
        header().easy_parse("## hi [https://example.com](LeafChage)"),
        Ok((
            Block::Header(
                2,
                S::cons(
                    Span::Text("hi".to_owned()),
                    S::unit(Span::link("https://example.com", "LeafChage"),)
                )
            ),
            ""
        ))
    );
}

parser! {
    fn vanilla[Input]()(Input) -> Block
        where [
            Input: Stream<Token = char>
        ]
    {
        span::parse().map(|spans| Block::Vanilla(spans))
    }
}

parser! {
    fn token_parse[Input]()(Input) -> Block
        where [
            Input: Stream<Token = char>,
        ]
    {
        choice((
                header(),
                code_block(),
                backquotes(),
                horizontal_rules(),
                vanilla(),
        ))
    }
}
parser! {
    pub fn parse[Input]()(Input) -> S::<Block>
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
    use super::super::element::Emphasis;
    assert_eq!(
        parse().easy_parse(
            r##"# Title
I'm **chage**.
I write program.
```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```
- - -
I'm chage.
I write program.
```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```
"##
        ),
        Ok((
            S::cons(
                Block::Header(1, S::unit(Span::text("Title"))),
                S::cons(
                    Block::Vanilla(S::cons(
                        Span::text("I'm"),
                        S::cons(
                            Span::Emphasis(Emphasis::emphasis("chage")),
                            S::unit(Span::text("."))
                        )
                    )),
                    S::cons(
                        Block::Vanilla(S::cons(
                            Span::text("I"),
                            S::cons(Span::text("write"), S::unit(Span::text("program."))),
                        )),
                        S::cons(
                            Block::CodeBlock(
                                "javascript".to_owned(),
                                Span::text(
                                    r#"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"#
                                )
                            ),
                            S::cons(
                                Block::HorizontalRules,
                                S::cons(
                                    Block::Vanilla(S::cons(
                                        Span::text("I'm"),
                                        S::unit(Span::text("chage."))
                                    )),
                                    S::cons(
                                        Block::Vanilla(S::cons(
                                            Span::text("I"),
                                            S::cons(
                                                Span::text("write"),
                                                S::unit(Span::text("program."))
                                            ),
                                        )),
                                        S::unit(Block::CodeBlock(
                                            "javascript".to_owned(),
                                            Span::text(
                                                r"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"
                                            )
                                        ))
                                    )
                                )
                            )
                        )
                    )
                )
            ),
            ""
        ))
    );
}
