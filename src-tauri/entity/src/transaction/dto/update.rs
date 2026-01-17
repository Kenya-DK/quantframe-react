use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{enums::*, transaction::*};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransaction {
    pub id: i64,
    pub price: FieldChange<i64>,
    pub quantity: FieldChange<i64>,
    pub created_at: FieldChange<String>,
    pub user_name: FieldChange<String>,
    pub properties: FieldChange<Value>,
}

impl UpdateTransaction {
    pub fn apply_to(self, mut item: transaction::ActiveModel) -> transaction::ActiveModel {
        use FieldChange::*;
        match self.price {
            Value(v) => item.price = Set(v),
            _ => {}
        }
        match self.quantity {
            Value(v) => item.quantity = Set(v),
            _ => {}
        }
        match self.user_name {
            Value(v) => item.user_name = Set(v),
            _ => {}
        }
        match self.created_at {
            Value(v) => item.created_at = Set(v.parse().unwrap()),
            _ => {}
        }
        match self.properties {
            Value(v) => item.properties = Set(Some(v)),
            Null => item.properties = Set(None),
            _ => {}
        }

        item
    }
    pub fn new(id: i64) -> Self {
        UpdateTransaction {
            id,
            price: FieldChange::Ignore,
            quantity: FieldChange::Ignore,
            user_name: FieldChange::Ignore,
            created_at: FieldChange::Ignore,
            properties: FieldChange::Ignore,
        }
    }
}
