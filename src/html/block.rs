use super::html::Html;
use md::block::{List, ListKind, ListLine};
use md::{Block, Span, S};

impl Html for Block {
    fn html(&self) -> String {
        match self {
            Block::Header(level, list) => format!("<h{}>{}</h{}>", level, list.html(), level),
            Block::Backquote(_level, _list) => format!(""),
            Block::List(l) => format!("{}", l.html()),
            Block::CodeBlock(_lang, src) => format!("<div>{}</div>", Span::html(src)),
            Block::HorizontalRules => format!("</hr>"),
            Block::Vanilla(list) => format!("<p>{}</p>", list.html()),
        }
    }
}

impl Html for List {
    fn html(&self) -> String {
        match self.kind {
            ListKind::Ordered => format!("<ol>{}</ol>", self.list.html()),
            ListKind::Unordered => format!("<ul>{}</ul>", self.list.html()),
            ListKind::Nil => String::from(""),
        }
    }
}
impl Html for S<ListLine> {
    fn html(&self) -> String {
        self.fold(String::from(""), |src, span| {
            format!("{}\n<li>{}</li>", src, span.html())
        })
    }
}

impl Html for ListLine {
    fn html(&self) -> String {
        format!("{}{}", self.line.html(), self.child_list.html())
    }
}
