use super::super::{Chain, Html};
use super::span::Span;

pub type SpanChain = Chain<Span>;

impl Html for SpanChain {
    fn html(&self) -> String {
        if let Some(ref tail) = self.tail {
            format!("{}{}", self.head.html(), tail.html())
        } else {
            format!("{}", self.head.html())
        }
    }
}
