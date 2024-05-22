use sea_orm::entity::{self, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    enums::stock_status::StockStatus,
    price_history::PriceHistoryVec,
    sub_type::SubType, transaction,
};

use super::{attribute::RivenAttributeVec, match_riven::MatchRivenStruct};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_riven")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub wfm_weapon_id: String,
    pub wfm_weapon_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_order_id: Option<String>,
    pub weapon_name: String,
    pub weapon_type: String,
    pub weapon_unique_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
    pub mod_name: String,
    pub attributes: RivenAttributeVec,
    pub mastery_rank: i64,
    pub re_rolls: i64,
    pub polarity: String,
    pub bought: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<i64>,
    pub filter: MatchRivenStruct,
    pub is_hidden: bool,
    pub comment: String,
    pub status: StockStatus,
    #[sea_orm(column_type = "Text")]
    pub price_history: PriceHistoryVec,
    #[sea_orm(updated_at)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(created_at)]
    pub created_at: DateTimeUtc,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(
        wfm_weapon_id: String,
        wfm_weapon_url: String,
        wfm_order_id: Option<String>,
        weapon_name: String,
        weapon_type: String,
        weapon_unique_name: String,
        rank: i64,
        mod_name: String,
        attributes: RivenAttributeVec,
        mastery_rank: i64,
        re_rolls: i64,
        polarity: String,
        bought: i64,
        minimum_price: Option<i64>,
        is_hidden: bool,
        comment: String,
    ) -> Self {
        Self {
            id: Default::default(),
            wfm_weapon_id,
            wfm_weapon_url,
            wfm_order_id,
            weapon_name,
            weapon_type,
            weapon_unique_name,
            sub_type: Some(SubType::new(Some(rank), None, None, None)),
            mod_name,
            attributes,
            mastery_rank,
            re_rolls,
            polarity,
            bought,
            minimum_price,
            list_price: None,
            filter: MatchRivenStruct::new(),
            is_hidden,
            comment,
            status: StockStatus::Pending,
            price_history: PriceHistoryVec(vec![]),
            updated_at:Default::default(),
            created_at:Default::default(),
        }
    }
    pub fn to_transaction(&self,user_name:&str, price: i64, transaction_type: transaction::TransactionType) -> transaction::Model {
        transaction::Model::new(
            self.wfm_weapon_id.clone(),
            self.wfm_weapon_url.clone(),
            self.weapon_name.clone(),
            transaction::TransactionItemType::Riven,
            self.weapon_unique_name.clone(),
            self.sub_type.clone(),
            vec![self.weapon_type.clone()],
            transaction_type,
            1,
            user_name.to_string(),
            price,
            Some(json!({
             "mod_name": self.mod_name,
             "mastery_rank": self.mastery_rank,
             "re_rolls": self.re_rolls,
             "polarity": self.polarity,
             "attributes": self.attributes,
            })),
        )
    }
}