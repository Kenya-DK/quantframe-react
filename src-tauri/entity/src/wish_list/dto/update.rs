use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{PriceHistory, PriceHistoryVec},
    enums::*,
    wish_list::wish_list,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateWishList {
    pub id: i64,

    #[serde(default)]
    pub quantity: FieldChange<i64>,

    #[serde(default)]
    pub maximum_price: FieldChange<i64>,

    #[serde(default)]
    pub minimum_price: FieldChange<i64>,

    #[serde(default)]
    pub list_price: FieldChange<i64>,

    #[serde(default)]
    pub status: FieldChange<StockStatus>,

    #[serde(default)]
    pub is_hidden: FieldChange<bool>,

    #[serde(default)]
    pub price_history: FieldChange<Vec<PriceHistory>>,
}

impl UpdateWishList {
    pub fn apply_to(self, mut item: wish_list::ActiveModel) -> wish_list::ActiveModel {
        use FieldChange::*;

        match self.quantity {
            Value(v) => item.quantity = Set(v),
            _ => {}
        }
        match self.minimum_price {
            Value(v) => item.minimum_price = Set(Some(v)),
            Null => item.minimum_price = Set(None),
            _ => {}
        }
        match self.maximum_price {
            Value(v) => item.maximum_price = Set(Some(v)),
            Null => item.maximum_price = Set(None),
            _ => {}
        }
        match self.list_price {
            Value(v) => item.list_price = Set(Some(v)),
            Null => item.list_price = Set(None),
            _ => {}
        }
        match self.is_hidden {
            Value(v) => item.is_hidden = Set(v),
            _ => {}
        }
        match self.status {
            Value(v) => item.status = Set(v),
            _ => {}
        }
        match self.price_history {
            Value(v) => item.price_history = Set(PriceHistoryVec(v)),
            _ => {}
        }

        item
    }
    pub fn new(id: i64) -> Self {
        UpdateWishList {
            id,
            quantity: FieldChange::Ignore,
            maximum_price: FieldChange::Ignore,
            minimum_price: FieldChange::Ignore,
            list_price: FieldChange::Ignore,
            is_hidden: FieldChange::Ignore,
            status: FieldChange::Ignore,
            price_history: FieldChange::Ignore,
        }
    }
    pub fn with_quantity(mut self, quantity: i64) -> Self {
        self.quantity = FieldChange::Value(quantity);
        self
    }

    pub fn with_maximum_price(mut self, maximum_price: Option<i64>) -> Self {
        self.maximum_price = match maximum_price {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }

    pub fn with_list_price(mut self, list_price: Option<i64>) -> Self {
        self.list_price = match list_price {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }

    pub fn with_is_hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = FieldChange::Value(is_hidden);
        self
    }

    pub fn with_status(mut self, status: StockStatus) -> Self {
        self.status = FieldChange::Value(status);
        self
    }
    pub fn with_price_history(mut self, price_history: Vec<PriceHistory>) -> Self {
        self.price_history = FieldChange::Value(price_history);
        self
    }
}
