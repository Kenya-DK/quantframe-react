use entity::stock::riven::create::CreateStockRiven;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RivenPayload {
    #[serde(rename = "by")]
    pub by: String,

    #[serde(rename = "riven_data")]
    pub riven_data: CreateStockRiven,
}
