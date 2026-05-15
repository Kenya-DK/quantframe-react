use serde::{Deserialize, Serialize};

use crate::cache::*;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CachePrimary {
    #[serde(flatten)]
    pub base: CacheWeaponBase,
}
