use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionOwner {
    #[serde(rename = "ingame_name")]
    pub ingame_name: String,

    #[serde(rename = "last_seen")]
    pub last_seen: String,

    #[serde(rename = "reputation")]
    pub reputation: i64,

    #[serde(rename = "locale")]
    pub locale: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "avatar")]
    pub avatar: Option<String>,
}
