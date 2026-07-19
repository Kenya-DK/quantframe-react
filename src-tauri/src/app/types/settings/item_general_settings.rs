use serde::{Deserialize, Serialize};
use utils::SubType;

use crate::enums::TradeMode;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemGeneralSettings {
    pub blacklist: Vec<BlackListItemSetting>,
    pub buy_list: Vec<BuyListItemSetting>,
}
impl ItemGeneralSettings {
    pub fn is_item_blacklisted(
        &self,
        wfm_id: &str,
        sub_type: &Option<SubType>,
        mode: &TradeMode,
    ) -> bool {
        for item in &self.blacklist {
            if item.is_disabled_for(wfm_id, sub_type, mode) {
                return true;
            }
        }
        false
    }
    pub fn get_item_max_price(&self, wfm_id: &str) -> i64 {
        for item in &self.buy_list {
            if item.wfm_id == wfm_id {
                return item.max_price;
            }
        }
        0
    }
}
impl Default for ItemGeneralSettings {
    fn default() -> Self {
        Self {
            blacklist: Vec::new(),
            buy_list: Vec::new(),
        }
    }
}
