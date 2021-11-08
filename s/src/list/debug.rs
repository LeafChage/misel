use super::S;
use std::fmt::{Debug, Formatter, Result};

impl<T> Debug for S<T>
where
    T: Eq + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            S::Cons(a, b) => write!(f, "({:?} {:?})", a, b),
            S::Nil => write!(f, "nil"),
        }
    }
}
