use super::super::block::Block;
use crate::tokenize::Token;
use s::{Result, ScannerError, S};

fn horizontal_rules_with_target(
    tokens: &S<Token>,
    horizontal_token: Token,
    devide_space: bool,
) -> Result<(Block, &S<Token>)> {
    let (_, tokens) = tokens.next_are_ignore(&S::from_vector(if devide_space {
        vec![
            horizontal_token.clone(),
            Token::Space,
            horizontal_token.clone(),
            Token::Space,
            horizontal_token.clone(),
        ]
    } else {
        vec![
            horizontal_token.clone(),
            horizontal_token.clone(),
            horizontal_token.clone(),
        ]
    }))?;

    let (_, tokens) = tokens.many_ignore(&S::from_vector(if devide_space {
        vec![Token::Space, horizontal_token]
    } else {
        vec![horizontal_token]
    }))?;

    let (_, tokens) = tokens.next_is_ignore(Token::Newline)?;
    Ok((Block::HorizontalRules, tokens))
}

pub fn horizontal_rules(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    if let Ok(v) = horizontal_rules_with_target(tokens, Token::Hyphen, true) {
        Ok(v)
    } else if let Ok(v) = horizontal_rules_with_target(tokens, Token::Asterisk, true) {
        Ok(v)
    } else if let Ok(v) = horizontal_rules_with_target(tokens, Token::UnderScore, true) {
        Ok(v)
    } else if let Ok(v) = horizontal_rules_with_target(tokens, Token::Hyphen, false) {
        Ok(v)
    } else if let Ok(v) = horizontal_rules_with_target(tokens, Token::Asterisk, false) {
        Ok(v)
    } else if let Ok(v) = horizontal_rules_with_target(tokens, Token::UnderScore, false) {
        Ok(v)
    } else {
        Err(ScannerError::not_found(&S::from_vector(vec![
            Token::Asterisk,
            Token::UnderScore,
            Token::Hyphen,
        ])))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ts_horizontal_rules1() {
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("- - -\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("_ _ _\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
    }
    #[test]
    fn ts_horizontal_rules2() {
        assert_ne!(
            horizontal_rules(&crate::tokenize::parse("- -  -\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_ne!(
            horizontal_rules(&crate::tokenize::parse("* *  *\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_ne!(
            horizontal_rules(&crate::tokenize::parse("_ _  _\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
    }
    #[test]
    fn ts_horizontal_rules3() {
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("---\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("***\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("___\nhello").unwrap()).map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
    }
    #[test]
    fn ts_horizontal_rules4() {
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("----------------\nhello").unwrap())
                .map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("****************\nhello").unwrap())
                .map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
        assert_eq!(
            horizontal_rules(&crate::tokenize::parse("________________\nhello").unwrap())
                .map(|v| v.0),
            Ok(Block::HorizontalRules)
        );
    }
}
