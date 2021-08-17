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

use element::service::Html;
pub fn generate_html(markdown: element::BlockChain, title: &str, style: &str) -> String {
    let html = markdown.html();
    String::from(format!(
        "<!doctype html>
        <meta charset=\"utf-8\">
        <title>{}</title>
    <style>
    {}
    </style>
    <body>
    <main>
    {}
    </main>
    </body>",
        title, style, html,
    ))
}
