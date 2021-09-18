use combine::parser;
use combine::{attempt, between, eof, many, skip_many, token, value, EasyParser, Parser, Stream};

parser! {
    pub fn space[Input]()(Input) -> char
        where [
        Input: Stream<Token = char>,
        ]
    {
        token(' ')
    }
}

parser! {
    pub fn devide_space[Input, P](parser: P)(Input) -> P::Output
        where [
        Input: Stream<Token = char>,
        P: Parser<Input>
        ]
    {
        skip_many(space())
            .with(parser)
            .skip(many::<Vec<_>, _, _>(space()))
    }
}
#[test]
fn ts_devide_space() {
    let mut p = devide_space(many::<Vec<char>, _, _>(parser::char::digit()));
    assert_eq!(p.parse(" 123 ").map(|d| d.0), Ok(vec!['1', '2', '3']));
}

parser! {
    pub fn newline[Input]()(Input) -> ()
        where [
        Input: Stream<Token = char>
        ]
    {
        parser::char::newline().map(|_| ())
            .or(parser::char::crlf().map(|_| ()))
    }
}
#[test]
fn ts_to_newline() {
    let result = newline().easy_parse("\n");
    assert_eq!(result, Ok(((), "")));
}

parser! {
    pub fn to[Input, P, E](parser: P, until: E)(Input) -> P::Output
        where [
        Input: Stream<Token = char>,
        P: Parser<Input>,
        E: Parser<Input>
        ]
    {
        between(
            value(()),
            until,
            parser
        )
    }
}
#[test]
fn ts_to() {
    let result = to(
        many::<Vec<_>, _, _>(parser::char::digit()),
        parser::char::string("end"),
    )
    .easy_parse("123end");
    assert_eq!(result, Ok((vec!['1', '2', '3'], "")));
}

parser! {
    pub fn end[Input]()(Input) -> ()
        where [
        Input: Stream<Token = char>
        ]
    {
        newline().or(eof())
    }
}
