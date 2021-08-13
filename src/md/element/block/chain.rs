use super::super::{Chain, Html};
use super::Block;

pub type BlockChain = Chain<Block>;

impl Html for BlockChain {
    fn html(&self) -> String {
        if let Some(ref tail) = self.tail {
            format!("{}\n{}", self.head.html(), tail.html())
        } else {
            format!("{}", self.head.html())
        }
    }
}
