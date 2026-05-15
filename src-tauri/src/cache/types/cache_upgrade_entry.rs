use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheUpgradeEntry {
    #[serde(rename = "prefixTag", default)]
    pub prefix: String,

    #[serde(rename = "suffixTag", default)]
    pub suffix: String,

    #[serde(rename = "wfmUrl", default)]
    pub wfm_url: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "label", default)]
    pub label: String,

    #[serde(rename = "highlightedLabel", default)]
    pub highlighted_label: String,

    #[serde(rename = "formattedValue", default)]
    pub formatted_value: String,

    #[serde(rename = "value")]
    pub value: f64,
}
