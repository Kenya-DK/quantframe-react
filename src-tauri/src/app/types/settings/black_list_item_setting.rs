use serde::{Deserialize, Serialize};

use crate::enums::TradeMode;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlackListItemSetting {
    #[serde(rename = "wfmId", alias = "wfm_id")]
    pub wfm_id: String,
    pub disabled_for: Vec<TradeMode>,
}

impl BlackListItemSetting {
    pub fn is_disabled_for(&self, wfm_id: impl Into<String>, mode: &TradeMode) -> bool {
        self.wfm_id == wfm_id.into() && self.disabled_for.contains(mode)
    }
}
