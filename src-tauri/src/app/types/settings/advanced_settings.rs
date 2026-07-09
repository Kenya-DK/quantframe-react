use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub http_server: HttpServerSettings,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        AdvancedSettings {
            http_server: HttpServerSettings::default(),
        }
    }
}
