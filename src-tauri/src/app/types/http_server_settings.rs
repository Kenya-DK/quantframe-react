use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpServerSettings {
    pub enable: bool,
    pub host: String,
    pub port: u16,
}
impl HttpServerSettings {
    pub fn uuid(&self) -> String {
        format!("E:{}:H{}:P{}", self.enable, self.host, self.port)
    }
}
impl Default for HttpServerSettings {
    fn default() -> Self {
        HttpServerSettings {
            enable: false,
            host: "127.0.0.1".into(),
            port: 4808,
        }
    }
}
