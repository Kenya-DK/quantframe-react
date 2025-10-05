use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessageType {
    Single(String),
    Multiple(Vec<String>),
}
impl MessageType {
    pub fn is_empty(&self) -> bool {
        match self {
            MessageType::Single(msg) => msg.is_empty(),
            MessageType::Multiple(msgs) => msgs.is_empty() || msgs.iter().all(|m| m.is_empty()),
        }
    }
    pub fn contains(&self, substring: &str) -> bool {
        match self {
            MessageType::Single(msg) => msg.contains(substring),
            MessageType::Multiple(msgs) => msgs.iter().any(|m| m.contains(substring)),
        }
    }
}
impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Single(msg) => write!(f, "{}", msg),
            MessageType::Multiple(msgs) => write!(f, "{}", msgs.join(", ")),
        }
    }
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Single("N/A".to_string())
    }
}

impl From<String> for MessageType {
    fn from(msg: String) -> Self {
        MessageType::Single(msg)
    }
}

impl From<Vec<String>> for MessageType {
    fn from(msgs: Vec<String>) -> Self {
        MessageType::Multiple(msgs)
    }
}

impl From<&str> for MessageType {
    fn from(msg: &str) -> Self {
        MessageType::Single(msg.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseError {
    pub message: MessageType,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub banned_reason: String,
    #[serde(default)]
    pub banned_until: String,
}
impl ResponseError {
    pub fn new<T: Into<MessageType>>(message: T, error: String) -> Self {
        ResponseError {
            message: message.into(),
            error,
            banned_reason: String::new(),
            banned_until: String::new(),
        }
    }

    pub fn new_multiple(messages: Vec<String>, error: String) -> Self {
        ResponseError {
            message: MessageType::Multiple(messages),
            error,
            banned_reason: String::new(),
            banned_until: String::new(),
        }
    }
}
impl Default for ResponseError {
    fn default() -> Self {
        ResponseError {
            message: MessageType::default(),
            error: "N/A".to_string(),
            banned_reason: "N/A".to_string(),
            banned_until: "N/A".to_string(),
        }
    }
}
impl ResponseError {
    pub fn set_ban_info(&mut self, reason: String, until: String) {
        self.banned_reason = reason;
        self.banned_until = until;
    }
}
impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {}", self.message)?;

        // Collect error values into a map
        if self.error != "N/A" && !self.error.is_empty() {
            write!(f, " (Error: {})", self.error)?;
        }

        if self.banned_reason != "N/A" && !self.banned_reason.is_empty() {
            write!(
                f,
                " (Banned: {} until {})",
                self.banned_reason, self.banned_until
            )?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseBanError {
    #[serde(default)]
    pub banned_reason: String,
    #[serde(default)]
    pub banned_until: String,
}
