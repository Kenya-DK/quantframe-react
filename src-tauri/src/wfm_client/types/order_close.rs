use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct OrderClose {
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "last_update")]
    pub last_update: String,
    #[serde(rename = "quantity")]
    pub quantity: i64,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "order_type")]
    pub order_type: String,
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "platinum")]
    pub platinum: i64,
    #[serde(rename = "creation_date")]
    pub creation_date: String,
    #[serde(rename = "item")]
    pub item: String,
}
