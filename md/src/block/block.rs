// https://daringfireball.net/projects/markdown/syntax
use super::List;
use crate::parser::s::S;
use crate::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum Block {
    Header(u32, S<Span>),
    Backquote(u32, S<Span>),
    List(List),
    CodeBlock(String, Span),
    Table(S<Span>, S<S<Span>>),
    HorizontalRules,
    Vanilla(S<Span>),
}
