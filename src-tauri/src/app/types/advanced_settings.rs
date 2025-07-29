use serde::{Deserialize, Serialize};

use crate::app::SummaryCategorySetting;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub wf_log_path: String, // Path to the Warframe log file
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        AdvancedSettings {
            wf_log_path: String::from(""),
        }
    }
}
