use super::token::Token;
use combine::parser::char::digit;
use combine::{attempt, choice, eof, look_ahead, many, many1, satisfy, token, Stream};
use s::S;
use std::vec::*;

fn is_reserved_token(c: char) -> bool {
    match c {
        '#' | '>' | '`' | '_' | '*' | '-' | '+' | '.' | '[' | ']' | '!' | '(' | ')' | '|' | ':'
        | ' ' | '\t' | '\n' => true,
        _ => false,
    }
}

parser! {
    pub fn text[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        many::<Vec<_>, _, _>(satisfy(|c| !is_reserved_token(c)))
            .map(|chars| Token::text(chars.iter().collect::<String>()))
    }
}

parser! {
    pub fn index[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        many1(digit())
            .and(look_ahead(token('.')))
            .map(|(nums, _dot): (Vec<char>, char)| {
                let num = nums.iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                Token::Index(num)
            })
    }
}

parser! {
    pub fn other[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        attempt(index()).or(text())
    }
}

parser! {
    pub fn parse[Input]()(Input) -> Vec<Token>
        where [
        Input: Stream<Token = char>,
        ]
    {
        choice((
                token('#').map(|_| Token::Sharp),
                token('>').map(|_| Token::AngleBracketEnd),
                token('`').map(|_| Token::BackQuote),
                token('_').map(|_| Token::UnderScore),
                token('*').map(|_| Token::Asterisk),
                token('-').map(|_| Token::Hyphen),
                token('+').map(|_| Token::Plus),
                token('.').map(|_| Token::Dot),
                token('[').map(|_| Token::BlockBracketStart),
                token(']').map(|_| Token::BlockBracketEnd),
                token('!').map(|_| Token::ExclamationMark),
                token('(').map(|_| Token::BracketStart),
                token(')').map(|_| Token::BracketEnd),
                token('|').map(|_| Token::Pipe),
                token(':').map(|_| Token::Colon),
                token(' ').map(|_| Token::Space),
                token('\t').map(|_| Token::Indent),
                token('\n').map(|_| Token::Newline),
                other(),
        )).map(|d| {
            println!("{:?}", d);
            d
        }).and(eof().map(|_| vec![Token::EOF].concat()S::cons(Token::EOF, S::Nil))
        .or(parse()))
        .map(|(car, cdr)| S::cons(car, cdr))
    }
}

#[test]
fn ts_parse() {
    use combine::EasyParser;
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
"##
        ),
        Ok((
            S::from_vector(vec![
                Token::Sharp,
                Token::Space,
                Token::text("Title"),
                Token::Newline,
                Token::text("I'm"),
                Token::Space,
                Token::Asterisk,
                Token::Asterisk,
                Token::text("chage"),
                Token::Asterisk,
                Token::Asterisk,
                Token::Dot,
                Token::Newline,
                Token::text("I"),
                Token::Space,
                Token::text("write"),
                Token::Space,
                Token::text("program"),
                Token::Dot,
                Token::Newline,
                Token::BackQuote,
                Token::BackQuote,
                Token::BackQuote,
                Token::text("javascript"),
                Token::Newline,
                Token::text("for"),
                Token::BracketStart,
                Token::text("int"),
                Token::Space,
                Token::text("i"),
                Token::Space,
                Token::text("="),
                Token::Space,
                Token::text("0;"),
                Token::Space,
                Token::text("i"),
                Token::Space,
                Token::text("<"),
                Token::Space,
                Token::text("10;"),
                Token::Space,
                Token::text("i"),
                Token::Plus,
                Token::Plus,
                Token::BracketEnd,
                Token::Space,
                Token::text("{"),
                Token::Newline,
                Token::Space,
                Token::Space,
                Token::Space,
                Token::Space,
                Token::text("console"),
                Token::Dot,
                Token::text("log"),
                Token::BracketStart,
                Token::text("i"),
                Token::BracketEnd,
                Token::text(";"),
                Token::Newline,
                Token::text("}"),
                Token::Newline,
                Token::BackQuote,
                Token::BackQuote,
                Token::BackQuote,
                Token::Newline,
                Token::Hyphen,
                Token::Space,
                Token::Hyphen,
                Token::Space,
                Token::Hyphen,
                Token::Newline,
                Token::EOF,
            ]),
            ""
        ))
    );
}
