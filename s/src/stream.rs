use std::convert::From;
use super::Scanner;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Stream<T>
where
    T: Eq + fmt::Debug,
{
    buffer: Vec<T>,
    index: usize,
}

impl<T> Stream<T>
where
    T: Eq + fmt::Debug,
{
    pub fn look(&self) -> Option<&T> {
        self.buffer.get(self.index)
    }

    pub fn look_to(&self, to: usize) -> Option<&[T]> {
        self.buffer.get(self.index..self.index + to)
    }

    pub fn next(&mut self) -> Option<&T> {
        let result = self.buffer.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result
    }

    pub fn next_to(&mut self, to: usize) -> Option<&[T]> {
        let result = self.buffer.get(self.index..self.index + to);
        if result.is_some() {
            self.index += to;
        }
        result
    }

    pub fn scan<T, S>(&mut self, scanner: &S) -> Option<&[T]>
        where
            S: Scanner<T, T>
    {
        let Some(v) = self.look_to(size) {
            if scanner.m(v) {
                scanner.handle()
            }
            self.index += to;
        }
    }
}

impl<T> From<Vec<T>> for Stream<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
{
    fn from(v: Vec<T>) -> Self {
        Stream {
            buffer: v,
            index: 0,
        }
    }
}

#[test]
fn ts_look() {
    let stream = Stream::from(vec![1, 2, 3]);
    assert_eq!(stream.look(), Some(&1),);
    assert_eq!(stream, Stream::from(vec![1, 2, 3]),);
}

#[test]
fn ts_look_to() {
    let stream = Stream::from(vec![1, 2, 3]);
    assert_eq!(stream.look_to(2), Some(&vec![1, 2][..]),);
    assert_eq!(stream, Stream::from(vec![1, 2, 3]),);
}

#[test]
fn ts_next() {
    let mut stream = Stream::from(vec![1, 2, 3]);
    assert_eq!(stream.next(), Some(&1),);
    assert_eq!(
        stream,
        Stream {
            buffer: vec![1, 2, 3],
            index: 1
        },
    );
}

#[test]
fn ts_next_to() {
    let mut stream = Stream::from(vec![1, 2, 3]);
    assert_eq!(stream.next_to(2), Some(&vec![1, 2][..]),);
    assert_eq!(
        stream,
        Stream {
            buffer: vec![1, 2, 3],
            index: 2
        },
    );
}

#[test]
fn ts_from_vec() {
    assert_eq!(
        Stream::from(vec![1, 2, 3]),
        Stream {
            buffer: vec![1, 2, 3],
            index: 0,
        }
    );
}
