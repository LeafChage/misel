use super::emphasis::EmphasisType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Span {
    Link(String, String),
    Emphasis(EmphasisType, String),
    Code(String),
    Image(String, String),
    Text(String),
    Nil,
}

impl Span {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Span::Link(label.into(), href.into())
    }

    pub fn emphasis(t: EmphasisType, v: impl Into<String>) -> Self {
        Span::Emphasis(t, v.into())
    }

    pub fn code(code: impl Into<String>) -> Self {
        Span::Code(code.into())
    }

    pub fn image(alt: impl Into<String>, url: impl Into<String>) -> Self {
        Span::Image(alt.into(), url.into())
    }

    pub fn text(t: impl Into<String>) -> Self {
        Span::Text(t.into())
    }
}
