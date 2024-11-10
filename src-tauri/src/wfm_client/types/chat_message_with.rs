use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessageWith {
    #[serde(rename = "reputation")]
    pub reputation: f64,

    #[serde(rename = "locale")]
    pub locale: String,

    #[serde(rename = "avatar")]
    pub avatar: Option<String>,

    #[serde(rename = "last_seen")]
    pub last_seen: String,

    #[serde(rename = "ingame_name")]
    pub ingame_name: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "region")]
    pub region: String,
}
