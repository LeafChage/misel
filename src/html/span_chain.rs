use super::html::Html;
use md::element::{Span, S};

impl Html for S<Span> {
    fn html(&self) -> String {
        self.fold(String::from(""), |src, span| {
            format!("{}{}", src, span.html())
        })
    }
}
