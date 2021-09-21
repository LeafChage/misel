use super::super::block::Block;
use crate::parser::error::parser::Result;
use crate::parser::s::S;
use crate::span;
use crate::tokenize::Token;

pub fn vanilla(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let newline = tokens.until_include(Token::Newline);
    let eof = tokens.until_include(Token::EOF);
    let (src, tokens) = match (&newline, &eof) {
        (Ok(_), _) => newline,
        (Err(_), Ok(_)) => eof,
        (Err(_), Err(_)) => newline,
    }?;

    let (spans, _) = span::parse(&src)?;
    Ok((Block::Vanilla(spans), tokens))
}
