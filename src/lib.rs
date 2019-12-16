pub mod document;
pub mod helpers;

use document::Document;
use std::path::PathBuf;

/// Parses a KDF document at a given path
pub fn parse_kdf_document(path: PathBuf) -> Document {
    println!("Parsing {}", path.to_str().unwrap_or("None"));
    Document::new(String::from("Hello world"))
}
