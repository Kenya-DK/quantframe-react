use serde::{Deserialize, Serialize};

use super::{chat_message::ChatMessage, chat_message_with::ChatMessageWith};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatData {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "chat_with")]
    pub chat_with: Vec<ChatMessageWith>,

    #[serde(rename = "unread_count")]
    pub unread_count: i64,

    #[serde(rename = "chat_name")]
    pub chat_name: String,

    #[serde(rename = "messages")]
    pub messages: Vec<ChatMessage>,

    #[serde(rename = "last_update")]
    pub last_update: String,
}
