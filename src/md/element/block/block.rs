// https://daringfireball.net/projects/markdown/syntax
use super::super::span::Span;
use super::super::Html;
use super::super::SpanChain;
use super::List;

#[derive(Debug, Eq)]
pub enum Block {
    Header(u32, SpanChain),
    Backquote(u32, SpanChain),
    List(List),
    CodeBlock(String, Span),
    HorizontalRules,
    Vanilla(SpanChain),
}

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

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Block::HorizontalRules, &Block::HorizontalRules) => true,
            (&Block::Header(ref a1, ref a2), &Block::Header(ref b1, ref b2)) => {
                a1 == b1 && a2 == b2
            }
            (&Block::CodeBlock(ref a1, ref a2), &Block::CodeBlock(ref b1, ref b2)) => {
                a1 == b1 && a2 == b2
            }
            (&Block::Backquote(ref a1, ref a2), &Block::Backquote(ref b1, ref b2)) => {
                a1 == b1 && a2 == b2
            }
            (&Block::Vanilla(ref a1), &Block::Vanilla(ref b1)) => a1 == b1,
            _ => false,
        }
    }
}
