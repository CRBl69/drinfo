use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageInsertion {
    base64: String,
}

impl ImageInsertion {
    pub fn new(base64: &str) -> Self {
        ImageInsertion {
            base64: String::from(base64),
        }
    }
}
