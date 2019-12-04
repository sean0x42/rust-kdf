pub mod colour;
pub mod dictionary;
pub mod metadata;
pub mod node;
pub mod styles;

use dictionary::Dictionary;
use metadata::Metadata;
use serde::{Deserialize, Serialize};
use styles::Styles;

/// A KDF document
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub content: Vec<node::Node>,
    pub colours: Vec<colour::Shade>,
    pub dictionary: Dictionary,
    pub metadata: Metadata,
    pub styles: Styles,
}

impl Document {
    /// Create a new KDF document
    pub fn new(title: String) -> Document {
        Document {
            colours: Vec::new(),
            content: Vec::new(),
            dictionary: Vec::new(),
            metadata: Metadata::new(title),
            styles: Styles {},
        }
    }
}
