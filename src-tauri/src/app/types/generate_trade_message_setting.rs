use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveTemplateSetting {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub suffix: String,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub group_by_key: String,
    #[serde(default, rename = "displaySettings")]
    pub display_settings: HashMap<String, DisplaySettings>,
}
impl Default for SaveTemplateSetting {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            prefix: "WTS ".to_string(),
            suffix: " :heart:".to_string(),
            template: "[<link>] <type><variant><rank><amber_stars><cyan_stars><price> ".to_string(),
            group_by_key: "".to_string(),
            display_settings: HashMap::from([
                (
                    "rank".to_string(),
                    DisplaySettings {
                        prefix: Some("R ".to_string()),
                        suffix: Some("".to_string()),
                    },
                ),
                (
                    "price".to_string(),
                    DisplaySettings {
                        prefix: Some("".to_string()),
                        suffix: Some("p".to_string()),
                    },
                ),
            ]),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTradeMessageSetting {
    #[serde(default)]
    pub templates: Vec<SaveTemplateSetting>,
}
impl GenerateTradeMessageSetting {
    pub fn new() -> Self {
        Self {
            templates: vec![SaveTemplateSetting::default()],
        }
    }
}
impl Default for GenerateTradeMessageSetting {
    fn default() -> Self {
        GenerateTradeMessageSetting::new()
    }
}
