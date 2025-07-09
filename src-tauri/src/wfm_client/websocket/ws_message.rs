use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;

use crate::wfm_client::enums::ApiVersion;

#[derive(Debug, Deserialize, Clone)]
pub struct WsMessage {
    // Ignore the version
    #[serde(skip_serializing, skip_deserializing)]
    pub version: ApiVersion,
    #[serde(rename = "route", alias = "type")]
    pub route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}
impl WsMessage {
    pub fn new(route: &str, payload: Option<serde_json::Value>, version: ApiVersion) -> Self {
        WsMessage {
            version,
            route: route.to_string(),
            payload,
            id: Some(uuid::Uuid::new_v4().to_string()),
            ref_id: None,
        }
    }
    pub fn connect(version: ApiVersion) -> Self {
        WsMessage::new(
            "@internal|internal/connected",
            Some(json!({"status": "connected"})),
            version,
        )
        .with_id("INTERNAL")
    }
    pub fn disconnect(error: String, version: ApiVersion) -> Self {
        WsMessage::new(
            "@internal|internal/disconnected",
            Some(json!({"reason": error})),
            version,
        )
        .with_id("INTERNAL")
    }
    pub fn set_version(mut self, version: ApiVersion) -> Self {
        self.version = version;
        self
    }
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
}
impl Serialize for WsMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut state = serializer.serialize_map(None)?;

        let route_key = match self.version {
            ApiVersion::V1 => "type",
            _ => "route",
        };

        state.serialize_entry(route_key, &self.route)?;

        if let Some(ref payload) = self.payload {
            state.serialize_entry("payload", payload)?;
        }

        if let Some(ref id) = self.id {
            state.serialize_entry("id", id)?;
        }

        if let Some(ref ref_id) = self.ref_id {
            state.serialize_entry("refId", ref_id)?;
        }

        state.end()
    }
}
