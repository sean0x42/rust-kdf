pub mod colour;
pub mod metadata;
pub mod node;

use metadata::Metadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub content: Vec<node::Node>,
    pub colours: Vec<colour::Shade>,
    pub dictionary: Vec<String>,
    pub metadata: Metadata,
}

impl Document {
    pub fn new(title: String) -> Document {
        Document {
            colours: Vec::new(),
            content: Vec::new(),
            dictionary: Vec::new(),
            metadata: Metadata::new(title),
        }
    }
}
