use super::super::span::Span;
use super::super::Chain;

#[derive(Debug, Eq)]
pub enum ListToken {
    Asterisk,  // *
    Hyphen,    // -
    Plus,      // +
    Numbering, // 1
}

impl ListToken {
    pub fn char(token: ListToken) -> char {
        match token {
            ListToken::Asterisk => '*',
            ListToken::Hyphen => '-',
            ListToken::Plus => '+',
            ListToken::Numbering => '1',
        }
    }
}

impl PartialEq for ListToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&ListToken::Asterisk, &ListToken::Asterisk)
            | (&ListToken::Hyphen, &ListToken::Hyphen)
            | (&ListToken::Plus, &ListToken::Plus)
            | (&ListToken::Numbering, &ListToken::Numbering) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq)]
pub enum List {
    List(ListToken, Vec<List>),
    Span(Span),
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&List::List(ref a, ref a2), &List::List(ref b, ref b2)) => a == b && a2 == b2,
            (&List::Span(ref a), &List::Span(ref b)) => a == b,
            _ => false,
        }
    }
}

// pub enum Tree {
//     Node(Vec<Tree>),
//     Leaf(Span),
// }
//
// pub fn b() -> Tree {
//     Tree::Node(vec![
//         Tree::Leaf(Span::Text("Hello1".to_owned())),
//         Tree::Node(vec![
//             Tree::Leaf(Span::Text("Hello2".to_owned())),
//             Tree::Node(vec![
//                 Tree::Leaf(Span::Text("Hello11".to_owned())),
//                 Tree::Leaf(Span::Text("Hello12".to_owned())),
//                 Tree::Leaf(Span::Text("Hello13".to_owned())),
//             ]),
//         ]),
//         Tree::Leaf(Span::Text("Hello3".to_owned())),
//     ])
// }
