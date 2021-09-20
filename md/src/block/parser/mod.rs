mod backquote;
mod code_block;
mod header;
mod horizontal_rules;
mod vanilla;

pub use backquote::backquote;
pub use code_block::code_block;
pub use header::header;
pub use horizontal_rules::horizontal_rules;
pub use vanilla::vanilla;

use super::block::Block;
use crate::parser::error::parser::{ParseError, Result};
use crate::parser::s::S;
use crate::tokenize::Token;

pub fn parse(tokens: &S<Token>) -> Result<(S<Block>, &S<Token>)> {
    println!("[Block] Next {:?} >>>", tokens.head());
    if let Ok((_, tokens)) = tokens.next_is_ignore(Token::EOF) {
        Ok((S::Nil, tokens))
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
    } else if let Ok((block, tokens)) = vanilla(tokens) {
        println!("[Block] {:?}", block);
        let (blocks, tokens) = parse(tokens)?;
        Ok((S::cons(block, blocks), tokens))
    } else {
        Err(ParseError::eof(vec![]))
    }
}

#[test]
fn ts_parse() {
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
"##
            )
            .unwrap(),
        )
        .map(|v| v.0),
        Ok(S::cons(
            Block::Header(1, S::unit(Span::text("Title"))),
            S::cons(
                Block::Vanilla(S::cons(
                    Span::text("I'm"),
                    S::cons(
                        Span::emphasis(EmphasisType::Emphasis, "chage"),
                        S::unit(Span::text("."))
                    )
                )),
                S::cons(
                    Block::Vanilla(S::unit(Span::text("I write program."),)),
                    S::cons(
                        Block::CodeBlock(
                            "javascript".to_owned(),
                            Span::text(
                                r#"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"#
                            )
                        ),
                        S::cons(
                            Block::HorizontalRules,
                            S::cons(
                                Block::Vanilla(S::unit(Span::text("I'm chage."))),
                                S::cons(
                                    Block::Vanilla(S::unit(Span::text("I write program."),)),
                                    S::unit(Block::CodeBlock(
                                        "javascript".to_owned(),
                                        Span::text(
                                            r"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"
                                        )
                                    ))
                                )
                            )
                        )
                    )
                )
            )
        ))
    );
}
