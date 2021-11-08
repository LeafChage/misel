use crate::s::{Mono, Result, ScannerError, S};
use crate::tokenize::Token;

fn generate_token_parser(
    t: u8,
    token: Token,
) -> Box<dyn FnOnce(&S<u8>) -> Result<(Token, &S<u8>)>> {
    Box::new(move |s: &S<u8>| {
        s.next(&Mono::new(t).include())
            .map(|(_, s)| (token.clone(), s))
    })
}

pub fn defined_token(s: &S<u8>) -> Result<(Token, &S<u8>)> {
    generate_token_parser(b'#', Token::Sharp)(s)
        .or(generate_token_parser(b'>', Token::AngleBracketEnd)(s))
        .or(generate_token_parser(b'`', Token::BackQuote)(s))
        .or(generate_token_parser(b'_', Token::UnderScore)(s))
        .or(generate_token_parser(b'*', Token::Asterisk)(s))
        .or(generate_token_parser(b'-', Token::Hyphen)(s))
        .or(generate_token_parser(b'+', Token::Plus)(s))
        .or(generate_token_parser(b'.', Token::Dot)(s))
        .or(generate_token_parser(b'[', Token::BlockBracketStart)(s))
        .or(generate_token_parser(b']', Token::BlockBracketEnd)(s))
        .or(generate_token_parser(b'!', Token::ExclamationMark)(s))
        .or(generate_token_parser(b'(', Token::BracketStart)(s))
        .or(generate_token_parser(b')', Token::BracketEnd)(s))
        .or(generate_token_parser(b'|', Token::Pipe)(s))
        .or(generate_token_parser(b':', Token::Colon)(s))
        .or(generate_token_parser(b' ', Token::Space)(s))
        .or(generate_token_parser(b'\t', Token::Indent)(s))
        .or(generate_token_parser(b'\n', Token::Newline)(s))
        .or(Err(ScannerError::message("")))
}

#[test]
fn ts_reserved_tokens() {
    assert_eq!(
        defined_token(&S::from(">ab")),
        Ok((Token::AngleBracketEnd, &S::from(vec![b'a', b'b'])))
    )
}
