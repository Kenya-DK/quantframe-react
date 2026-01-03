use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub shop_id: String,
    pub date: DateTime<Utc>,
    pub price: i64,
    pub items_received: Vec<(String, i64)>,
}
