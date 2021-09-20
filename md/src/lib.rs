#[macro_use]
extern crate combine;

pub mod block;
pub mod parser;
pub mod span;
pub mod tokenize;

pub use block::Block;
pub use parser::error::parser::{ParseError, Result};
pub use parser::s::S;
pub use span::Span;

pub fn parser(src: &str) -> Result<S<Block>> {
    if let Ok(tokens) = tokenize::parse(src) {
        let (blocks, _) = block::parse(&tokens)?;
        Ok(blocks)
    } else {
        Err(ParseError::message("unexpected"))
    }
}
