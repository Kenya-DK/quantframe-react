use serde::{Deserialize, Serialize};

use crate::cache::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheMod {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,

    #[serde(rename = "upgradeEntries", default)]
    pub upgrade_entries: Vec<CacheUpgradeEntry>,
}
