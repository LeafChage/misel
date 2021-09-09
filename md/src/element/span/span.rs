use super::emphasis::Emphasis;

#[derive(Debug, Eq)]
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

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Span::Link(ref a1, ref a2), &Span::Link(ref b1, ref b2)) => a1 == b1 && a2 == b2,
            (&Span::Emphasis(ref a), &Span::Emphasis(ref b)) => a == b,
            (&Span::Code(ref a), &Span::Code(ref b)) => a == b,
            (&Span::Image(ref a1, ref a2), &Span::Image(ref b1, ref b2)) => a1 == b1 && a2 == b2,
            (&Span::Text(ref a), &Span::Text(ref b)) => a == b,
            (&Span::Nil, &Span::Nil) => true,
            _ => false,
        }
    }
}

#[test]
fn ts_eq() {
    assert_eq!(
        Span::Link("link".to_owned(), "https://example.com".to_owned()),
        Span::Link("link".to_owned(), "https://example.com".to_owned()),
    );

    assert_eq!(
        Span::Emphasis(Emphasis::emphasis("hello")),
        Span::Emphasis(Emphasis::emphasis("hello"))
    );
    assert_ne!(
        Span::Emphasis(Emphasis::emphasis("hello")),
        Span::Emphasis(Emphasis::emphasis("world"))
    );
    assert_ne!(
        Span::Emphasis(Emphasis::emphasis("hello")),
        Span::Emphasis(Emphasis::strong("hello"))
    );

    assert_eq!(
        Span::code("console.log('hello')"),
        Span::code("console.log('hello')")
    );
    assert_eq!(
        Span::image("https://example.com/image.jpeg", "alt"),
        Span::image("https://example.com/image.jpeg", "alt")
    );
    assert_eq!(Span::text("hello world"), Span::text("hello world"));
    assert_eq!(Span::Nil, Span::Nil);
}
