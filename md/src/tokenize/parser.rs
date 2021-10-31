use super::token::Token;
use combine::parser::char::digit;
// use combine::{attempt, choice, eof, look_ahead, many, many1, satisfy, token, Stream};
use s::{Result, S};

fn reserved_token(data: &S<u8>) -> Result<(Token, &S<u8>)> {
    let t = data
        .next_is_leave(b'#')
        .map(|_| Token::Sharp)
        .or_else(|_| data.next_is_ignore(b'>').map(|_| Token::AngleBracketEnd))
        .or_else(|_| data.next_is_ignore(b'`').map(|_| Token::BackQuote))
        .or_else(|_| data.next_is_ignore(b'_').map(|_| Token::UnderScore))
        .or_else(|_| data.next_is_ignore(b'*').map(|_| Token::Asterisk))
        .or_else(|_| data.next_is_ignore(b'-').map(|_| Token::Hyphen))
        .or_else(|_| data.next_is_ignore(b'+').map(|_| Token::Plus))
        .or_else(|_| data.next_is_ignore(b'.').map(|_| Token::Dot))
        .or_else(|_| data.next_is_ignore(b'[').map(|_| Token::BlockBracketStart))
        .or_else(|_| data.next_is_ignore(b']').map(|_| Token::BlockBracketEnd))
        .or_else(|_| data.next_is_ignore(b'!').map(|_| Token::ExclamationMark))
        .or_else(|_| data.next_is_ignore(b'(').map(|_| Token::BracketStart))
        .or_else(|_| data.next_is_ignore(b')').map(|_| Token::BracketEnd))
        .or_else(|_| data.next_is_ignore(b'|').map(|_| Token::Pipe))
        .or_else(|_| data.next_is_ignore(b':').map(|_| Token::Colon))
        .or_else(|_| data.next_is_ignore(b' ').map(|_| Token::Space))
        .or_else(|_| data.next_is_ignore(b'\t').map(|_| Token::Indent))
        .or_else(|_| data.next_is_ignore(b'\n').map(|_| Token::Newline))?;
    Ok((t, data.tail()))
}

fn reserved_tokens() -> Vec<u8> {
    Token::values()
        .into_iter()
        .map(|v| v.raw())
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect()
}

fn text(data: &S<u8>) -> Result<(Token, &S<u8>)> {
    let (output, data) = data.until_targets_ignore(&S::from_vector(reserved_tokens()))?;
    let v = String::from_utf8(output.to_vector().into_iter().map(|v| v.clone()).collect()).unwrap();
    Ok((Token::text(v), data))
}

fn number(data: &S<u8>) -> Result<(Token, &S<u8>)> {
    // pub fn number[Input]()(Input) -> Token
    //     where [
    //     Input: Stream<Token = char>,
    //     ]
    // {
    //     many1(digit()).map(|numbers: Vec<char>| Token::Number(numbers.into_iter()
    //             .collect::<String>()
    //             .parse::<usize>()
    //             .unwrap()))
    // }

    let (n, data) = data.next_is_or_ignore(&vec![
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    ])?;

    let numbers = vec![n];
    while let Ok((n, d)) = data.next_is_or_ignore(&vec![
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    ]) {
        numbers.push(n);
        data = d;
    }

    Ok((Token::Number(numbers.into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap(), data))
}

// parser! {
//     pub fn other[Input]()(Input) -> Token
//         where [
//         Input: Stream<Token = char>,
//         ]
//     {
//         attempt(number()).or(text())
//     }
// }
//
// parser! {
//     pub fn parse[Input]()(Input) -> S<Token>
//         where [
//         Input: Stream<Token = char>,
//         ]
//     {
//         choice((
//                 token('#').map(|_| Token::Sharp),
//                 token('>').map(|_| Token::AngleBracketEnd),
//                 token('`').map(|_| Token::BackQuote),
//                 token('_').map(|_| Token::UnderScore),
//                 token('*').map(|_| Token::Asterisk),
//                 token('-').map(|_| Token::Hyphen),
//                 token('+').map(|_| Token::Plus),
//                 token('.').map(|_| Token::Dot),
//                 token('[').map(|_| Token::BlockBracketStart),
//                 token(']').map(|_| Token::BlockBracketEnd),
//                 token('!').map(|_| Token::ExclamationMark),
//                 token('(').map(|_| Token::BracketStart),
//                 token(')').map(|_| Token::BracketEnd),
//                 token('|').map(|_| Token::Pipe),
//                 token(':').map(|_| Token::Colon),
//                 token(' ').map(|_| Token::Space),
//                 token('\t').map(|_| Token::Indent),
//                 token('\n').map(|_| Token::Newline),
//                 other(),
//         )).map(|d| {
//             println!("{:?}", d);
//             d
//         }).and(eof().map(|_| S::cons(Token::EOF, S::Nil))
//         .or(parse()))
//         .map(|(car, cdr)| S::cons(car, cdr))
//     }
// }
//
// #[test]
// fn ts_parse() {
//     use combine::EasyParser;
//     assert_eq!(
//         parse().easy_parse(
//             r##"# Title
// I'm **chage**.
// I write program.
// ```javascript
// for(int i = 0; i < 10; i++) {
//     console.log(i);
// }
// ```
// - - -
// "##
//         ),
//         Ok((
//             S::from_vector(vec![
//                 Token::Sharp,
//                 Token::Space,
//                 Token::text("Title"),
//                 Token::Newline,
//                 Token::text("I'm"),
//                 Token::Space,
//                 Token::Asterisk,
//                 Token::Asterisk,
//                 Token::text("chage"),
//                 Token::Asterisk,
//                 Token::Asterisk,
//                 Token::Dot,
//                 Token::Newline,
//                 Token::text("I"),
//                 Token::Space,
//                 Token::text("write"),
//                 Token::Space,
//                 Token::text("program"),
//                 Token::Dot,
//                 Token::Newline,
//                 Token::BackQuote,
//                 Token::BackQuote,
//                 Token::BackQuote,
//                 Token::text("javascript"),
//                 Token::Newline,
//                 Token::text("for"),
//                 Token::BracketStart,
//                 Token::text("int"),
//                 Token::Space,
//                 Token::text("i"),
//                 Token::Space,
//                 Token::text("="),
//                 Token::Space,
//                 Token::Number(0),
//                 Token::text(";"),
//                 Token::Space,
//                 Token::text("i"),
//                 Token::Space,
//                 Token::text("<"),
//                 Token::Space,
//                 Token::Number(10),
//                 Token::text(";"),
//                 Token::Space,
//                 Token::text("i"),
//                 Token::Plus,
//                 Token::Plus,
//                 Token::BracketEnd,
//                 Token::Space,
//                 Token::text("{"),
//                 Token::Newline,
//                 Token::Space,
//                 Token::Space,
//                 Token::Space,
//                 Token::Space,
//                 Token::text("console"),
//                 Token::Dot,
//                 Token::text("log"),
//                 Token::BracketStart,
//                 Token::text("i"),
//                 Token::BracketEnd,
//                 Token::text(";"),
//                 Token::Newline,
//                 Token::text("}"),
//                 Token::Newline,
//                 Token::BackQuote,
//                 Token::BackQuote,
//                 Token::BackQuote,
//                 Token::Newline,
//                 Token::Hyphen,
//                 Token::Space,
//                 Token::Hyphen,
//                 Token::Space,
//                 Token::Hyphen,
//                 Token::Newline,
//                 Token::EOF,
//             ]),
//             ""
//         ))
//     );
// }
