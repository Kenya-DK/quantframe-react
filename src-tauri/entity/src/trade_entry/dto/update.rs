use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{dto::*, enums::*, trade_entry::*};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTradeEntry {
    pub id: i64,
    #[serde(default)]
    pub price: FieldChange<i64>,
    #[serde(default)]
    pub tags: FieldChange<Vec<String>>,
    #[serde(default)]
    pub sub_type: FieldChange<Option<SubType>>,
}

impl UpdateTradeEntry {
    pub fn apply_to(self, mut item: trade_entry::ActiveModel) -> trade_entry::ActiveModel {
        use FieldChange::*;

        match self.price {
            Value(v) => item.price = Set(v),
            _ => {}
        }
        match self.tags {
            Value(v) => item.tags = Set(v.join(",")),
            _ => {}
        }
        match self.sub_type {
            Value(v) => item.sub_type = Set(v),
            _ => {}
        }
        item
    }
    pub fn new(id: i64) -> Self {
        UpdateTradeEntry {
            id,
            price: FieldChange::Ignore,
            tags: FieldChange::Ignore,
            sub_type: FieldChange::Ignore,
        }
    }
}
