use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheQuest {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,
}
