use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

use crate::{enums::stock_status::StockStatus, price_history::{PriceHistory, PriceHistoryVec}, stock_riven::{MatchRivenStruct, RivenAttributeVec}, sub_type::SubType};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_riven")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub order_id: Option<String>,
    pub weapon_id: String,
    pub weapon_url: String,
    pub weapon_name: String,
    pub weapon_type: String,
    pub mod_name: String,
    pub rank: i32,
    pub attributes: RivenAttributeVec,
    pub mastery_rank: i32,
    pub re_rolls: i32,
    pub polarity: String,
    pub price: f64,
    pub minium_price: Option<i32>,
    pub listed_price: Option<i32>,
    pub price_history: PriceHistoryVec,
    pub private: bool,
    pub status: String,
    pub comment: Option<String>,
    pub created: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
