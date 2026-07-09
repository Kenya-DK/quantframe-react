use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomSound {
    pub name: String,
    pub name_key: String,
    pub file_name: String,
}

impl CustomSound {
    pub fn new(name: String, file_name: String) -> Self {
        Self {
            name_key: name.to_lowercase(),
            name,
            file_name,
        }
    }
}
