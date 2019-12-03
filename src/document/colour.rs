use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shade {
    id: String,
    name: String,
    value: String,
}
