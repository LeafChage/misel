#[macro_use]
extern crate combine;

use combine::EasyParser;

pub mod element;
mod parser;

pub fn markdown_parser(
    src: &str,
) -> Result<
    element::S<element::Block>,
    combine::easy::Errors<char, &str, combine::stream::PointerOffset<str>>,
> {
    let (tokens, _) = parser::block::parse().easy_parse(src)?;
    Ok(tokens)
}
