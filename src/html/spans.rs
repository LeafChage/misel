use super::html::Html;
use md::Span;
use s::S;

impl Html for S<Span> {
    fn html(&self) -> String {
        self.fold(String::from(""), |src, span| {
            format!("{}{}", src, span.html())
        })
    }
}
