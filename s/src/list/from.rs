use super::S;
use std::convert::From;
use std::fmt::Debug;

impl<T> From<Vec<T>> for S<T>
where
    T: Eq + Debug,
{
    fn from(v: Vec<T>) -> S<T> {
        v.into_iter().rev().fold(S::Nil, |l, head| S::cons(head, l))
    }
}

impl From<&str> for S<u8> {
    fn from(v: &str) -> S<u8> {
        S::from(v.as_bytes().to_vec())
    }
}

#[test]
fn ts_from_vector() {
    assert_eq!(
        S::from(vec![1, 2, 3]),
        S::cons(1, S::cons(2, S::cons(3, S::Nil)))
    )
}

#[test]
fn ts_from_str() {
    assert_eq!(S::from("123"), S::from(vec![b'1', b'2', b'3']))
}
