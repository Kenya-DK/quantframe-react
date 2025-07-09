use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum ApiVersion {
    V1,
    V2,
}

impl ApiVersion {
    pub fn api_url(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "https://api.warframe.market/v1",
            ApiVersion::V2 => "https://api.warframe.market/v2",
        }
    }
    pub fn websocket_url(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "wss://warframe.market/socket?platform=pc",
            ApiVersion::V2 => "wss://warframe.market/socket-v2",
        }
    }
}
impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::V2
    }
}
