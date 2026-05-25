use std::fmt::Display;

use entity::dto::sub_type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemBase {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "category", default)]
    pub category: String,

    #[serde(rename = "source", default)]
    pub source: String,

    #[serde(rename = "wfmUrl")]
    pub wfm_url: Option<String>,

    #[serde(rename = "tags", default)]
    pub tags: Vec<String>,

    #[serde(rename = "subType")]
    pub sub_type: Option<sub_type::SubType>,

    #[serde(rename = "previousNames", default)]
    pub previous_names: Vec<String>,

    #[serde(rename = "ItemCount", default)]
    pub quantity: i64,

    #[serde(rename = "isTradeable", default)]
    pub is_tradeable: bool,
}
impl CacheItemBase {
    pub fn new(unique_name: impl Into<String>, quantity: i64) -> Self {
        Self {
            unique_name: unique_name.into(),
            name: String::new(),
            category: String::new(),
            source: String::new(),
            wfm_url: None,
            sub_type: None,
            previous_names: vec![],
            tags: vec![],
            quantity,
            is_tradeable: false,
        }
    }
}

impl Display for CacheItemBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<String> = vec![];
        if !self.name.is_empty() {
            items.push(format!("Name: {}", self.name));
        }
        if !self.unique_name.is_empty() {
            items.push(format!("Unique Name: {}", self.unique_name));
        }

        if let Some(sub_type) = &self.sub_type {
            items.push(format!("Sub Type: {}", sub_type.display()));
        }

        items.push(format!("Quantity: {}", self.quantity));

        if !self.category.is_empty() {
            items.push(format!("Category: {}", self.category));
        }
        if !self.source.is_empty() {
            items.push(format!("Source: {}", self.source));
        }
        if let Some(wfm_url) = &self.wfm_url {
            items.push(format!("WFM URL: {}", wfm_url));
        }

        if self.is_tradeable {
            items.push(format!("Tradeable"));
        }
        if !self.tags.is_empty() {
            items.push(format!("Tags: [{}]", self.tags.join(", ")));
        }
        write!(f, "{}", items.join(" | "))
    }
}
