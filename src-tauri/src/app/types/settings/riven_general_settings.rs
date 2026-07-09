use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RivenGeneralSettings {
    pub update_interval: i64, // in seconds
}

impl Default for RivenGeneralSettings {
    fn default() -> Self {
        Self {
            update_interval: 120,
        }
    }
}
