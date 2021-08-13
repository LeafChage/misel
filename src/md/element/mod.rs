pub mod block;
pub mod miscellaneous;
pub mod service;
pub mod span;
mod util;
use util::chain::Chain;

pub use block::{Block, BlockChain};
pub use service::Html;
pub use span::emphasis::{Emphasis, Token as EmphasisToken};
pub use span::{Span, SpanChain};
