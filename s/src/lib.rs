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

use std::fmt;
use std::io::Result;
use stream::Stream;

pub trait Parser<Input, Output>
where
    Input: Eq + fmt::Debug + Clone + Copy,
{
    fn parse<'a, 'b>(
        &self,
        s: &'a mut Stream<Input>,
    ) -> Result<(Option<&'a Output>, &'a Stream<Input>)>;
}
