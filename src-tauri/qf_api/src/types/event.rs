use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::enums::ApplicationEvent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEventDto {
    #[serde(rename = "event")]
    pub event: ApplicationEvent,

    #[serde(rename = "properties", default)]
    pub properties: HashMap<String, String>,
}

impl CreateEventDto {
    pub fn new(event: ApplicationEvent, properties: HashMap<String, String>) -> Self {
        Self { event, properties }
    }
    pub fn from_pairs<I, K, V>(event_type: ApplicationEvent, properties: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        Self::new(
            event_type,
            properties
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}
