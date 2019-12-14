use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contains document metadata
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    title: String,
    authors: Vec<String>,
    created_at: String,
    updated_at: String,
    edit_duration: String,
    kdf_version: String,
    supported_mediums: Vec<String>,

    /// All remaining, custom metadata entries
    /// Serde will automatically flatten these into the metadata object when
    /// serializing to JSON for us.
    #[serde(flatten)]
    custom: HashMap<String, MetadataOption>,
}

/// A possible metadata entry
pub type MetadataOption = serde_json::Value;

impl Metadata {
    /// Create a new metadata entry
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
