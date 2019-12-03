use chrono::prelude::*;
use std::collections::HashMap;

/// # Metadata
///
/// This struct contains all required metadata keys for a valid KDF document,
/// in accordance with the specification. For a full list of required keys,
/// see https://kdf.now.sh/specification/metadata.html#required-metadata.
///
/// There is more to metadata than required keys however. Applications may use
/// the metadata file as a generic key-value store, as long as the values are
/// a valid JSON data type.
#[derive(Debug)]
pub struct Metadata {
    title: String,
    authors: Vec<String>,
    created_at: String,
    updated_at: String,
    edit_duration: String,
    kdf_version: String,
    supported_mediums: Vec<String>,
    custom: HashMap<String, MetadataOption>,
}

pub type MetadataOption = serde_json::Value;

impl Metadata {
    /// Construct a new metadata struct
    pub fn new(title: String) -> Metadata {
        let now = Utc::now();
        Metadata {
            title,
            authors: Vec::new(),
            created_at: now.to_string(),
            updated_at: now.to_string(),
            edit_duration: String::from("PT0S"),
            kdf_version: String::from("0.1.0-alpha"),
            supported_mediums: Vec::new(),
            custom: HashMap::new(),
        }
    }
}
