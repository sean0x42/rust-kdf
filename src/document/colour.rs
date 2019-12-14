use serde::{Deserialize, Serialize};

pub type Colours = Vec<Shade>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Shade {
    id: String,
    name: String,
    value: String,
}
