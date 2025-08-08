use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseError {
    pub message: String,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub banned_reason: String,
    #[serde(default)]
    pub banned_until: String,
}
impl ResponseError {
    pub fn new(message: String, error: String) -> Self {
        ResponseError {
            message,
            error,
            banned_reason: String::new(),
            banned_until: String::new(),
        }
    }
}
impl Default for ResponseError {
    fn default() -> Self {
        ResponseError {
            message: "N/A".to_string(),
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
