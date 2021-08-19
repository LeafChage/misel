// https://daringfireball.net/projects/markdown/syntax
use super::super::span::Span;
use super::super::SpanChain;
use super::List;

#[derive(Debug, Eq)]
pub enum Block {
    Header(u32, SpanChain),
    Backquote(u32, SpanChain),
    List(List),
    CodeBlock(String, Span),
    HorizontalRules,
    Vanilla(SpanChain),
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Block::HorizontalRules, &Block::HorizontalRules) => true,
            (&Block::Header(ref a1, ref a2), &Block::Header(ref b1, ref b2)) => {
                a1 == b1 && a2 == b2
            }
            (&Block::CodeBlock(ref a1, ref a2), &Block::CodeBlock(ref b1, ref b2)) => {
                a1 == b1 && a2 == b2
            }
            (&Block::Backquote(ref a1, ref a2), &Block::Backquote(ref b1, ref b2)) => {
                a1 == b1 && a2 == b2
            }
            (&Block::Vanilla(ref a1), &Block::Vanilla(ref b1)) => a1 == b1,
            _ => false,
        }
    }
}
