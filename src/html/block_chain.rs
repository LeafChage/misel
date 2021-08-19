use super::html::Html;
use crate::md::element::BlockChain;

impl Html for BlockChain {
    fn html(&self) -> String {
        if let Some(ref tail) = self.tail {
            format!("{}\n{}", self.head.html(), tail.html())
        } else {
            format!("{}", self.head.html())
        }
    }
}
