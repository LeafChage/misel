use super::html::Html;
use md::element::{Block, Span};

impl Html for Block {
    fn html(&self) -> String {
        match self {
            Block::Header(level, list) => format!("<h{}>{}</h{}>", level, list.html(), level),
            Block::Backquote(level, list) => format!(""),
            Block::List(l) => format!(""),
            Block::CodeBlock(_lang, src) => format!("<div>{}</div>", Span::html(src)),
            Block::HorizontalRules => format!("</hr>"),
            Block::Vanilla(list) => format!("<p>{}</p>", list.html()),
        }
    }
}
