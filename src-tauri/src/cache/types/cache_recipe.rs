use std::{collections::HashMap, fmt::Display};

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
    pub fn can_craft(
        &self,
        tradeable_only: bool,
        from_recipe_only: bool,
        items: &[CacheItemBase],
    ) -> bool {
        let mut owned_counts: HashMap<String, i64> = HashMap::new();
        for item in items {
            *owned_counts.entry(item.unique_name.clone()).or_insert(0) += item.quantity;
        }
        // Only tradeable crafted parts
        let required_parts: Vec<&CacheIngredient> = self
            .ingredients
            .iter()
            .filter(|ingredient| tradeable_only == false || ingredient.base.is_tradeable)
            .collect();
        if required_parts.is_empty() {
            return false;
        }
        let has_all_parts = required_parts.iter().all(|ingredient| {
            let ingredient_key = if from_recipe_only {
                ingredient.from_recipe.clone()
            } else {
                ingredient.base.unique_name.clone()
            };
            if ingredient_key.is_empty() {
                return false;
            }
            let owned = owned_counts.get(&ingredient_key).copied().unwrap_or(0);
            owned >= ingredient.base.quantity
        });

        if !has_all_parts {
            return false;
        }

        if !self.base.is_tradeable && tradeable_only {
            return false;
        }

        // Check Main Blueprint requirement if needed
        if self.base.is_tradeable && tradeable_only {
            let main_blueprint_key = if self.override_unique_name.is_empty() {
                self.base.unique_name.clone()
            } else {
                self.override_unique_name.clone()
            };
            let main_blueprint_owned = owned_counts.get(&main_blueprint_key).copied().unwrap_or(0);
            if main_blueprint_owned < 1 {
                return false;
            }
        }

        true
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
