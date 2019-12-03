pub mod metadata;
pub mod node;

use metadata::Metadata;
use node::Node;

/// A KDF document
#[derive(Debug)]
pub struct Document {
    pub content: Vec<Node>,
    pub metadata: Metadata,
}

impl Document {
    /// Construct a new KDF document.
    pub fn new(title: String) -> Document {
        Document {
            content: Vec::new(),
            metadata: Metadata::new(title),
        }
    }
}
