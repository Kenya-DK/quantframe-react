use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

use crate::{enums::stock_status::StockStatus, price_history::{PriceHistory, PriceHistoryVec}, sub_type::SubType};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_riven")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub wfm_weapon_id: String,
    pub wfm_weapon_url: String,
    pub wfm_order_id: Option<String>,
    pub weapon_name: String,
    pub weapon_type: String,
    pub weapon_unique_name: String,
    pub sub_type: Option<SubType>,
    pub mod_name: String,
    pub attributes: RivenAttributeVec,
    pub mastery_rank: i64,
    pub re_rolls: i64,
    pub polarity: String,
    pub bought: i64,
    pub minimum_price: Option<i64>,
    pub list_price: Option<i64>,
    pub filter: MatchRivenStruct,
    pub is_hidden: bool,
    pub comment:String,
    pub status: StockStatus,
    #[sea_orm(column_type = "Text")]
    pub price_history: PriceHistoryVec,
    #[sea_orm(updated_at)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(created_at)]
    pub created_at: DateTimeUtc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RivenAttribute {
    pub positive: bool,
    pub value: f64,
    pub url_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct RivenAttributeVec(pub Vec<RivenAttribute>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct MatchRivenStruct {
    pub enabled: Option<bool>,
    pub rank: Option<MinMaxStruct>,
    pub mastery_rank: Option<MinMaxStruct>,
    pub re_rolls: Option<MinMaxStruct>,
    pub polarity: Option<String>,
    pub similarity: Option<f64>,
    pub required_negative: Option<bool>,
    pub attributes: Option<Vec<MatchRivenAttributeStruct>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchRivenAttributeStruct {
    pub url_name: String,
    pub is_negative: bool,
    pub is_required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinMaxStruct {
    pub min: i64,
    pub max: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
