use serde::{Deserialize, Serialize};

use crate::cache::*;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArchMelee {
    #[serde(flatten)]
    pub base: CacheWeaponBase,
}
