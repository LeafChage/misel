mod backquote;
mod code_block;
mod header;
mod horizontal_rules;
mod list;
mod table;
mod vanilla;

pub use backquote::backquote;
pub use code_block::code_block;
pub use header::header;
pub use horizontal_rules::horizontal_rules;
pub use list::list;
pub use table::table;
pub use vanilla::vanilla;

use super::block::Block;
use crate::tokenize::Token;
use s::{Result, ScannerError, S};

pub fn parse(tokens: &S<Token>) -> Result<(S<Block>, &S<Token>)> {
    println!("[Block] >>>");
    println!("[Block] >>> NextToken {:?}", tokens.head());
    if let Ok((_, tokens)) = tokens.next_is_ignore(&Token::EOF) {
        Ok((S::Nil, tokens))
    } else if let Ok((block, tokens)) = vanilla(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else if let Ok((block, tokens)) = header(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else if let Ok((block, tokens)) = code_block(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else if let Ok((block, tokens)) = backquote(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else if let Ok((block, tokens)) = horizontal_rules(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else if let Ok((block, tokens)) = list(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else if let Ok((block, tokens)) = table(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else {
        Err(ScannerError::end())
    }
}

#[test]
fn ts_parse() {
    use crate::block::{List, ListKind, ListLine};
    use crate::span::emphasis::EmphasisType;
    use crate::span::Span;
    assert_eq!(
        parse(
            &crate::tokenize::parse(
                r##"# Title
I'm **chage**.
I write program.
```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```
- - -
I'm chage.
I write program.
```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```
***********
* List1
* List2
  + List21
  + List22
* List3
  1. List31
  2. List32
  3. List33
"##
            )
            .unwrap(),
        )
        .map(|v| v.0),
        Ok(S::from_vector(vec![
            Block::Header(1, S::unit(Span::text("Title"))),
            Block::Vanilla(S::from_vector(vec![
                Span::text("I'm"),
                Span::emphasis(EmphasisType::Emphasis, "chage"),
                Span::text(".")
            ])),
            Block::Vanilla(S::unit(Span::text("I write program."),)),
            Block::CodeBlock(
                "javascript".to_owned(),
                Span::text(
                    r#"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"#
                )
            ),
            Block::HorizontalRules,
            Block::Vanilla(S::unit(Span::text("I'm chage."))),
            Block::Vanilla(S::unit(Span::text("I write program."),)),
            Block::CodeBlock(
                "javascript".to_owned(),
                Span::text(
                    r"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"
                )
            ),
            Block::HorizontalRules,
            Block::List(List::new(
                ListKind::Unordered,
                S::from_vector(vec![
                    ListLine::new(S::from_vector(vec![Span::text("List1")]), List::nil()),
                    ListLine::new(
                        S::from_vector(vec![Span::text("List2")]),
                        List::new(
                            ListKind::Unordered,
                            S::from_vector(vec![
                                ListLine::new(
                                    S::from_vector(vec![Span::text("List21")]),
                                    List::nil()
                                ),
                                ListLine::new(
                                    S::from_vector(vec![Span::text("List22")]),
                                    List::nil()
                                ),
                            ])
                        )
                    ),
                    ListLine::new(
                        S::from_vector(vec![Span::text("List3")]),
                        List::new(
                            ListKind::Ordered,
                            S::from_vector(vec![
                                ListLine::new(
                                    S::from_vector(vec![Span::text("List31")]),
                                    List::nil()
                                ),
                                ListLine::new(
                                    S::from_vector(vec![Span::text("List32")]),
                                    List::nil()
                                ),
                                ListLine::new(
                                    S::from_vector(vec![Span::text("List33")]),
                                    List::nil()
                                ),
                            ])
                        )
                    ),
                ])
            )),
        ]))
    );
}
