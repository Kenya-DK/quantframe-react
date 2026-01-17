use chrono::{DateTime, Utc};
use entity::{dto::*, enums::FieldChange};
use serde::{Deserialize, Serialize};
use utils::filters_by;

use crate::log_parser::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPaginationQueryDto {
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
impl TransactionPaginationQueryDto {
    pub fn apply_query(&self, items: &Vec<Transaction>) -> Vec<Transaction> {
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
                    "sku" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.sku.cmp(&b.sku),
                        SortDirection::Desc => b.sku.cmp(&a.sku),
                    }),
                    "price" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a
                            .price
                            .partial_cmp(&b.price)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        SortDirection::Desc => b
                            .price
                            .partial_cmp(&a.price)
                            .unwrap_or(std::cmp::Ordering::Equal),
                    }),
                    "currency" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.currency.cmp(&b.currency),
                        SortDirection::Desc => b.currency.cmp(&a.currency),
                    }),
                    "vendor" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.vendor.cmp(&b.vendor),
                        SortDirection::Desc => b.vendor.cmp(&a.vendor),
                    }),
                    "date" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.date.cmp(&b.date),
                        SortDirection::Desc => b.date.cmp(&a.date),
                    }),
                    "account" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.account.cmp(&b.account),
                        SortDirection::Desc => b.account.cmp(&a.account),
                    }),
                    _ => {}
                }
            }
            _ => {}
        }
        items
    }
}
