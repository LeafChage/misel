use std::convert::From;
use std::fmt;

pub struct Stream<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
{
    buffer: [T],
    index: usize,
}

impl<T> Stream<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
{
    pub fn look(&self) -> Option<&T> {
        self.buffer.get(self.index + 1)
    }

    pub fn look_to(&self, to: usize) -> Option<&[T]> {
        self.buffer.get(0..self.index + to)
    }

    pub fn next(&mut self) -> Option<&T> {
        self.buffer.get(self.index + 1).map(|v| {
            self.index += 1;
            v
        })
    }

    pub fn next_to(&mut self, to: usize) -> Option<&[T]> {
        self.buffer.get(0..self.index + to).map(|v| {
            self.index += to;
            v
        })
    }
}

impl<T> From<Vec<T>> for Stream<T>
where
    T: Eq + fmt::Debug + Clone + Copy,
{
    fn from(v: Vec<T>) -> Self {
        Stream {
            buffer: v[..],
            index: 0,
        }
    }
}
