use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
    pub date: DateTime<Utc>,
    pub ip: Option<String>,
    pub client_type: Option<String>,
}
