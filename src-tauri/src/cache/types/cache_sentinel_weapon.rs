use serde::{Deserialize, Serialize};

use crate::cache::CacheWeaponBase;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSentinelWeapon {
    #[serde(flatten)]
    pub base: CacheWeaponBase,
}
