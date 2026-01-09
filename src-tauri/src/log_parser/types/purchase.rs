use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseItem {
    pub name: String,
    pub quantity: i64,
}
impl PurchaseItem {
    pub fn new(name: String, quantity: i64) -> Self {
        Self { name, quantity }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub shop_id: String,
    pub date: DateTime<Utc>,
    pub price: i64,
    pub items_received: Vec<PurchaseItem>,
}