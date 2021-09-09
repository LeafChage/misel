#[derive(Debug, Eq)]
pub enum Emphasis {
    Emphasis(String),
    Strong(String),
}

impl Emphasis {
    pub fn emphasis(src: impl Into<String>) -> Self {
        Emphasis::Emphasis(src.into())
    }

    pub fn strong(src: impl Into<String>) -> Self {
        Emphasis::Strong(src.into())
    }
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
    assert_eq!(Emphasis::emphasis("hi"), Emphasis::emphasis("hi"));
    assert_eq!(Emphasis::strong("hi"), Emphasis::strong("hi"));
    assert_ne!(Emphasis::emphasis("hi"), Emphasis::strong("hi"));
}

#[derive(Debug, Copy, Clone)]
pub enum EmphasisToken {
    UnderScore,
    Asterisk,
}

impl EmphasisToken {
    pub fn char(token: EmphasisToken) -> char {
        match token {
            EmphasisToken::Asterisk => '*',
            EmphasisToken::UnderScore => '_',
        }
    }
}
