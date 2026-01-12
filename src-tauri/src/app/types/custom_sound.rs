use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomSound {
    pub name: String,
    pub file_name: String,
}

impl CustomSound {
    pub fn new(name: String, file_name: String) -> Self {
        Self { name, file_name }
    }
}
