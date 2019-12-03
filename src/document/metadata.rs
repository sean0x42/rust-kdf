use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(flatten)]
    custom: HashMap<String, MetadataOption>,
}

pub type MetadataOption = serde_json::Value;

impl Metadata {
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
