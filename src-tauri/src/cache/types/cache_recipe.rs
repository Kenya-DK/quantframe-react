use serde::{Deserialize, Serialize};

use crate::cache::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRecipe {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,

    #[serde(rename = "ingredients", default)]
    pub ingredients: Vec<CacheIngredient>,
}
