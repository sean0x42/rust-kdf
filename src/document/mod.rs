pub mod colour;
pub mod dictionary;
pub mod metadata;
pub mod node;
pub mod styles;

use colour::Colours;
use dictionary::Dictionary;
use metadata::Metadata;
use serde::{Deserialize, Serialize};
use styles::Styles;

/// A KDF document
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub content: Vec<node::Node>,
    pub colours: Colours,
    pub dictionary: Dictionary,
    pub metadata: Metadata,
    pub styles: Styles,
}

impl Document {
    /// Create a new KDF document
    pub fn new(title: String) -> Document {
        Document {
            colours: Colours::new(),
            content: Vec::new(),
            dictionary: Dictionary::new(),
            metadata: Metadata::new(title),
            styles: Styles {},
        }
    }
}
