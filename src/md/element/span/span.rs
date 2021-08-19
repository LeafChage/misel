use super::emphasis::Emphasis;

#[derive(Debug, Eq)]
pub enum Span {
    Link(String, String),
    Emphasis(Emphasis),
    Code(String),
    Image(String, String),
    Text(String),
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Span::Link(ref a1, ref a2), &Span::Link(ref b1, ref b2)) => a1 == b1 && a2 == b2,
            (&Span::Emphasis(ref a), &Span::Emphasis(ref b)) => a == b,
            (&Span::Code(ref a), &Span::Code(ref b)) => a == b,
            (&Span::Image(ref a1, ref a2), &Span::Image(ref b1, ref b2)) => a1 == b1 && a2 == b2,
            (&Span::Text(ref a), &Span::Text(ref b)) => a == b,
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
        Span::Emphasis(Emphasis::Emphasis("hello".to_owned())),
        Span::Emphasis(Emphasis::Emphasis("hello".to_owned()))
    );
    assert_ne!(
        Span::Emphasis(Emphasis::Emphasis("hello".to_owned())),
        Span::Emphasis(Emphasis::Emphasis("world".to_owned()))
    );
    assert_ne!(
        Span::Emphasis(Emphasis::Emphasis("hello".to_owned())),
        Span::Emphasis(Emphasis::Strong("hello".to_owned()))
    );

    assert_eq!(
        Span::Code("console.log('hello')".to_owned()),
        Span::Code("console.log('hello')".to_owned())
    );
    assert_eq!(
        Span::Image(
            "https://example.com/image.jpeg".to_owned(),
            "alt".to_owned()
        ),
        Span::Image(
            "https://example.com/image.jpeg".to_owned(),
            "alt".to_owned()
        )
    );
    assert_eq!(
        Span::Text("hello world".to_owned(),),
        Span::Text("hello world".to_owned(),)
    );
}
