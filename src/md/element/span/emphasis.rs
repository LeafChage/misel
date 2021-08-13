#[derive(Debug, Eq)]
pub enum Emphasis {
    Emphasis(String),
    Strong(String),
}

impl PartialEq for Emphasis {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Emphasis::Emphasis(ref a), &Emphasis::Emphasis(ref b))
            | (&Emphasis::Strong(ref a), &Emphasis::Strong(ref b)) => a == b,
            _ => false,
        }
    }
}

#[test]
fn ts_eq() {
    assert_eq!(
        Emphasis::Emphasis("hi".to_owned()),
        Emphasis::Emphasis("hi".to_owned())
    );
    assert_eq!(
        Emphasis::Strong("hi".to_owned()),
        Emphasis::Strong("hi".to_owned())
    );
    assert_ne!(
        Emphasis::Emphasis("hi".to_owned()),
        Emphasis::Strong("hi".to_owned())
    );
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    UnderScore,
    Asterisk,
}

impl Token {
    pub fn char(token: Token) -> char {
        match token {
            Token::Asterisk => '*',
            Token::UnderScore => '_',
        }
    }
}
