use entity::{
    enums::stock_type::StockType,
    stock::{
        item::create::CreateStockItem,
        riven::{attribute::RivenAttribute, create::CreateStockRiven},
    },
    sub_type::SubType,
    transaction,
};
use serde::{Deserialize, Serialize};

use crate::{cache::{client::CacheClient, types::cache_tradable_item}, utils::modules::error::AppError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockEntity {
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,

    #[serde(rename = "entity_type")]
    pub entity_type: StockType,

    #[serde(rename = "wfm_id")]
    #[serde(default = "String::default")]
    pub wfm_id: String,

    #[serde(rename = "wfm_url")]
    #[serde(default = "String::default")]
    pub wfm_url: String,

    #[serde(rename = "item_name")]
    #[serde(default = "String::default")]
    pub name: String,

    #[serde(rename = "item_unique_name")]
    #[serde(default = "String::default")]
    pub unique_name: String,

    #[serde(default)]
    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<i64>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "is_validated")]
    #[serde(default = "bool::default")]
    pub is_validated: bool,

    // Riven specific fields
    #[serde(rename = "mod_name")]
    pub mod_name: String,

    #[serde(rename = "mastery_rank")]
    #[serde(default = "i64::default")]
    pub mastery_rank: i64,

    #[serde(rename = "re_rolls")]
    #[serde(default = "i64::default")]
    pub re_rolls: i64,

    #[serde(rename = "polarity")]
    #[serde(default = "String::default")]
    pub polarity: String,

    #[serde(rename = "attributes")]
    #[serde(default)]
    pub attributes: Vec<RivenAttribute>,
}

impl std::ops::Deref for CreateStockEntity {
    type Target = StockType;

    fn deref(&self) -> &Self::Target {
        &self.entity_type
    }
}

impl Default for CreateStockEntity {
    fn default() -> Self {
        CreateStockEntity {
            raw: "".to_string(),
            bought: None,
            entity_type: StockType::Unknown,
            wfm_id: "".to_string(),
            wfm_url: "".to_string(),
            name: "".to_string(),
            unique_name: "".to_string(),
            tags: vec![],
            minimum_price: None,
            quantity: 1,
            is_hidden: false,
            sub_type: None,
            is_validated: false,
            mod_name: "".to_string(),
            mastery_rank: 0,
            re_rolls: 0,
            polarity: "".to_string(),
            attributes: vec![],
        }
    }
}

impl CreateStockEntity {
    pub fn new(raw: &str, bought: i64) -> Self {
        CreateStockEntity {
            raw: raw.to_string(),
            bought: Some(bought),
            ..Default::default()
        }
    }
    pub fn to_stock_item(&self) -> CreateStockItem {
        if !self.is_validated {
            CreateStockItem::new(
                self.raw.clone(),
                self.sub_type.clone(),
                self.bought,
                self.minimum_price,
                self.quantity,
                self.is_hidden,
            )
        } else {
            CreateStockItem::new_valid(
                self.wfm_id.clone(),
                self.wfm_url.clone(),
                self.name.clone(),
                self.unique_name.clone(),
                self.tags.clone(),
                self.sub_type.clone(),
                self.bought,
                self.minimum_price,
                self.quantity,
                self.is_hidden,
            )
        }
    }
    pub fn to_stock_riven(&self) -> CreateStockRiven {
        let mut rank: i64 = 0;
        if self.sub_type.is_some() {
            let sub_type = self.sub_type.as_ref().unwrap();
            if sub_type.rank.is_some() {
                rank = sub_type.rank.unwrap_or(0);
            }
        }
        if !self.is_validated {
            CreateStockRiven::new(
                self.raw.clone(),
                self.mod_name.clone(),
                self.mastery_rank,
                self.re_rolls,
                self.polarity.clone(),
                self.attributes.clone(),
                rank,
            )
        } else {
            CreateStockRiven::new_valid(
                self.wfm_id.clone(),
                self.wfm_id.clone(),
                self.wfm_url.clone(),
                self.name.clone(),
                self.unique_name.clone(),
                self.tags.join(","),
                self.mod_name.clone(),
                self.mastery_rank,
                self.re_rolls.clone(),
                self.polarity.clone(),
                self.attributes.clone(),
                self.minimum_price,
                self.bought,
                rank,
            )
        }
    }
    pub fn validate_entity(&mut self, cache: &CacheClient, by: &str) -> Result<(), AppError> {
        if self.entity_type == StockType::Unknown {
            return Err(AppError::new(
                "ValidateStockEntity",
                eyre::eyre!("Invalid entity type: {}", self.entity_type.as_str()),
            ));
        }
        if self.entity_type == StockType::Riven {
            let mut c_riven = self.to_stock_riven();
            let riven = cache.riven().validate_create_riven(&mut c_riven, by)?;
            self.wfm_id = riven.wfm_id.clone();
            self.wfm_url = riven.wfm_url.clone();
            self.name = riven.weapon_name.clone();
            self.unique_name = riven.weapon_unique_name.clone();
            self.tags = vec![riven.weapon_type.clone()];
            self.is_validated = true;
        } else if self.entity_type == StockType::Item {
            let mut c_item = self.to_stock_item();
            let item = cache
                .tradable_items()
                .validate_create_item(&mut c_item, by)?;
            self.wfm_id = item.wfm_id.clone();
            self.wfm_url = item.wfm_url.clone();
            self.name = item.item_name.clone();
            self.unique_name = item.item_unique_name.clone();
            self.tags = item.tags.clone();
            self.is_validated = true;
        } else {
            return Err(AppError::new(
                "ValidateStockEntity",
                eyre::eyre!("Invalid entity type: {}", self.entity_type.as_str()),
            ));
        }
        Ok(())
    }
    pub fn to_transaction(
        &self,
        user_name: &str,
        transaction_type: transaction::transaction::TransactionType,
    ) -> Result<transaction::transaction::Model, AppError> {
        if !self.is_validated {
            return Err(AppError::new(
                "CreateTransaction",
                eyre::eyre!("Entity is not validated"),
            ));
        }

        match self.entity_type {
            StockType::Item => {
                let item = self.to_stock_item();
                let transaction = item.to_stock().to_transaction(
                    user_name,
                    self.tags.clone(),
                    self.quantity,
                    self.bought.unwrap_or(0),
                    transaction_type,
                );
                Ok(transaction)
            }
            StockType::Riven => {
                let riven = self.to_stock_riven();
                let transaction = riven.to_stock().to_transaction(
                    user_name,
                    self.bought.unwrap_or(0),
                    transaction_type,
                );
                Ok(transaction)
            }
            _ => Err(AppError::new(
                "CreateTransaction",
                eyre::eyre!("Invalid entity type: {}", self.entity_type.as_str()),
            )),
        }
    }
    pub fn get_name(&self) -> Result<String, AppError> {
        if !self.is_validated {
            return Err(AppError::new(
                "GetEntityName",
                eyre::eyre!("Entity is not validated"),
            ));
        }

        match self.entity_type {
            StockType::Item => {     
                let name = match self.sub_type.as_ref() {
                    Some(sub_type) => {
                        format!("{} ({})", self.name.clone(), sub_type.shot_display())
                    }
                    None => self.name.clone(),                    
                };           
                Ok(name)
            }
            StockType::Riven => {
                Ok(self.name.clone() +" "+ &self.mod_name.clone())
            }
            _ => Err(AppError::new(
                "CreateTransaction",
                eyre::eyre!("Invalid entity type: {}", self.entity_type.as_str()),
            )),
        }
    }

}
