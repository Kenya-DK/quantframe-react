use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvAffiliation {
    #[serde(rename = "Tag", default)]
    pub id: String,

    #[serde(rename = "Standing", default)]
    pub standing: i64,

    #[serde(rename = "Title", default)]
    pub rank: i64,
}
