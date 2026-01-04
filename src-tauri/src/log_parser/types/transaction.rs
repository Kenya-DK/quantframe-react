use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sku: String,
    pub price: i64,
    pub currency: String,
    pub vendor: String,
    pub date: DateTime<Utc>,
    pub account: String,
}
