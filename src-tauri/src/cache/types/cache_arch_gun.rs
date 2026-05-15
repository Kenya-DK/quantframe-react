use serde::{Deserialize, Serialize};

use crate::cache::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArchGun {
    #[serde(flatten)]
    pub base: CacheWeaponBase,
}
