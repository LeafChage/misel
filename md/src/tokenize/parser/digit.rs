use crate::s::{Or, Result, S};
use crate::tokenize::Token;

fn digit(s: &S<u8>) -> Result<(S<u8>, &S<u8>)> {
    s.many1(&Or::from(Token::raw_numbers()).include())
        .map(|(_, nums, s)| (nums, s))
}

pub fn digit_token(s: &S<u8>) -> Result<(Token, &S<u8>)> {
    digit(s).map(|(nums, s)| {
        (
            Token::Number(
                nums.to_vector()
                    .into_iter()
                    .map(|&s| s as char)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            ),
            s,
        )
    })
}

#[test]
fn ts_digit_token_finished_not_num() {
    assert_eq!(
        digit_token(&S::from("111a")),
        Ok((Token::Number(111), &S::from(vec![b'a'])))
    );
}

#[test]
fn ts_digit_token_finished_num() {
    assert_eq!(
        digit_token(&S::from("123456789")),
        Ok((Token::Number(123456789), &S::from(vec![])))
    );
}

#[test]
fn ts_digit_token_start_zero() {
    assert_eq!(
        digit_token(&S::from("012")),
        Ok((Token::Number(12), &S::from(vec![])))
    );
}
