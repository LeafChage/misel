use super::super::block::Block;
use crate::parser::error::parser::Result;
use crate::parser::s::S;
use crate::span;
use crate::tokenize::Token;

pub fn code_block(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let (_, tokens) = tokens.next_are_ignore(S::from_vector(vec![
        Token::BackQuote,
        Token::BackQuote,
        Token::BackQuote,
    ]))?;
    let (lang, tokens) = tokens.until_ignore(Token::Newline)?;
    let (src, tokens) = tokens.until_targets_ignore(S::from_vector(vec![
        Token::BackQuote,
        Token::BackQuote,
        Token::BackQuote,
        Token::Newline,
    ]))?;
    Ok((
        Block::CodeBlock(lang.string(), span::Span::text(src.string())),
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
