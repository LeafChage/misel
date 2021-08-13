use combine;
use combine::EasyParser;

pub mod element;
mod parser;

pub fn markdown_parser(
    src: &str,
) -> Result<
    element::BlockChain,
    combine::easy::Errors<char, &str, combine::stream::PointerOffset<str>>,
> {
    let (tokens, _) = parser::block::parse().easy_parse(src)?;
    Ok(tokens)
}
