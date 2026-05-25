use serde::{Deserialize, Serialize};

use super::cache_item_base::CacheItemBase;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheGear {
    #[serde(flatten)]
    pub base: CacheItemBase,
}
