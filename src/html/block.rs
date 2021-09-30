use super::html::Html;
use md::block::{List, ListKind, ListLine};
use md::{Block, Span, S};

impl Html for Block {
    fn html(&self) -> String {
        match self {
            Block::Header(level, list) => format!("<h{}>{}</h{}>", level, list.html(), level),
            Block::Backquote(_level, list) => format!("<backquote>{}</backquote>", list.html()),
            Block::List(l) => format!("{}", l.html()),
            Block::CodeBlock(lang, src) => format!("<div class=\"{}\">{}</div>", lang, src.html()),
            Block::HorizontalRules => format!("</hr>"),
            Block::Vanilla(list) => format!("<p>{}</p>", list.html()),
            Block::Table(headers, bodies) => format!(
                "<table>\n<tr>{}</tr>\n{}\n</table>",
                headers.fold(String::from(""), |src, column| {
                    format!("{}\n<th>{}</th>", src, column.html())
                }),
                bodies.fold(String::from(""), |src, body| {
                    format!(
                        "{}\n<tr>{}</tr>",
                        src,
                        body.fold(String::from(""), |src, column| {
                            format!("{}\n<td>{}</td>", src, column.html())
                        }),
                    )
                })
            ),
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
