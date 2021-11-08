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
    pub fn text(str: impl Into<String>) -> Token {
        Token::Text(str.into())
    }

    pub fn raw_numbers() -> Vec<u8> {
        vec![b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']
    }

    pub fn raw_reserved_chars() -> Vec<u8> {
        vec![
            b'#', b'>', b'`', b'_', b'*', b'-', b'+', b'.', b'[', b']', b'!', b'(', b')', b'|',
            b':', b' ', b'\t', b'\n',
        ]
    }

    pub fn show(&self) -> String {
        match self {
            &Token::Sharp => "#".to_string(),
            &Token::AngleBracketEnd => ">".to_string(),
            &Token::BackQuote => "`".to_string(),
            &Token::UnderScore => "_".to_string(),
            &Token::Asterisk => "*".to_string(),
            &Token::Hyphen => "-".to_string(),
            &Token::Plus => "+".to_string(),
            &Token::Dot => ".".to_string(),
            &Token::BlockBracketStart => "[".to_string(),
            &Token::BlockBracketEnd => "]".to_string(),
            &Token::ExclamationMark => "!".to_string(),
            &Token::BracketStart => "(".to_string(),
            &Token::BracketEnd => ")".to_string(),
            &Token::Pipe => "|".to_string(),
            &Token::Colon => ":".to_string(),
            &Token::Space => " ".to_string(),
            &Token::Indent => "\t".to_string(),
            &Token::Newline => "\n".to_string(),
            &Token::Text(ref s) => s.to_string(),
            &Token::Number(n) => n.to_string(),
            &Token::EOF => panic!("unexpected"),
        }
    }
}
