// mod and;
mod error;
mod list;
mod or;
// mod scanner;
mod how_to_handle;
mod stream;
mod token;

pub use error::ScannerError;
pub use list::S;

use std::io::Result;
use stream::Stream;

pub trait Scanner<Input, Output>
where
    Input: Eq + std::fmt::Debug,
{
    fn size(&self) -> usize;
    fn pass(&self, v: Input) -> bool;
}

pub trait Parser<Input, Output>
where
    Input: Eq + std::fmt::Debug + Clone + Copy,
{
    fn parse<'a>(&self, s: &'a mut Stream<Input>) -> Result<Option<&'a Output>>;
}
