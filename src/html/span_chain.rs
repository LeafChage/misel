use super::html::Html;
use crate::md::element::SpanChain;

impl Html for SpanChain {
    fn html(&self) -> String {
        if let Some(ref tail) = self.tail {
            format!("{}{}", self.head.html(), tail.html())
        } else {
            format!("{}", self.head.html())
        }
    }
}
