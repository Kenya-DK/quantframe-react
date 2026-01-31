use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::stock_item::*;

use crate::dto::*;
use crate::enums::*;
use crate::transaction::Model as TransactionModel;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockItem {
    // Properties use for validation
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<i64>,

    // Set By validation method
    #[serde(default = "String::default")]
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(default = "String::default")]
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(default)]
    #[serde(rename = "credits")]
    pub credits: i64,

    #[serde(default = "String::default")]
    #[serde(rename = "item_name")]
    pub item_name: String,

    #[serde(default = "String::default")]
    #[serde(rename = "item_unique_name")]
    pub item_unique_name: String,

    #[serde(default)]
    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "is_validated")]
    #[serde(default = "bool::default")]
    pub is_validated: bool,
}

impl CreateStockItem {
    pub fn new(raw: impl Into<String>, sub_type: Option<SubType>, quantity: i64) -> Self {
        CreateStockItem {
            raw: raw.into(),
            wfm_id: "".to_string(),
            wfm_url: "".to_string(),
            credits: 0,
            item_name: "".to_string(),
            item_unique_name: "".to_string(),
            tags: vec![],
            sub_type,
            bought: None,
            minimum_price: None,
            quantity,
            is_validated: false,
        }
    }

    pub fn set_bought(mut self, bought: i64) -> Self {
        self.bought = Some(bought);
        self
    }

    pub fn to_model(&self) -> Model {
        Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.bought.unwrap_or(0),
            self.minimum_price,
            self.quantity.clone(),
            false,
        )
    }
    pub fn to_transaction(&self, user_name: impl Into<String>) -> Result<TransactionModel, String> {
        if !self.is_validated {
            return Err("Stock item is not validated yet".to_string());
        }
        let transaction_type = if self.bought.unwrap_or(0) > 0 {
            TransactionType::Purchase
        } else {
            TransactionType::Sale
        };
        let transaction = TransactionModel::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            TransactionItemType::Item,
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.tags.clone(),
            transaction_type,
            self.quantity,
            user_name.into(),
            self.bought.unwrap_or(0),
            self.credits,
            None,
        );
        Ok(transaction)
    }
}

impl Display for CreateStockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateStockItem ")?;
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
        if let Some(bought) = self.bought {
            write!(f, "Bought: {}, ", bought)?;
        } else {
            write!(f, "Bought: Not provided, ")?;
        }
        if let Some(minimum_price) = self.minimum_price {
            write!(f, "Minimum Price: {}, ", minimum_price)?;
        } else {
            write!(f, "Minimum Price: Not provided, ")?;
        }
        write!(f, "Quantity: {}, ", self.quantity)?;
        if let Some(sub_type) = &self.sub_type {
            write!(f, "Sub Type: {}, ", sub_type.display())?;
        } else {
            write!(f, "Sub Type: Not provided, ")?;
        }
        write!(f, "Is Validated: {}", self.is_validated)?;
        Ok(())
    }
}
