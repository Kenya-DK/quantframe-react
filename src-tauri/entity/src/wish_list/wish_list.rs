//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    enums::stock_status::StockStatus, price_history::PriceHistoryVec, sub_type::SubType,
    transaction,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wish_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub wfm_id: String,
    pub wfm_url: String,
    pub item_name: String,
    pub item_unique_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<i64>,
    #[sea_orm(column_type = "Text")]
    pub price_history: PriceHistoryVec,
    pub status: StockStatus,
    #[sea_orm(updated_at)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(created_at)]
    pub created_at: DateTimeUtc,

    #[sea_orm(ignore)]
    #[serde(rename = "is_dirty", default)]
    pub is_dirty: bool,

    #[sea_orm(ignore)]
    #[serde(rename = "locked", default)]
    pub locked: bool,

    #[sea_orm(ignore)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes")]
    pub changes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(
        wfm_id: String,
        wfm_url: String,
        item_name: String,
        item_unique_name: String,
        sub_type: Option<SubType>,
        maximum_price: Option<i64>,
        quantity: i64,
    ) -> Self {
        Self {
            id: Default::default(),
            wfm_id,
            wfm_url,
            item_name,
            item_unique_name,
            sub_type,
            quantity,
            list_price: None,
            maximum_price,
            status: StockStatus::Pending,
            price_history: PriceHistoryVec(vec![]),
            updated_at: Default::default(),
            created_at: Default::default(),
            is_dirty: true,
            locked: false,
            changes: None,
        }
    }
    pub fn to_transaction(
        &self,
        user_name: &str,
        tags: Vec<String>,
        quantity: i64,
        price: i64,
        transaction_type: transaction::transaction::TransactionType,
    ) -> transaction::transaction::Model {
        transaction::transaction::Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            transaction::transaction::TransactionItemType::Item,
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            tags,
            transaction_type,
            quantity,
            user_name.to_string(),
            price,
            None,
        )
    }
    fn set_if_changed<T: PartialEq>(current: &mut T, new_value: T, is_dirty: &mut bool) -> bool {
        if *current != new_value {
            *current = new_value;
            *is_dirty = true;
            return true;
        }
        false
    }

    pub fn set_list_price(&mut self, list_price: Option<i64>) {
        if self.locked {
            return;
        }
        if Self::set_if_changed(&mut self.list_price, list_price, &mut self.is_dirty) {
            self.changes = Some("list_price".to_string());
        }
    }

    pub fn set_status(&mut self, status: StockStatus) {
        if self.locked {
            return;
        }
        if Self::set_if_changed(&mut self.status, status, &mut self.is_dirty) {
            self.changes = Some("status".to_string());
        }
    }
}
