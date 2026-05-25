use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheIngredient {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,

    #[serde(rename = "fromRecipe", default)]
    pub from_recipe: String,
}
impl Display for CacheIngredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<String> = vec![self.base.to_string()];
        if !self.from_recipe.is_empty() {
            items.push(format!("From Recipe: {}", self.from_recipe));
        }
        write!(f, "{}", items.join(" | "))
    }
}
