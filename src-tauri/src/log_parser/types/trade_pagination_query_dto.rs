use chrono::{DateTime, Utc};
use entity::{dto::*, enums::*};

use serde::{Deserialize, Serialize};
use utils::{filters_by, sorting::SortDirection};

use crate::log_parser::PlayerTrade;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,

    #[serde(default)]
    pub from_date: FieldChange<DateTime<Utc>>,

    #[serde(default)]
    pub to_date: FieldChange<DateTime<Utc>>,

    #[serde(default)]
    pub year: FieldChange<i32>,

    #[serde(default)]
    pub sort_by: FieldChange<String>,

    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
}

impl TradePaginationQueryDto {
    pub fn apply_query(&self, items: &Vec<PlayerTrade>) -> Vec<PlayerTrade> {
        let mut items = filters_by(&items, |o| {
            match &self.from_date {
                FieldChange::Value(q) => {
                    if o.trade_time <= *q {
                        return false;
                    }
                }
                _ => {}
            }
            match &self.to_date {
                FieldChange::Value(q) => {
                    if o.trade_time > *q {
                        return false;
                    }
                }
                _ => {}
            }
            true
        });

        match &self.sort_by {
            FieldChange::Value(sort_by) => {
                let dir = match &self.sort_direction {
                    FieldChange::Value(dir) => dir,
                    _ => &SortDirection::Asc,
                };
                // Only allow sorting by known columns for safety
                match sort_by.as_str() {
                    "trade_time" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.trade_time.cmp(&b.trade_time),
                        SortDirection::Desc => b.trade_time.cmp(&a.trade_time),
                    }),
                    "platinum" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.platinum.cmp(&b.platinum),
                        SortDirection::Desc => b.platinum.cmp(&a.platinum),
                    }),
                    "credits" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.credits.cmp(&b.credits),
                        SortDirection::Desc => b.credits.cmp(&a.credits),
                    }),
                    "trade_type" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.trade_type.cmp(&b.trade_type),
                        SortDirection::Desc => b.trade_type.cmp(&a.trade_type),
                    }),
                    _ => {}
                }
            }
            _ => {}
        }
        items
    }
}
