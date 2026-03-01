use serde::{Deserialize, Serialize};
use utils::Properties;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvItemBase {
    #[serde(rename = "id", default)]
    pub id: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "unique_name", default)]
    pub unique_name: String,

    #[serde(rename = "wfm_url", default)]
    pub wfm_url: String,

    #[serde(rename = "quantity", default)]
    pub quantity: i64,

    #[serde(rename = "sub_type", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<entity::dto::SubType>,

    // Extra properties that may be needed for specific item types, stored as a JSON object
    pub properties: Properties,
}

impl WFInvItemBase {}

impl Default for WFInvItemBase {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            unique_name: String::new(),
            wfm_url: String::new(),
            quantity: 1,
            sub_type: None,
            properties: Properties::default(),
        }
    }
}
