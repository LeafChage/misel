#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Sharp,             // #
    AngleBracketEnd,   // >
    BackQuote,         // `
    UnderScore,        // _
    Asterisk,          // *
    Hyphen,            // -
    Plus,              // +
    Dot,               // .
    BlockBracketStart, //[
    BlockBracketEnd,   //]
    ExclamationMark,   // !
    BracketStart,      //(
    BracketEnd,        //)
    Pipe,              // |
    Colon,             // :
    Space,
    Indent,
    Newline,
    Text(String),
    Number(usize),
    EOF,
}

impl Token {
    pub fn values() -> Vec<Self> {
        vec![
            Token::Sharp,
            Token::AngleBracketEnd,
            Token::BackQuote,
            Token::UnderScore,
            Token::Asterisk,
            Token::Hyphen,
            Token::Plus,
            Token::Dot,
            Token::BlockBracketStart,
            Token::BlockBracketEnd,
            Token::ExclamationMark,
            Token::BracketStart,
            Token::BracketEnd,
            Token::Pipe,
            Token::Colon,
            Token::Space,
            Token::Indent,
            Token::Newline,
            Token::Text(String::default()),
            Token::Number(usize::default()),
            Token::EOF,
        ]
    }

    pub fn text(str: impl Into<String>) -> Token {
        Token::Text(str.into())
    }

    pub fn show(&self) -> String {
        match self {
            &Token::Text(ref s) => s.to_string(),
            &Token::Number(n) => n.to_string(),
            &Token::EOF => panic!("unexpected"),
            _ => self.raw().unwrap().to_string(),
        }
    }

    pub fn raw(&self) -> Option<u8> {
        match self {
            &Token::Sharp => Some(b'#'),
            &Token::AngleBracketEnd => Some(b'>'),
            &Token::BackQuote => Some(b'`'),
            &Token::UnderScore => Some(b'_'),
            &Token::Asterisk => Some(b'*'),
            &Token::Hyphen => Some(b'-'),
            &Token::Plus => Some(b'+'),
            &Token::Dot => Some(b'.'),
            &Token::BlockBracketStart => Some(b'['),
            &Token::BlockBracketEnd => Some(b']'),
            &Token::ExclamationMark => Some(b'!'),
            &Token::BracketStart => Some(b'('),
            &Token::BracketEnd => Some(b')'),
            &Token::Pipe => Some(b'|'),
            &Token::Colon => Some(b':'),
            &Token::Space => Some(b' '),
            &Token::Indent => Some(b'\t'),
            &Token::Newline => Some(b'\n'),
            &Token::Text(_) | &Token::Number(_) | &Token::EOF => None,
        }
    }

    pub fn is_reserved(c: u8) -> bool {
        for v in Self::values().iter().map(|v| v.raw()) {
            if v == Some(c) {
                return true;
            }
        }
        false
    }
}
