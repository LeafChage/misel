use super::super::{Block, List, ListKind, ListLine};
use crate::span;
use crate::tokenize::Token;
use s::{And, Mono, Or, OrAnd, Result, ScannerError, S};

fn list_with(tokens: &S<Token>, depth: usize, target: Token) -> Result<(S<ListLine>, &S<Token>)> {
    let mut tails = tokens;
    for _ in 0..depth {
        if let Ok((_, t)) = tails.next(
            &OrAnd::from(vec![
                And::from(vec![Token::Space, Token::Space]),
                And::from(vec![Token::Indent]),
            ])
            .ignore(),
        ) {
            tails = t;
        } else {
            return Ok((S::Nil, tokens));
        }
    }

    let parsed_depth_tokens = tails;

    if let Ok(_) = parsed_depth_tokens.next(&Mono::new(target.clone()).leave()) {
        // parse same level list
        let (_, parsed_list_tokens) = parsed_depth_tokens
            .next(&And::from(ListLine::need_parsed_targets(&target)).ignore())?;
        let (line, parsed_list_line) =
            parsed_list_tokens.until(&Or::from(vec![Token::Newline, Token::EOF]).include())?;
        let (spans, _) = span::parse(&line.push(Token::EOF))?;

        // try to parse child list
        let (child_lines, parsed_list_child) =
            if let Ok(child_list_token) = first_token_in_same_depth(parsed_list_line, depth + 1) {
                let (lines, parsed_child_list) =
                    list_with(parsed_list_line, depth + 1, child_list_token.clone())?;
                (
                    List::new(ListKind::from_token(&child_list_token), lines),
                    parsed_child_list,
                )
            } else {
                // child is nothing
                (List::nil(), parsed_list_line)
            };

        let (next_list, parsed_next_line) = list_with(
            parsed_list_child,
            depth,
            ListLine::next_list_target(&target),
        )?;

        Ok((
            S::cons(ListLine::new(spans, child_lines), next_list),
            parsed_next_line,
        ))
    } else {
        Ok((S::Nil, tokens))
    }
}

fn first_token_in_same_depth(tokens: &S<Token>, depth: usize) -> Result<Token> {
    let mut tails = tokens;
    for _ in 0..depth {
        let (_, t) = tails.next(
            &OrAnd::from(vec![
                And::from(vec![Token::Space, Token::Space]),
                And::from(vec![Token::Indent]),
            ])
            .ignore(),
        )?;
        tails = t;
    }

    if let Ok(_) = tails.next(&Mono::new(Token::Asterisk).leave()) {
        Ok(Token::Asterisk)
    } else if let Ok(_) = tails.next(&Mono::new(Token::Plus).leave()) {
        Ok(Token::Plus)
    } else if let Ok(_) = tails.next(&Mono::new(Token::Hyphen).leave()) {
        Ok(Token::Hyphen)
    } else if let Ok(_) = tails.next(&And::from(vec![Token::Number(1), Token::Dot]).leave()) {
        Ok(Token::Number(1))
    } else {
        Err(ScannerError::not_found(&S::from(vec![
            &Token::Asterisk,
            &Token::Plus,
            &Token::Hyphen,
        ])))
    }
}

pub fn list(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let list_token = first_token_in_same_depth(tokens, 0)?;
    let (l, tokens) = list_with(tokens, 0, list_token.clone())?;
    Ok((
        Block::List(List::new(ListKind::from_token(&list_token), l)),
        tokens,
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ts_parse_list1() {
        use crate::span::Span;
        assert_eq!(
            list(
                &crate::tokenize::parse(
                    r#"* hello1
* hello2
* hello3
"#
                )
                .unwrap()
            )
            .map(|v| v.0),
            Ok(Block::List(List::new(
                ListKind::Unordered,
                S::from(vec![
                    ListLine::new(S::unit(Span::text("hello1")), List::nil()),
                    ListLine::new(S::unit(Span::text("hello2")), List::nil()),
                    ListLine::new(S::unit(Span::text("hello3")), List::nil()),
                ])
            )))
        );
    }

    #[test]
    fn ts_parse_list2() {
        use crate::span::Span;
        assert_eq!(
            list(
                &crate::tokenize::parse(
                    r#"* hello1
* hello2
  + hello21
  + hello22
  + hello23
* hello3
"#
                )
                .unwrap()
            )
            .map(|v| v.0),
            Ok(Block::List(List::new(
                ListKind::Unordered,
                S::from(vec![
                    ListLine::new(S::unit(Span::text("hello1")), List::nil()),
                    ListLine::new(
                        S::unit(Span::text("hello2")),
                        List::new(
                            ListKind::Unordered,
                            S::from(vec![
                                ListLine::new(S::unit(Span::text("hello21")), List::nil()),
                                ListLine::new(S::unit(Span::text("hello22")), List::nil()),
                                ListLine::new(S::unit(Span::text("hello23")), List::nil()),
                            ])
                        )
                    ),
                    ListLine::new(S::unit(Span::text("hello3")), List::nil()),
                ])
            )))
        );
    }

    #[test]
    fn ts_parse_list3() {
        use crate::span::Span;
        assert_eq!(
            list(
                &crate::tokenize::parse(
                    r#"* hello1
* hello2
  1. hello21
  2. hello22
  3. hello23
* hello3
"#
                )
                .unwrap()
            )
            .map(|v| v.0),
            Ok(Block::List(List::new(
                ListKind::Unordered,
                S::from(vec![
                    ListLine::new(S::unit(Span::text("hello1")), List::nil()),
                    ListLine::new(
                        S::unit(Span::text("hello2")),
                        List::new(
                            ListKind::Ordered,
                            S::from(vec![
                                ListLine::new(S::unit(Span::text("hello21")), List::nil()),
                                ListLine::new(S::unit(Span::text("hello22")), List::nil()),
                                ListLine::new(S::unit(Span::text("hello23")), List::nil()),
                            ])
                        )
                    ),
                    ListLine::new(S::unit(Span::text("hello3")), List::nil()),
                ])
            )))
        );
    }

    #[test]
    fn ts_parse_list_last_child() {
        use crate::span::Span;
        assert_eq!(
            list(
                &crate::tokenize::parse(
                    r#"* hello1
* hello2
  1. hello21
  2. hello22
  3. hello23
"#
                )
                .unwrap()
            )
            .map(|v| v.0),
            Ok(Block::List(List::new(
                ListKind::Unordered,
                S::from(vec![
                    ListLine::new(S::unit(Span::text("hello1")), List::nil()),
                    ListLine::new(
                        S::unit(Span::text("hello2")),
                        List::new(
                            ListKind::Ordered,
                            S::from(vec![
                                ListLine::new(S::unit(Span::text("hello21")), List::nil()),
                                ListLine::new(S::unit(Span::text("hello22")), List::nil()),
                                ListLine::new(S::unit(Span::text("hello23")), List::nil()),
                            ])
                        )
                    ),
                ])
            )))
        );
    }
}
