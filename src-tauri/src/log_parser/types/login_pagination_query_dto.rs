use chrono::{DateTime, Utc};
use entity::{dto::*, enums::*};

use serde::{Deserialize, Serialize};
use utils::filters_by;

use crate::log_parser::Login;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPaginationQueryDto {
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
impl LoginPaginationQueryDto {
    pub fn apply_query(&self, items: &Vec<Login>) -> Vec<Login> {
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
                    "date" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.date.cmp(&b.date),
                        SortDirection::Desc => b.date.cmp(&a.date),
                    }),
                    "ip" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.ip.cmp(&b.ip),
                        SortDirection::Desc => b.ip.cmp(&a.ip),
                    }),
                    "client_type" => items.sort_by(|a, b| match dir {
                        SortDirection::Asc => a.client_type.cmp(&b.client_type),
                        SortDirection::Desc => b.client_type.cmp(&a.client_type),
                    }),
                    _ => {}
                }
            }
            _ => {}
        }
        items
    }
}
