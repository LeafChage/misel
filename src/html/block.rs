use super::html::Html;
use crate::md::element::{Block, Span};

impl Html for Block {
    fn html(&self) -> String {
        match self {
            Block::Header(level, chain) => {
                format!("<h{}>{}</h{}>", level, chain.head.html(), level)
            }
            Block::Backquote(level, chain) => format!(""),
            Block::List(l) => format!(""),
            Block::CodeBlock(_lang, src) => format!("<div>{}</div>", Span::html(src)),
            Block::HorizontalRules => format!("</hr>"),
            Block::Vanilla(chain) => {
                if let Some(tail) = &chain.tail {
                    format!("<p>{}{}</p>", chain.head.html(), tail.html())
                } else {
                    format!("<p>{}</p>", chain.head.html())
                }
            }
        }
    }
}
