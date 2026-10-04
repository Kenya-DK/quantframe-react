use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use utils::Properties;
use utils::SubType;

use crate::syndicate_item::*;

use crate::enums::*;
use crate::transaction::Model as TransactionModel;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateSyndicateItem {
    // Properties use for validation
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "raw_syndicate")]
    pub raw_syndicate: String,

    // Set By validation method
    #[serde(default = "String::default")]
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(default = "String::default")]
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(default = "String::default")]
    #[serde(rename = "item_name")]
    pub item_name: String,

    #[serde(default = "String::default")]
    #[serde(rename = "item_unique_name")]
    pub item_unique_name: String,

    #[serde(rename = "syndicate_unique_name")]
    pub syndicate_unique_name: String,

    #[serde(rename = "syndicate_name")]
    pub syndicate_name: String,

    #[serde(default)]
    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "standing_cost")]
    pub standing_cost: i64,

    #[serde(default, flatten)]
    pub properties: Properties,

    #[serde(rename = "is_validated")]
    #[serde(default = "bool::default")]
    pub is_validated: bool,
}

impl CreateSyndicateItem {
    pub fn new(
        raw: impl Into<String>,
        sub_type: Option<SubType>,
        raw_syndicate: impl Into<String>,
    ) -> Self {
        CreateSyndicateItem {
            raw: raw.into(),
            raw_syndicate: raw_syndicate.into(),
            wfm_id: "".to_string(),
            wfm_url: "".to_string(),
            item_name: "".to_string(),
            item_unique_name: "".to_string(),
            tags: vec![],
            sub_type,
            syndicate_unique_name: "".to_string(),
            syndicate_name: "".to_string(),
            standing_cost: 0,
            is_validated: false,
            properties: Properties::default(),
        }
    }

    pub fn to_model(&self) -> Model {
        let model = Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.raw_syndicate.clone(),
            self.syndicate_unique_name.clone(),
            self.standing_cost,
            self.properties.clone(),
        );
        model
    }
    pub fn to_transaction(
        &self,
        user_name: impl Into<String>,
        price: i64,
    ) -> Result<TransactionModel, String> {
        if !self.is_validated {
            return Err("Syndicate item is not validated yet".to_string());
        }
        let transaction = TransactionModel::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            TransactionItemType::Item,
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.tags.clone(),
            TransactionType::Sale,
            1,
            user_name.into(),
            price,
            0,
            Some(json!({
                "unique_name": &self.syndicate_unique_name,
                "cost": &self.standing_cost,
            })),
        );
        Ok(transaction)
    }
}

impl Display for CreateSyndicateItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateSyndicateItem ")?;
        if self.raw.is_empty() {
            write!(f, "Raw: Not provided, ")?;
        } else {
            write!(f, "Raw: {}, ", self.raw)?;
        }
        if self.wfm_id.is_empty() {
            write!(f, "WFM ID: Not provided, ")?;
        } else {
            write!(f, "WFM ID: {}, ", self.wfm_id)?;
        }
        write!(f, "WFM URL: {}, ", self.wfm_url)?;
        write!(f, "Item Name: {}, ", self.item_name)?;
        write!(f, "Item Unique Name: {}, ", self.item_unique_name)?;
        write!(f, "Tags: {:?}, ", self.tags)?;
        write!(f, "Syndicate Name: {}, ", self.raw_syndicate)?;
        write!(f, "Syndicate Unique Name: {}, ", self.syndicate_unique_name)?;
        write!(f, "Standing Cost: {}, ", self.standing_cost)?;
        if let Some(sub_type) = &self.sub_type {
            write!(f, "Sub Type: {}, ", sub_type.display())?;
        } else {
            write!(f, "Sub Type: Not provided, ")?;
        }
        write!(f, "Is Validated: {}", self.is_validated)?;
        Ok(())
    }
}
