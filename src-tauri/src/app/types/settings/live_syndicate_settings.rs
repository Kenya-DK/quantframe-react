use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyndicateSettings {
    pub wts: LiveSyndicateWtsSettings,
}
impl Default for SyndicateSettings {
    fn default() -> Self {
        Self {
            wts: LiveSyndicateWtsSettings::default(),
        }
    }
}
