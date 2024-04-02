use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "chat_id")]
    pub chat_id: String,

    #[serde(rename = "send_date")]
    pub send_date: String,

    #[serde(rename = "message_from")]
    pub message_from: String,

    #[serde(rename = "raw_message")]
    pub raw_message: Option<String>,
}
