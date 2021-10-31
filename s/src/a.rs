use std::fmt;
use std::marker::Sized;
/// get from stream
/// match?
/// return result

/// matchしたものを残すのかどうか leave, ignore, inlcude
/// Scan方法に関して指定 until, skip, next
/// matchの可能性の拡張 or, and,

// until().leave().and(
//     token('1'),
//     token('2'),
//     token('3'),
// ).parse(buffer)
//
// until().leave().or(
//     token('1'),
//     token('2'),
//     token('3'),
// ).parse(buffer)

// trait Parser {
//     fn parse(&self, Vec<u8>) -> (Output, Vec<u8>);
//     fn match(&self, Input);
// }

trait Is<T>
where
    T: Eq + fmt::Debug,
{
    fn is(&self, v: &T) -> Option<T>;
}

struct Token<T>
where
    T: Eq + fmt::Debug,
{
    value: T,
}

impl<T> Is<T> for Token<T>
where
    T: Eq + fmt::Debug + Copy,
{
    fn is(&self, v: &T) -> Option<T> {
        if self.value == *v {
            Some(self.value)
        } else {
            None
        }
    }
}

struct Or<T>
where
    T: Eq + fmt::Debug,
{
    values: Vec<Box<dyn Is<T>>>,
}

impl<T> Is<T> for Or<T>
where
    T: Eq + fmt::Debug,
{
    fn is(&self, v: &T) -> Option<T> {
        for value in self.values.iter() {
            if let Some(value) = value.is(&v) {
                return Some(value);
            }
        }
        None
    }
}

// struct And<T> {
//     tokens: Vec<Box<Token<T>>>,
// }
//
// impl<T> Token<T> for And<T> {
//     fn is(&self, input: T) -> bool {
//         tokens.is(input)
//     }
// }
//
// #[derive(Clone, Debug)]
// enum HowToHandle {
//     /// abcd until b return (a, cd)
//     Ignore,
//
//     /// abcd until b return (a, bcd)
//     Leave,
//
//     /// abcd until b return (ab, cd)
//     Include,
// }
//
// struct P<T> {
//     howToHandle: HowToHandle,
//     token: Token<T>,
// }
