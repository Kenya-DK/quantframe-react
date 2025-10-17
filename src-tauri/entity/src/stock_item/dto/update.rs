use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{dto::*, enums::*, stock_item::*};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateStockItem {
    pub id: i64,

    #[serde(default)]
    pub owned: FieldChange<i64>,

    #[serde(default)]
    pub bought: FieldChange<i64>,

    #[serde(default)]
    pub minimum_price: FieldChange<i64>,

    #[serde(default)]
    pub minimum_profit: FieldChange<i64>,

    #[serde(default)]
    pub minimum_sma: FieldChange<i64>,

    #[serde(default)]
    pub list_price: FieldChange<i64>,

    #[serde(default)]
    pub status: FieldChange<StockStatus>,

    #[serde(default)]
    pub is_hidden: FieldChange<bool>,

    #[serde(default)]
    pub price_history: FieldChange<Vec<PriceHistory>>,
}

impl UpdateStockItem {
    pub fn apply_to(self, mut item: stock_item::ActiveModel) -> stock_item::ActiveModel {
        use FieldChange::*;

        match self.owned {
            Value(v) => item.owned = Set(v),
            _ => {}
        }
        match self.bought {
            Value(v) => item.bought = Set(v),
            _ => {}
        }
        match self.minimum_price {
            Value(v) => {
                if v <= 0 {
                    item.minimum_price = Set(None)
                } else {
                    item.minimum_price = Set(Some(v))
                }
            }
            Null => item.minimum_price = Set(None),
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
        match self.minimum_profit {
            Value(v) => {
                if v <= 0 {
                    item.minimum_profit = Set(None)
                } else {
                    item.minimum_profit = Set(Some(v))
                }
            }
            Null => item.minimum_profit = Set(None),
            _ => {}
        }
        match self.minimum_sma {
            Value(v) => {
                if v <= 0 {
                    item.minimum_sma = Set(None)
                } else {
                    item.minimum_sma = Set(Some(v))
                }
            }
            Null => item.minimum_sma = Set(None),
            _ => {}
        }
        item
    }
    pub fn new(id: i64) -> Self {
        UpdateStockItem {
            id,
            owned: FieldChange::Ignore,
            bought: FieldChange::Ignore,
            minimum_price: FieldChange::Ignore,
            list_price: FieldChange::Ignore,
            is_hidden: FieldChange::Ignore,
            status: FieldChange::Ignore,
            minimum_profit: FieldChange::Ignore,
            minimum_sma: FieldChange::Ignore,
            price_history: FieldChange::Ignore,
        }
    }
    pub fn with_owned(mut self, owned: i64) -> Self {
        self.owned = FieldChange::Value(owned);
        self
    }

    pub fn with_bought(mut self, bought: i64) -> Self {
        self.bought = FieldChange::Value(bought);
        self
    }

    pub fn with_minimum_price(mut self, minimum_price: Option<i64>) -> Self {
        self.minimum_price = match minimum_price {
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
    pub fn with_price_history(mut self, price_history: Option<Vec<PriceHistory>>) -> Self {
        self.price_history = match price_history {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }
    pub fn with_minimum_profit(mut self, minimum_profit: Option<i64>) -> Self {
        self.minimum_profit = match minimum_profit {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }
    pub fn with_minimum_sma(mut self, minimum_sma: Option<i64>) -> Self {
        self.minimum_sma = match minimum_sma {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }
}
