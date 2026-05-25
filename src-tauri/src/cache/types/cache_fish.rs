use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheFish {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,
}
