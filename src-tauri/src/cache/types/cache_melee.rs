use serde::{Deserialize, Serialize};

use crate::cache::*;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheMelee {
    #[serde(flatten)]
    pub base: CacheWeaponBase,
}
