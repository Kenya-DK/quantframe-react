use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseError {
    pub message: String,
    #[serde(default)]
    pub error: String,
}
impl ResponseError {
    pub fn new(message: String, error: String) -> Self {
        ResponseError { message, error }
    }
}
impl Default for ResponseError {
    fn default() -> Self {
        ResponseError {
            message: "N/A".to_string(),
            error: "N/A".to_string(),
        }
    }
}
impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Start with API version
        write!(f, "API Error: {}", self.message)?;
        // Collect error values into a map
        if self.error != "N/A" && !self.error.is_empty() {
            write!(f, " (Error: {})", self.error)?;
        }

        Ok(())
    }
}
