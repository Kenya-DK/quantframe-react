use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{errors::ResponseError, utils::mask_sensitive_data};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestError {
    pub status_code: u16,
    pub method: String,
    pub url: String,
    pub payload: Option<serde_json::Value>,
    pub headers: HashMap<String, String>,
    pub content: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub error: ResponseError,
}

impl RequestError {
    pub fn new(method: String, url: String, payload: Option<serde_json::Value>) -> Self {
        RequestError {
            status_code: 0, // Default value, can be set later if needed
            method,
            url,
            payload,
            headers: HashMap::new(),         // Default empty headers
            content: String::new(),          // Default empty content
            error: ResponseError::default(), // Default no error
        }
    }
    pub fn set_status_code(&mut self, status_code: u16) {
        self.status_code = status_code;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }
    pub fn set_payload(&mut self, payload: Option<serde_json::Value>) {
        self.payload = payload;
    }
    pub fn set_error(&mut self, error: ResponseError) {
        self.error = error;
    }
    /**
     * Masks sensitive data in payload and headers.
     * This function iterates over the payload and headers, masking any sensitive data
     * that matches the provided properties.
     * It modifies the payload and headers in place.
     * # Arguments
     * * `properties` - A slice of strings representing the properties to mask.
     * # Example
     * ```
     * let mut error = RequestError::new(ApiVersion::V1, "GET".to_string(), "https://example.com".to_string(), None);
     * error.mask_sensitive_data(&["password", "token"]);
     * ```
     */
    pub fn mask_sensitive_data(&mut self, properties: &[&str]) {
        if let Some(obj) = self.payload.as_mut() {
            if obj.is_object() {
                mask_sensitive_data(obj.as_object_mut().unwrap(), properties);
            }
        }

        for (key, value) in self.headers.iter_mut() {
            let lower_key = key.to_lowercase();
            if properties
                .iter()
                .any(|&prop| lower_key.contains(prop.to_lowercase().as_str()))
            {
                *value = "*******".to_string();
            }
        }
    }

    /**
     * Returns a human-readable error sentence that masks sensitive data.
     * This provides a secure way to display error information to users.
     */
    pub fn error_sentence(&self) -> String {
        let mut parts = Vec::new();

        // Add HTTP method and URL
        parts.push(format!("{} {}", self.method, self.url));

        // Add status code if available
        if self.status_code != 0 {
            parts.push(format!("returned status {}", self.status_code));
        }

        // Add API Payload
        if let Some(ref payload) = self.payload {
            parts.push(format!(
                "Payload: {}",
                serde_json::to_string(payload).unwrap_or_else(|_| "Invalid JSON".to_string())
            ));
        }

        // Add WFM error if available
        if self.error.message != "N/A" {
            parts.push(format!("error: {}", self.error));
        }

        // Add content if available (but limit length for security)
        if !self.content.is_empty() {
            let content_preview = if self.content.len() > 100 {
                format!("{}...", &self.content[..100])
            } else {
                self.content.clone()
            };
            parts.push(format!("content: {}", content_preview));
        }

        parts.join(", ")
    }
}

impl Default for RequestError {
    fn default() -> Self {
        RequestError {
            status_code: 0,
            method: String::new(),
            url: String::new(),
            payload: None,
            headers: HashMap::new(),
            content: String::new(),
            error: ResponseError::default(),
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_sentence())
    }
}
