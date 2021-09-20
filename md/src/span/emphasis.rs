use crate::tokenize::Token;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EmphasisType {
    Emphasis,
    Strong,
}

impl EmphasisType {
    pub fn other(&self) -> EmphasisType {
        match self {
            EmphasisType::Emphasis => EmphasisType::Strong,
            EmphasisType::Strong => EmphasisType::Emphasis,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EmphasisToken {
    UnderScore,
    Asterisk,
}

impl EmphasisToken {
    pub fn token(&self) -> Token {
        match self {
            EmphasisToken::Asterisk => Token::Asterisk,
            EmphasisToken::UnderScore => Token::UnderScore,
        }
    }
}
