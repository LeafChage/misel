use super::super::block::Block;
use crate::span;
use crate::token_list::TokenList;
use crate::tokenize::Token;
use s::{And, Mono, Result, S};

pub fn code_block(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let (_, tokens) = tokens
        .next(&And::from(vec![Token::BackQuote, Token::BackQuote, Token::BackQuote]).ignore())?;

    let (lang, tokens) = tokens.until(&Mono::new(Token::Newline).ignore())?;
    let (src, tokens) = tokens.until(
        &And::from(vec![
            Token::BackQuote,
            Token::BackQuote,
            Token::BackQuote,
            Token::Newline,
        ])
        .ignore(),
    )?;
    Ok((
        Block::CodeBlock(lang.show(), span::Span::text(src.show())),
        tokens,
    ))
}

#[test]
fn ts_parse_code_block() {
    assert_eq!(
        code_block(
            &crate::tokenize::parse(
                r#"```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```
hi
"#
            )
            .unwrap()
        )
        .map(|v| v.0),
        Ok(Block::CodeBlock(
            "javascript".to_owned(),
            span::Span::text(
                r#"for(int i = 0; i < 10; i++) {
    console.log(i);
}
"#
            )
        ),)
    );
}
