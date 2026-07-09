use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInventorySettings {
    pub inv_path: String,
}

impl Default for WFInventorySettings {
    fn default() -> Self {
        Self {
            inv_path: String::from(""),
        }
    }
}
