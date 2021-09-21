use super::html::Html;
use md::span::emphasis::EmphasisType;
use md::Span;

impl Html for Span {
    fn html(&self) -> String {
        match self {
            Span::Link(label, src) => format!("<a href=\"{}\">{}</a>", src, label),
            Span::Emphasis(EmphasisType::Strong, value) => format!("<strong>{}</strong>", value),
            Span::Emphasis(EmphasisType::Emphasis, value) => format!("<em>{}</em>", value),
            Span::Code(src) => format!("<span>{}</span>", src),
            Span::Image(alt, src) => format!("<img src=\"{}\" alt=\"{}\"></img>", src, alt),
            Span::Text(value) => format!("{}", value),
            Span::Nil => String::new(),
        }
    }
}

#[test]
fn ts_text_to_html() {
    assert_eq!(
        Span::text("hello world!!!").html(),
        r#"hello world!!!"#.to_owned(),
    );
}
#[test]
fn ts_emphasis_strong_to_html() {
    assert_eq!(
        Span::emphasis(EmphasisType::Strong, "strong").html(),
        r#"<strong>strong</strong>"#.to_owned(),
    );
}
#[test]
fn ts_emphasis_emphasis_to_html() {
    assert_eq!(
        Span::emphasis(EmphasisType::Emphasis, "emphasis").html(),
        r#"<em>emphasis</em>"#.to_owned(),
    );
}

#[test]
fn ts_link_to_html() {
    assert_eq!(
        Span::link("link", "https://example.com").html(),
        r#"<a href="https://example.com">link</a>"#.to_owned(),
    );
}

#[test]
fn ts_image_to_html() {
    assert_eq!(
        Span::image("alt", "https://example.com/image.jpeg",).html(),
        r#"<img src="https://example.com/image.jpeg" alt="alt"></img>"#.to_owned(),
    );
}
