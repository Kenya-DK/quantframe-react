use crate::wf_inventory::WFInventorySource;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInventorySettings {
    pub source: WFInventorySource,
}

impl Default for WFInventorySettings {
    fn default() -> Self {
        Self {
            source: WFInventorySource::None,
        }
    }
}
