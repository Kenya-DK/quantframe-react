use sea_orm::{ActiveValue, Set};
use serde::{Deserialize, Serialize};
use utils::Properties;

use crate::{dto::*, enums::*, stock_riven::*};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateStockRiven {
    pub id: i64,

    #[serde(default)]
    pub bought: FieldChange<i64>,

    #[serde(default)]
    pub list_price: FieldChange<i64>,

    #[serde(default)]
    pub re_rolls: FieldChange<i64>,

    #[serde(default)]
    pub mastery_rank: FieldChange<i64>,

    #[serde(default)]
    pub status: FieldChange<StockStatus>,

    #[serde(default)]
    pub is_hidden: FieldChange<bool>,

    #[serde(default)]
    pub filter: FieldChange<MatchRivenStruct>,

    #[serde(default)]
    pub price_history: FieldChange<Vec<PriceHistory>>,

    #[serde(default)]
    pub grade: FieldChange<RivenGrade>,

    #[serde(default, flatten)]
    pub properties: FieldChange<Properties>,
}

impl UpdateStockRiven {
    pub fn apply_to(self, mut item: stock_riven::ActiveModel) -> stock_riven::ActiveModel {
        use FieldChange::*;
        match self.bought {
            Value(v) => item.bought = Set(v),
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
        match self.filter {
            Value(v) => item.filter = Set(v),
            _ => {}
        }
        match self.mastery_rank {
            Value(v) => item.mastery_rank = Set(v),
            _ => {}
        }
        match self.re_rolls {
            Value(v) => item.re_rolls = Set(v),
            _ => {}
        }
        match self.price_history {
            Value(v) => item.price_history = Set(PriceHistoryVec(v)),
            _ => {}
        }
        match self.properties {
            Value(mut v) => {
                v.keep_property_values(ALLOWED_PROPERTIES_FIELDS);
                v.nullify_zeroed_properties(ALLOWED_PROPERTIES_FIELDS);

                let properties = match item.properties {
                    ActiveValue::Set(mut existing) | ActiveValue::Unchanged(mut existing) => {
                        existing.merge_properties(v.properties, true, true);
                        existing
                    }
                    _ => v,
                };
                item.properties = Set(properties);
            }
            _ => {}
        }

        item
    }
    pub fn new(id: i64) -> Self {
        UpdateStockRiven {
            id,
            bought: FieldChange::Ignore,
            list_price: FieldChange::Ignore,
            is_hidden: FieldChange::Ignore,
            filter: FieldChange::Ignore,
            status: FieldChange::Ignore,
            mastery_rank: FieldChange::Ignore,
            re_rolls: FieldChange::Ignore,
            price_history: FieldChange::Ignore,
            grade: FieldChange::Ignore,
            properties: FieldChange::Ignore,
        }
    }

    pub fn with_bought(mut self, bought: i64) -> Self {
        self.bought = FieldChange::Value(bought);
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
    pub fn with_filter(mut self, filter: Option<MatchRivenStruct>) -> Self {
        self.filter = match filter {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }
    pub fn with_price_history(mut self, price_history: Option<Vec<PriceHistory>>) -> Self {
        self.price_history = match price_history {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }
    pub fn with_grade(mut self, grade: RivenGrade) -> Self {
        self.grade = FieldChange::Value(grade);
        self
    }
    pub fn with_properties(mut self, properties: Properties) -> Self {
        self.properties = FieldChange::Value(properties);
        self
    }
}
