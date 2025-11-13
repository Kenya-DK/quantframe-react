use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatLink {
    #[serde(default)]
    pub prefix: String,

    pub link: String,

    #[serde(default)]
    pub suffix: String,
}

impl ChatLink {
    pub fn new(link: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            link: link.into(),
            suffix: suffix.into(),
        }
    }
}
