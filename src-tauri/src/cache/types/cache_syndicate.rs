use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSyndicateTitle {
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "minStanding")]
    pub min_standing: i64,
    #[serde(rename = "maxStanding")]
    pub max_standing: i64,
}

impl Default for CacheSyndicateTitle {
    fn default() -> Self {
        Self {
            level: 0,
            name: "Unknown".to_string(),
            min_standing: 0,
            max_standing: 0,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSyndicate {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "canSelect", default)]
    pub can_select: bool,

    #[serde(rename = "iconColour", default)]
    pub icon_colour: String,

    #[serde(rename = "backgroundColour", default)]
    pub background_colour: String,

    #[serde(rename = "titles", default)]
    pub titles: Vec<CacheSyndicateTitle>,
}

impl CacheSyndicate {
    // pub fn get_title_by_level(&self, level: i32) -> Option<CacheSyndicateTitle> {
    //     self.titles
    //         .iter()
    //         .find(|title| level == title.level)
    //         .cloned()
    // }
}
