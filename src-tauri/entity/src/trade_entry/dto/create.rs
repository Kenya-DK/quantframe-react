use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::trade_entry::*;

use crate::dto::*;
use crate::enums::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateTradeEntry {
    // Properties use for validation
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(default)]
    pub wfm_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub sub_type: Option<SubType>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub price: i64,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub is_validated: bool,
}

impl CreateTradeEntry {
    pub fn new(raw: impl Into<String>, sub_type: Option<SubType>) -> Self {
        CreateTradeEntry {
            raw: raw.into(),
            sub_type,
            tags: Vec::new(),
            wfm_id: String::new(),
            name: String::new(),
            price: 0,
            group: String::new(),
            properties: None,
            is_validated: false,
        }
    }
    pub fn to_model(&self) -> Model {
        Model::new(
            self.wfm_id.clone(),
            self.name.clone(),
            self.sub_type.clone(),
            self.price,
            self.tags.clone(),
            self.group.clone(),
            self.properties.clone(),
        )
    }
}

impl Display for CreateTradeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateTradeEntry ")?;
        if self.raw.is_empty() {
            write!(f, "Raw: Not provided, ")?;
        } else {
            write!(f, "Raw: {}, ", self.raw)?;
        }

        Ok(())
    }
}
