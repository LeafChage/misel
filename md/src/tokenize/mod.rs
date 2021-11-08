use crate::s::{Result, S};
mod parser;
mod token;

pub use token::Token;

pub fn parse(src: &str) -> Result<S<Token>> {
    println!("[Tokenize]>>>");
    let tokens = parser::parse(&S::from(src))?;
    println!("<<<[Tokenize]");
    Ok(tokens)
}
