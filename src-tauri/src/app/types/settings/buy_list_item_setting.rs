use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuyListItemSetting {
    #[serde(rename = "wfmId", alias = "wfm_id")]
    pub wfm_id: String,
    pub max_price: i64,
}
