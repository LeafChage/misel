use super::html::Html;
use crate::md::element::{Emphasis, Span};

impl Html for Span {
    fn html(&self) -> String {
        match self {
            Span::Link(label, src) => format!("<a href=\"{}\">{}</a>", src, label),
            Span::Emphasis(Emphasis::Strong(value)) => format!("<strong>{}</strong>", value),
            Span::Emphasis(Emphasis::Emphasis(value)) => format!("<em>{}</em>", value),
            Span::Code(src) => format!("<span>{}</span>", src),
            Span::Image(src, alt) => format!("<img src=\"{}\" alt=\"{}\"></img>", src, alt),
            Span::Text(value) => format!("{}", value),
        }
    }
}

#[test]
fn ts_text_to_html() {
    assert_eq!(
        Span::Text("hello world!!!".to_owned()).html(),
        r#"hello world!!!"#.to_owned(),
    );
}
#[test]
fn ts_emphasis_strong_to_html() {
    assert_eq!(
        Span::Emphasis(Emphasis::Strong("strong".to_owned())).html(),
        r#"<strong>strong</strong>"#.to_owned(),
    );
}
#[test]
fn ts_emphasis_emphasis_to_html() {
    assert_eq!(
        Span::Emphasis(Emphasis::Emphasis("emphasis".to_owned())).html(),
        r#"<em>emphasis</em>"#.to_owned(),
    );
}

#[test]
fn ts_link_to_html() {
    assert_eq!(
        Span::Link("link".to_owned(), "https://example.com".to_owned()).html(),
        r#"<a href="https://example.com">link</a>"#.to_owned(),
    );
}

#[test]
fn ts_image_to_html() {
    assert_eq!(
        Span::Image(
            "https://example.com/image.jpeg".to_owned(),
            "alt".to_owned()
        )
        .html(),
        r#"<img src="https://example.com/image.jpeg" alt="alt"></img>"#.to_owned(),
    );
}
