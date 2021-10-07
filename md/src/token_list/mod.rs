use crate::tokenize::Token;
use s::S;

pub trait TokenList {
    fn chmop(&self) -> Self;
    fn show(&self) -> String;
}

impl TokenList for S<Token> {
    fn chmop(&self) -> Self {
        let head = self.head();
        match head {
            Some(head) => match head {
                Token::Space | Token::Newline => {
                    let s = self.tail().chmop();
                    if s == S::Nil {
                        S::Nil
                    } else {
                        S::cons(head.clone(), s)
                    }
                }
                _ => S::cons(head.clone(), self.tail().chmop()),
            },
            None => S::Nil,
        }
    }

    fn show(&self) -> String {
        self.fold(String::new(), |a, b| format!("{}{}", a, b.show()))
    }
}
