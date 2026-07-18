use entity::dto::SubType;
use serde::{Deserialize, Serialize};

use crate::enums::TradeMode;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlackListItemSetting {
    #[serde(rename = "wfmId", alias = "wfm_id")]
    pub wfm_id: String,
    #[serde(rename = "subType", alias = "sub_type")]
    pub sub_type: Option<SubType>,
    pub disabled_for: Vec<TradeMode>,
}

impl BlackListItemSetting {
    pub fn is_disabled_for(
        &self,
        wfm_id: impl Into<String>,
        sub_type: &Option<SubType>,
        mode: &TradeMode,
    ) -> bool {
        let wfm_id = wfm_id.into();
        if self.wfm_id == wfm_id && self.sub_type.is_none() && self.disabled_for.contains(mode) {
            return true;
        }
        self.wfm_id == wfm_id && self.sub_type == *sub_type && self.disabled_for.contains(mode)
    }
}
