use chrono::{DateTime, Utc};
use entity::{dto::*, enums::*};

use serde::{Deserialize, Serialize};

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
