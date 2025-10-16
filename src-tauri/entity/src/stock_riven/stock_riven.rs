use crate::{dto::*, enums::*, stock_riven::dto::*, transaction::Model as TransactionModel};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::{attribute::RivenAttributeVec, match_riven::MatchRivenStruct};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_riven")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub wfm_weapon_id: String,
    pub wfm_weapon_url: String,
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
    // Default UUID
    pub uuid: String,

    #[sea_orm(column_type = "Text")]
    pub price_history: PriceHistoryVec,
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
        wfm_weapon_id: String,
        wfm_weapon_url: String,
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
        let mut item = Self {
            id: Default::default(),
            wfm_weapon_id,
            wfm_weapon_url,
            weapon_name,
            weapon_type,
            weapon_unique_name,
            sub_type: Some(SubType::new(Some(rank), None, None, None, None)),
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
            updated_at: Default::default(),
            created_at: Default::default(),
            is_dirty: true,
            locked: false,
            changes: None,
            uuid: "".to_string(),
        };
        item.uuid = item.uuid().to_string();
        item
    }
    pub fn to_transaction(
        &self,
        user_name: impl Into<String>,
        price: i64,
        transaction_type: TransactionType,
    ) -> TransactionModel {
        TransactionModel::new(
            self.wfm_weapon_id.clone(),
            self.wfm_weapon_url.clone(),
            self.weapon_name.clone(),
            TransactionItemType::Riven,
            self.weapon_unique_name.clone(),
            self.sub_type.clone(),
            vec![self.weapon_type.clone()],
            transaction_type,
            1,
            user_name.into(),
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
    // Helper to set dirty flag when values are changed
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
    pub fn add_price_history(&mut self, price_history: PriceHistory) {
        let mut items = self.price_history.0.clone();
        let last_item = items.last().cloned();
        if last_item.is_none() || last_item.unwrap().price != price_history.price {
            // Limit to 5 elements
            if items.len() >= 5 {
                items.remove(0);
            }
            items.push(price_history);
            self.is_dirty = true;
            self.changes = Some("price_history".to_string());
            self.price_history = PriceHistoryVec(items);
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

    pub fn uuid(&self) -> Uuid {
        let mut input = String::new();

        input.push_str(&format!("type:{};", "0"));
        input.push_str(&format!("weapon:{};", self.wfm_weapon_url));

        input.push_str(&format!("mod_name:{};", self.mod_name.to_lowercase()));
        input.push_str(&format!("re_rolls:{};", self.re_rolls));
        input.push_str(&format!("mastery:{};", self.mastery_rank));
        if let Some(v) = &self.sub_type {
            input.push_str(&format!("mod_rank:{};", v.rank.unwrap_or(0)));
        }
        input.push_str(&format!("polarity:{};", self.polarity.to_lowercase()));

        let mut sorted_attrs = self.attributes.clone().0;
        sorted_attrs.sort_by_key(|a| a.url_name.clone());
        for a in sorted_attrs {
            input.push_str(&format!("attr:{}:{}:{};", a.url_name, a.positive, a.value));
        }
        Uuid::new_v5(&Uuid::NAMESPACE_OID, input.as_bytes())
    }
    pub fn to_update(&self) -> UpdateStockRiven {
        UpdateStockRiven {
            id: self.id,
            bought: FieldChange::Value(self.bought),
            minimum_price: self
                .minimum_price
                .map_or(FieldChange::Null, |v| FieldChange::Value(v)),
            list_price: self
                .list_price
                .map_or(FieldChange::Null, |v| FieldChange::Value(v)),
            is_hidden: FieldChange::Value(self.is_hidden),
            filter: FieldChange::Value(self.filter.clone()),
            status: FieldChange::Value(self.status.clone()),
            mastery_rank: FieldChange::Value(self.mastery_rank),
            re_rolls: FieldChange::Value(self.re_rolls),
            price_history: FieldChange::Value(self.price_history.0.clone()),
        }
    }
}
