use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::cache::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRecipe {
    #[serde(flatten)]
    pub base: super::cache_item_base::CacheItemBase,

    #[serde(rename = "resultType", default)]
    pub result_type: String,

    #[serde(rename = "overrideUniqueName", default)]
    pub override_unique_name: String,

    #[serde(rename = "ingredients", default)]
    pub ingredients: Vec<CacheIngredient>,
}

impl CacheRecipe {
    pub fn keys(&self, tradeable_only: bool, from_recipe_only: bool) -> Vec<String> {
        let mut keys = Vec::new();

        let include_base =
            (!tradeable_only || self.base.is_tradeable) && !self.base.unique_name.is_empty();

        if include_base {
            keys.push(format!("{}|{}", self.base.unique_name, 1));
        }

        for ingredient in &self.ingredients {
            if tradeable_only && !ingredient.base.is_tradeable {
                continue;
            }

            let ingredient_key = if from_recipe_only {
                &ingredient.from_recipe
            } else {
                &ingredient.base.unique_name
            };

            if ingredient_key.is_empty() {
                continue;
            }

            keys.push(format!("{}|{}", ingredient_key, ingredient.base.quantity));
        }

        // Ensure deterministic ordering
        keys.sort();

        keys
    }
}

impl Display for CacheRecipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<String> = vec![self.base.to_string()];
        if !self.result_type.is_empty() {
            items.push(format!("Result Type: {}", self.result_type));
        }
        if !self.override_unique_name.is_empty() {
            items.push(format!(
                "Override Unique Name: {}",
                self.override_unique_name
            ));
        }
        if !self.ingredients.is_empty() {
            let ingredient_str = self
                .ingredients
                .iter()
                .map(|ing| ing.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            items.push(format!("Ingredients: [{}]", ingredient_str));
        }
        write!(f, "{}", items.join(" | "))
    }
}
