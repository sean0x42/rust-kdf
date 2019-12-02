pub mod node;

use node::Node;

/// A KDF document
#[derive(Debug)]
pub struct Document {
    pub content: Vec<Node>,
}

impl Document {
    /// Construct a new KDF document.
    pub fn new() -> Document {
        Document {
            content: Vec::new(),
        }
    }
}
