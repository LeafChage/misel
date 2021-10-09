use crate::s::S;
use combine::EasyParser;
mod parser;
mod token;

pub use token::Token;

pub fn parse(
    src: &str,
) -> Result<S<Token>, combine::easy::Errors<char, &str, combine::stream::PointerOffset<str>>> {
    // let b = src.to_owned().into_bytes();
    // let _ = S::from_vector(b);
    // println!("[Tokenize]>>>");
    // let (tokens, _) = parser::parse().easy_parse(src)?;
    // println!("<<<[Tokenize]");
    // Ok(tokens)
    Ok(S::Nil)
}
