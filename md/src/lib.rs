#[macro_use]
extern crate combine;
extern crate s;

pub mod block;
pub mod error;
pub mod span;
pub mod tokenize;

pub use block::Block;
pub use error::parser::{ParseError, Result};
use s::S;
pub use span::Span;

pub fn parser(src: &str) -> Result<S<Block>> {
    if let Ok(tokens) = tokenize::parse(src) {
        let (blocks, _) = block::parse(&tokens)?;
        Ok(blocks)
    } else {
        Err(ParseError::message("unexpected"))
    }
}

#[test]
fn ts_parser() {
    use crate::block::{List, ListKind, ListLine};
    use crate::span::emphasis::EmphasisType;
    use crate::span::Span;
    assert_eq!(
        parser(
            r##"# Portfolio
---

## Name
**LeafChage**

## My favorite code
```js
console.log("hello world");
```
Do you like it?.

##  Backquote
> hello world

## Like Food
+ Fruit
  1. Apple
  2. Orange
  3. indent count have to 2space (sorry)
+ Meat

## Image
![image](https://avatars.githubusercontent.com/u/18657444?s=48&v=4)

## Link
[misel](https://github.com/LeafChage/misel)

## Table
|header1|header2|header3|
|---|---|---|
|body11|**body12**|body13|
|body21|body22|body23|
"##
        )
        .unwrap(),
        S::from_vector(vec![
            Block::Header(1, S::from_vector(vec![Span::text("Portfolio")])),
            Block::HorizontalRules,
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("Name")])),
            Block::Vanilla(S::unit(Span::emphasis(EmphasisType::Emphasis, "LeafChage"))),
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("My favorite code")])),
            Block::CodeBlock(
                String::from("js"),
                Span::text("console.log(\"hello world\");\n")
            ),
            Block::Vanilla(S::unit(Span::text("Do you like it?."))),
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("Backquote")])),
            Block::Backquote(1, S::unit(Span::text("hello world"))),
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("Like Food")])),
            Block::List(List::new(
                ListKind::Unordered,
                S::from_vector(vec![
                    ListLine::new(
                        S::unit(Span::text("Fruit")),
                        List::new(
                            ListKind::Ordered,
                            S::from_vector(vec![
                                ListLine::new(S::unit(Span::text("Apple")), List::nil()),
                                ListLine::new(S::unit(Span::text("Orange")), List::nil()),
                                ListLine::new(
                                    S::unit(Span::text("indent count have to 2space (sorry)")),
                                    List::nil()
                                ),
                            ])
                        )
                    ),
                    ListLine::new(S::unit(Span::text("Meat")), List::nil())
                ])
            )),
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("Image")])),
            Block::Vanilla(S::from_vector(vec![Span::image(
                "image",
                "https://avatars.githubusercontent.com/u/18657444?s=48&v=4"
            )])),
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("Link")])),
            Block::Vanilla(S::from_vector(vec![Span::link(
                "misel",
                "https://github.com/LeafChage/misel"
            )])),
            Block::Vanilla(S::Nil),
            Block::Header(2, S::from_vector(vec![Span::text("Table")])),
            Block::Table(
                S::from_vector(vec![
                    Span::text("header1"),
                    Span::text("header2"),
                    Span::text("header3"),
                ]),
                S::from_vector(vec![
                    S::from_vector(vec![
                        S::unit(Span::text("body11")),
                        S::unit(Span::emphasis(EmphasisType::Emphasis, "body12")),
                        S::unit(Span::text("body13")),
                    ]),
                    S::from_vector(vec![
                        S::unit(Span::text("body21")),
                        S::unit(Span::text("body22")),
                        S::unit(Span::text("body23")),
                    ],)
                ])
            ),
        ])
    );
}
