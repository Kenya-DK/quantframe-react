use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheIngredient {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,

    #[serde(rename = "fromRecipe", skip_serializing_if = "Option::is_none")]
    pub from_recipe: Option<String>,
}
