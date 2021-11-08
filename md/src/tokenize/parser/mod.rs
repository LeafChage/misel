mod defined_token;
mod digit;
mod text;

use super::token::Token;
use defined_token::defined_token;
use digit::digit_token;
use s::{Result, S};
use text::text_token;

pub fn parse(s: &S<u8>) -> Result<S<Token>> {
    let (head, s) = defined_token(s).or(digit_token(s)).or(text_token(s))?;
    let tail = if s.head().is_some() {
        parse(s)?
    } else {
        S::cons(Token::EOF, S::Nil)
    };
    Ok(S::cons(head, tail))
}

#[test]
fn ts_parse() {
    assert_eq!(
        parse(&S::from(
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
        )),
        Ok(S::from(vec![
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
            Token::Number(0),
            Token::text(";"),
            Token::Space,
            Token::text("i"),
            Token::Space,
            Token::text("<"),
            Token::Space,
            Token::Number(10),
            Token::text(";"),
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
        ]),)
    );
}
