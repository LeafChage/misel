#[derive(Debug, Eq, PartialEq)]
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
