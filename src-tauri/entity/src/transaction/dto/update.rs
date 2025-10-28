use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{enums::*, transaction::*};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransaction {
    pub id: i64,
    pub price: FieldChange<i64>,
    pub quantity: FieldChange<i64>,
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

        item
    }
    pub fn new(id: i64) -> Self {
        UpdateTransaction {
            id,
            price: FieldChange::Ignore,
            quantity: FieldChange::Ignore,
        }
    }
}
