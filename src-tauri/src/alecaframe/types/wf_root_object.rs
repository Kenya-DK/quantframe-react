use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarframeRootObject {
    #[serde(rename = "PremiumCredits", default)]
    pub platinum: i64,

    #[serde(rename = "RegularCredits", default)]
    pub credits: i64,
}
impl Default for WarframeRootObject {
    fn default() -> Self {
        Self {
            platinum: 0,
            credits: 0,
        }
    }
}
