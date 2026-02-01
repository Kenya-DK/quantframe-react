use chrono::{DateTime, Utc};
use entity::{dto::*, enums::*};

use serde::{Deserialize, Serialize};
use utils::{filters_by, sorting::SortDirection};

use crate::log_parser::Purchase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchasePaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,

    #[serde(default)]
    pub from_date: FieldChange<DateTime<Utc>>,

    #[serde(default)]
    pub to_date: FieldChange<DateTime<Utc>>,

    #[serde(default)]
    pub sort_by: FieldChange<String>,

    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
}
impl PurchasePaginationQueryDto {
    pub fn apply_query(&self, items: &Vec<Purchase>) -> Vec<Purchase> {
        let mut items = filters_by(&items, |o| {
            match &self.from_date {
                FieldChange::Value(q) => {
                    if o.date <= *q {
                        return false;
                    }
                }
                _ => {}
            }
            match &self.to_date {
                FieldChange::Value(q) => {
                    if o.date > *q {
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
                    "shop_id" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.shop_id.cmp(&b.shop_id),
                        SortDirection::Desc => b.shop_id.cmp(&a.shop_id),
                    }),
                    "date" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.date.cmp(&b.date),
                        SortDirection::Desc => b.date.cmp(&a.date),
                    }),
                    "price" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.price.cmp(&b.price),
                        SortDirection::Desc => b.price.cmp(&a.price),
                    }),
                    "items_received" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.items_received.len().cmp(&b.items_received.len()),
                        SortDirection::Desc => b.items_received.len().cmp(&a.items_received.len()),
                    }),
                    _ => {}
                }
            }
            _ => {}
        }
        items
    }
}
