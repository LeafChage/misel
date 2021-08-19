pub mod block;
pub mod miscellaneous;
pub mod span;
mod util;
use util::chain::Chain;

pub use block::{Block, BlockChain};
pub use span::emphasis::{Emphasis, Token as EmphasisToken};
pub use span::{Span, SpanChain};
