use serde::{Deserialize, Serialize};

use super::cache_item_base::CacheItemBase;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CachePet {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,
}
