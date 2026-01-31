use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{dto::*, enums::*, transaction::Model as TransactionModel, wish_list::*};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateWishListItem {
    // Properties use for validation
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maximum_price")]
    pub maximum_price: Option<i64>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

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

impl CreateWishListItem {
    pub fn new(raw: impl Into<String>, sub_type: Option<SubType>, quantity: i64) -> Self {
        CreateWishListItem {
            raw: raw.into(),
            wfm_id: "".to_string(),
            wfm_url: "".to_string(),
            credits: 0,
            item_name: "".to_string(),
            item_unique_name: "".to_string(),
            tags: vec![],
            maximum_price: None,
            bought: None,
            quantity,
            sub_type,
            is_validated: false,
        }
    }

    pub fn set_bought(mut self, bought: i64) -> Self {
        self.bought = Some(bought);
        self
    }
    pub fn to_transaction(&self, user_name: impl Into<String>) -> Result<TransactionModel, String> {
        if !self.is_validated {
            return Err("Wish list item is not validated yet".to_string());
        }
        let transaction = TransactionModel::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            TransactionItemType::Item,
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.tags.clone(),
            TransactionType::Purchase,
            self.quantity,
            user_name.into(),
            self.bought.unwrap_or(0),
            self.credits,
            None,
        );
        Ok(transaction)
    }
    pub fn to_model(&self) -> Model {
        Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.maximum_price.clone(),
            self.quantity.clone(),
        )
    }
}

impl Display for CreateWishListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateWishListItem ")?;
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
        if let Some(maximum_price) = self.maximum_price {
            write!(f, "Maximum Price: {}, ", maximum_price)?;
        } else {
            write!(f, "Maximum Price: Not provided, ")?;
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
