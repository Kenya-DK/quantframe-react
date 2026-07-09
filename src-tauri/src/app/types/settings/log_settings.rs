use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogSettings {
    pub ee_log_path: String,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            ee_log_path: String::from(""),
        }
    }
}
