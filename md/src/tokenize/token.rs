use s::S;

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
    Index(usize),
    EOF,
}

impl Token {
    pub fn text(str: impl Into<String>) -> Token {
        Token::Text(str.into())
    }

    pub fn v(&self) -> &str {
        match self {
            &Token::Sharp => "#",
            &Token::AngleBracketEnd => ">",
            &Token::BackQuote => "`",
            &Token::UnderScore => "_",
            &Token::Asterisk => "*",
            &Token::Hyphen => "-",
            &Token::Plus => "+",
            &Token::Dot => ".",
            &Token::BlockBracketStart => "[",
            &Token::BlockBracketEnd => "]",
            &Token::ExclamationMark => "!",
            &Token::BracketStart => "(",
            &Token::BracketEnd => ")",
            &Token::Pipe => "|",
            &Token::Colon => ":",
            &Token::Space => " ",
            &Token::Indent => "\t",
            &Token::Newline => "\n",
            &Token::Text(ref s) => s,
            &Token::Index(_) => panic!("unexpected"),
            &Token::EOF => panic!("unexpected"),
        }
    }
}

impl ToString for S<Token> {
    fn to_string(&self) -> String {
        if let Some(head) = self.head() {
            format!("{}{}", head.v(), self.tail().to_string())
        } else {
            String::new()
        }
    }
}

impl ToString for S<&Token> {
    fn to_string(&self) -> String {
        if let Some(head) = self.head() {
            format!("{}{}", head.v(), self.tail().to_string())
        } else {
            String::new()
        }
    }
}
