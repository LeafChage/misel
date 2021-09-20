use crate::parser::s::S;
use combine::EasyParser;
mod parser;
mod token;

pub use token::Token;

pub fn parse(
    src: &str,
) -> Result<S<Token>, combine::easy::Errors<char, &str, combine::stream::PointerOffset<str>>> {
    println!("[Tokenize]>>>");
    let (tokens, _) = parser::parse().easy_parse(src)?;
    println!("<<<[Tokenize]");
    Ok(tokens)
}
