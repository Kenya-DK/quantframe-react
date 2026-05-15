use serde::{Deserialize, Serialize};

use crate::cache::*;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSecondary {
    #[serde(flatten)]
    pub base: CacheWeaponBase,
}
