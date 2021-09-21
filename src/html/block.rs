use super::html::Html;
use md::block::ListLine;
use md::{Block, Span, S};

impl Html for Block {
    fn html(&self) -> String {
        match self {
            Block::Header(level, list) => format!("<h{}>{}</h{}>", level, list.html(), level),
            Block::Backquote(level, list) => format!(""),
            Block::List(l) => format!("<ul>{}</ul>", l.html()),
            Block::CodeBlock(_lang, src) => format!("<div>{}</div>", Span::html(src)),
            Block::HorizontalRules => format!("</hr>"),
            Block::Vanilla(list) => format!("<p>{}</p>", list.html()),
        }
    }
}

impl Html for S<ListLine> {
    fn html(&self) -> String {
        self.fold(String::from(""), |src, span| {
            format!("{}{}", src, span.html())
        })
    }
}

impl Html for ListLine {
    fn html(&self) -> String {
        match self {
            ListLine::Ordered(_, spans, list) => {
                format!("<ol>{}</ol>\n{}", spans.html(), list.html())
            }
            ListLine::Unordered(spans, list) => {
                format!("<li>{}</li>\n{}", spans.html(), list.html())
            }
        }
    }
}
