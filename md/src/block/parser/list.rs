use super::super::{Block, ListLine};
use crate::parser::error::parser::{ParseError, Result};
use crate::parser::s::S;
use crate::span;
use crate::tokenize::Token;

fn list_with(tokens: &S<Token>, depth: usize, target: Token) -> Result<(S<ListLine>, &S<Token>)> {
    let mut tails = tokens;
    for _ in 0..depth {
        if let Ok((_, t)) = tails.next_are_or_ignore(vec![
            S::from_vector(vec![Token::Space, Token::Space]),
            S::from_vector(vec![Token::Indent]),
        ]) {
            tails = t;
        } else {
            return Ok((S::Nil, tokens));
        }
    }

    let parsed_depth_tokens = tails;

    if let Ok(_) = parsed_depth_tokens.next_is_leave(target.clone()) {
        // parse same level list
        let (_, parsed_list_tokens) =
            parsed_depth_tokens.next_are_ignore(ListLine::need_parsed_targets(&target))?;
        let (line, parsed_list_line) =
            parsed_list_tokens.to_somewhere_include(vec![Token::Newline, Token::EOF])?;
        let (spans, _) = span::parse(&line)?;

        // try to parse child list
        let (lines, parsed_list_child) =
            if let Ok(child_list_token) = first_token_in_same_depth(parsed_list_line, depth + 1) {
                let (lines, parsed_child_list) =
                    list_with(parsed_list_line, depth + 1, child_list_token)?;
                (lines, parsed_child_list)
            } else {
                // child is nothing
                (S::Nil, parsed_list_line)
            };

        let list_line = match target {
            Token::Index(n) => ListLine::Ordered(n, spans, Box::new(lines)),
            Token::Asterisk | Token::Plus | Token::Hyphen => {
                ListLine::Unordered(spans, Box::new(lines))
            }
            _ => unimplemented!(),
        };

        let (next_list, parsed_next_line) = list_with(
            parsed_list_child,
            depth,
            ListLine::next_list_target(&target),
        )?;

        Ok((S::cons(list_line, next_list), parsed_next_line))
    } else {
        Ok((S::Nil, tokens))
    }
}

fn first_token_in_same_depth(tokens: &S<Token>, depth: usize) -> Result<Token> {
    let mut tails = tokens;
    for _ in 0..depth {
        if let Ok((_, t)) = tails.next_are_or_ignore(vec![
            S::from_vector(vec![Token::Space, Token::Space]),
            S::from_vector(vec![Token::Indent]),
        ]) {
            tails = t;
        }
    }

    if let Ok(_) = tails.next_is_leave(Token::Asterisk) {
        Ok(Token::Asterisk)
    } else if let Ok(_) = tails.next_is_leave(Token::Plus) {
        Ok(Token::Plus)
    } else if let Ok(_) = tails.next_is_leave(Token::Hyphen) {
        Ok(Token::Hyphen)
    } else if let Ok(_) = tails.next_are_leave(S::from_vector(vec![Token::Index(1), Token::Dot])) {
        Ok(Token::Index(1))
    } else {
        Err(ParseError::not_found(vec![
            &Token::Asterisk,
            &Token::Plus,
            &Token::Hyphen,
        ]))
    }
}

pub fn list(tokens: &S<Token>) -> Result<(Block, &S<Token>)> {
    let list_token = first_token_in_same_depth(tokens, 0)?;
    let (l, tokens) = list_with(tokens, 0, list_token)?;
    Ok((Block::List(l), tokens))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ts_parse_list() {
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
            Ok(Block::List(S::from_vector(vec![
                ListLine::Unordered(S::unit(Span::text("hello1")), Box::new(S::Nil)),
                ListLine::Unordered(S::unit(Span::text("hello2")), Box::new(S::Nil)),
                ListLine::Unordered(S::unit(Span::text("hello3")), Box::new(S::Nil)),
            ])))
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
            Ok(Block::List(S::from_vector(vec![
                ListLine::Unordered(S::unit(Span::text("hello1")), Box::new(S::Nil)),
                ListLine::Unordered(
                    S::unit(Span::text("hello2")),
                    Box::new(S::from_vector(vec![
                        ListLine::Unordered(S::unit(Span::text("hello21")), Box::new(S::Nil)),
                        ListLine::Unordered(S::unit(Span::text("hello22")), Box::new(S::Nil)),
                        ListLine::Unordered(S::unit(Span::text("hello23")), Box::new(S::Nil)),
                    ]))
                ),
                ListLine::Unordered(S::unit(Span::text("hello3")), Box::new(S::Nil)),
            ])))
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
            Ok(Block::List(S::from_vector(vec![
                ListLine::Unordered(S::unit(Span::text("hello1")), Box::new(S::Nil)),
                ListLine::Unordered(
                    S::unit(Span::text("hello2")),
                    Box::new(S::from_vector(vec![
                        ListLine::Ordered(1, S::unit(Span::text("hello21")), Box::new(S::Nil)),
                        ListLine::Ordered(2, S::unit(Span::text("hello22")), Box::new(S::Nil)),
                        ListLine::Ordered(3, S::unit(Span::text("hello23")), Box::new(S::Nil)),
                    ]))
                ),
                ListLine::Unordered(S::unit(Span::text("hello3")), Box::new(S::Nil)),
            ])))
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
            Ok(Block::List(S::from_vector(vec![
                ListLine::Unordered(S::unit(Span::text("hello1")), Box::new(S::Nil)),
                ListLine::Unordered(
                    S::unit(Span::text("hello2")),
                    Box::new(S::from_vector(vec![
                        ListLine::Ordered(1, S::unit(Span::text("hello21")), Box::new(S::Nil)),
                        ListLine::Ordered(2, S::unit(Span::text("hello22")), Box::new(S::Nil)),
                        ListLine::Ordered(3, S::unit(Span::text("hello23")), Box::new(S::Nil)),
                    ]))
                ),
            ])))
        );
    }
}
