use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTradeMessageSetting {}
impl GenerateTradeMessageSetting {}
impl Default for GenerateTradeMessageSetting {
    fn default() -> Self {
        GenerateTradeMessageSetting {}
    }
}
