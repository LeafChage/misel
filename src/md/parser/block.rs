use super::super::element::block::{Block, List, ListToken};
use super::super::element::span::Span;
use super::super::element::{BlockChain, SpanChain};
use super::span;
use super::util::{newline, to};
use combine::parser::char::{space, string, tab};
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
            Block::Backquote(2, SpanChain::none(Span::Text("hello world".to_owned()))),
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
            .skip(optional(space()))
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
            Block::Header(1, SpanChain::none(Span::Text("hello world".to_owned()))),
            "hello"
        ))
    );
    assert_eq!(
        header().easy_parse("## hi [https://example.com](LeafChage)"),
        Ok((
            Block::Header(
                2,
                SpanChain::some(
                    Span::Text("hi ".to_owned()),
                    SpanChain::none(Span::Link(
                        "https://example.com".to_owned(),
                        "LeafChage".to_owned()
                    ),)
                )
            ),
            ""
        ))
    );
}

parser! {
    fn list[Input](t: ListToken)(Input) -> Block
        where [
            Input: Stream<Token = char>
        ]
    {
        value(()).map(|_| Block::HorizontalRules)
            // many1::<Vec<_>, _, _>(token(ListToken::char(*t)).skip(space().or(tab())))
            //     .and(to(
            //             span::spans(&oneline),
            //             take_until(newline()).or(take_until(eof()))
            //     ))
    }
}

// #[test]
// fn ts_list() {
//     assert_eq!(
//         list(ListToken::Asterisk).easy_parse(
//             r#"```* Hello1
// * Hello2
//     * Hello11
//     * Hello12
//     * Hello13
// * Hello3```"#
//         ),
//         Ok((
//             Block::List(List::List(
//                 ListToken::Asterisk,
//                 vec![
//                     List::Span(Span::Text("Hello1".to_owned())),
//                     List::Span(Span::Text("Hello2".to_owned())),
//                     List::List(
//                         ListToken::Asterisk,
//                         vec![
//                             List::Span(Span::Text("Hello11".to_owned())),
//                             List::Span(Span::Text("Hello12".to_owned())),
//                             List::Span(Span::Text("Hello13".to_owned())),
//                         ]
//                     ),
//                     List::Span(Span::Text("Hello3".to_owned())),
//                 ]
//             )),
//             ""
//         ))
//     );
// }

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
    pub fn parse[Input]()(Input) -> BlockChain
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
            .and(eof().map(|_| None)
                .or(parse().map(| v | Some(Box::new(v))))
            ).map(|(v, v2)| BlockChain::new(v, v2))
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
            BlockChain::some(
                Block::Header(1, SpanChain::none(Span::Text("Title".to_owned()))),
                BlockChain::some(
                    Block::Vanilla(SpanChain::some(
                        Span::Text("I'm ".to_owned()),
                        SpanChain::some(
                            Span::Emphasis(Emphasis::Emphasis("chage".to_owned())),
                            SpanChain::none(Span::Text(".".to_owned()))
                        )
                    )),
                    BlockChain::some(
                        Block::Vanilla(SpanChain::none(Span::Text("I write program.".to_owned()))),
                        BlockChain::some(
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
                            BlockChain::some(
                                Block::HorizontalRules,
                                BlockChain::some(
                                    Block::Vanilla(SpanChain::none(Span::Text(
                                        "I'm chage.".to_owned()
                                    ))),
                                    BlockChain::some(
                                        Block::Vanilla(SpanChain::none(Span::Text(
                                            "I write program.".to_owned()
                                        ))),
                                        BlockChain::none(Block::CodeBlock(
                                            "javascript".to_owned(),
                                            Span::Text(
                                                r"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"
                                                .to_owned()
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
