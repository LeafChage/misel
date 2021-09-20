use super::html::Html;
use md::{Block, S};

impl Html for S<Block> {
    fn html(&self) -> String {
        self.fold(String::from(""), |src, span| {
            format!("{}\n{}", src, span.html())
        })
    }
}
