use super::super::token::Token;
use s::{Or, Result, ScannerError, S};

/// not number and reserved_token
fn text(s: &S<u8>) -> Result<(S<u8>, &S<u8>)> {
    let mut c = vec![];
    c.append(&mut Token::raw_numbers());
    c.append(&mut Token::raw_reserved_chars());
    let target = Or::from(c).not().leave();
    let (v, s) = s.until_or_end(&target)?;

    if v.length() <= 0 {
        Err(ScannerError::not_found(&target))
    } else {
        Ok((v, s))
    }
}

pub fn text_token(s: &S<u8>) -> Result<(Token, &S<u8>)> {
    text(s).map(|(v, s)| {
        (
            Token::text(
                v.to_vector()
                    .into_iter()
                    .map(|&s| s as char)
                    .collect::<String>(),
            ),
            s,
        )
    })
}

#[test]
fn text_token_first_reserved_token() {
    assert_eq!(text_token(&S::from("#123456")).is_err(), true);
}

#[test]
fn text_token_goal() {
    assert_eq!(
        text_token(&S::from("abcd#123")),
        Ok((Token::text("abcd"), &S::from("#123")))
    );
}

#[test]
fn text_token_no_goal() {
    assert_eq!(
        text_token(&S::from("abcd")),
        Ok((Token::text("abcd"), &S::Nil))
    );
}
