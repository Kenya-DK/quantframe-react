use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RivenSettings {
    pub general: RivenGeneralSettings,
    pub wts: RivenWtsSettings,
}

impl Default for RivenSettings {
    fn default() -> Self {
        Self {
            general: RivenGeneralSettings::default(),
            wts: RivenWtsSettings::default(),
        }
    }
}
