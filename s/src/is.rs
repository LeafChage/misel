use std::fmt;

pub trait Is<T>
where
    T: Eq + fmt::Debug + Clone,
{
    fn is(&self, v: &T) -> Option<T>;
}

// pub trait Scanner<T>
// where
//     T: Eq + fmt::Debug + Clone,
// {
//     fn on_memory(v: Stream) -> (T, Stream);
//     fn is(&self, v: &T) -> Option<T>;
// }
//
///
///
/// token
///
///
pub struct Token<T> {
    value: T,
}

impl<T> Token<T>
where
    T: Eq + fmt::Debug + Clone,
{
    fn new(v: T) -> Self {
        Token { value: v }
    }

    fn or(&self, other: T) -> Or<T> {
        Or {
            values: vec![
                Box::new(Self::new(self.value.clone())),
                Box::new(Self::new(other)),
            ],
        }
    }
}

impl<T> Is<T> for Token<T>
where
    T: Eq + fmt::Debug + Clone,
{
    fn is(&self, v: &T) -> Option<T> {
        if self.value == *v {
            Some(self.value)
        } else {
            None
        }
    }
}

#[test]
fn ts_is_token() {
    assert_eq!(Token::<char>::new('a').is(&'a'), Ok('a'));
    assert_eq!(Token::<char>::new('a').is(&'b'), None);
}

///
///
/// or
///
///
pub struct Or<T> {
    values: Vec<Box<dyn Is<T>>>,
}

impl<T> Or<T>
where
    T: Eq + fmt::Debug + Clone,
{
    fn or(self, other: T) -> Or<T> {
        self.values.push(Box::new(Token::new(other)));
        return self;
    }
}

impl<T> Is<T> for Or<T>
where
    T: Eq + fmt::Debug + Clone,
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

#[test]
fn ts_is_or() {
    assert_eq!(Token::<char>::new('a').or('b').is(&'a'), Ok('a'));
    assert_eq!(Token::<char>::new('a').or('b').is(&'b'), Ok('b'));
    assert_eq!(Token::<char>::new('a').or('b').or('c').is(&'c'), Ok('c'));
    assert_eq!(Token::<char>::new('a').or('b').or('c').is(&'d'), None);
}

///
///
/// and
///
///
#[test]
fn ts_is_and() {
    // assert_eq!(Token::<char>::new('a').and('b').is(&'a'), Ok('a'));
    // assert_eq!(Token::<char>::new('a').and('b').is(&'b'), Ok('b'));
    // assert_eq!(Token::<char>::new('a').and('b').is(&'c'), None);
}
