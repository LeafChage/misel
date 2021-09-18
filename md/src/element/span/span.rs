use super::emphasis::Emphasis;

#[derive(Debug, Eq, PartialEq)]
pub enum Span {
    Link(String, String),
    Emphasis(Emphasis),
    Code(String),
    Image(String, String),
    Text(String),
    Nil,
}

impl Span {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Span::Link(label.into(), href.into())
    }

    pub fn code(code: impl Into<String>) -> Self {
        Span::Code(code.into())
    }

    pub fn image(url: impl Into<String>, alt: impl Into<String>) -> Self {
        Span::Image(url.into(), alt.into())
    }

    pub fn text(t: impl Into<String>) -> Self {
        Span::Text(t.into())
    }
}
