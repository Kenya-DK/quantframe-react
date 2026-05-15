use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheAttribute {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "wfmId")]
    pub wfm_id: String,

    #[serde(rename = "wfmUrl")]
    pub wfm_url: String,

    #[serde(rename = "exclusiveTo", skip_serializing_if = "Option::is_none")]
    pub exclusive_to: Option<Vec<String>>,

    #[serde(rename = "formattedValue")]
    pub formatted_value: String,

    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "highlightedLabel")]
    pub highlighted_label: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(rename = "positiveIsNegative", skip_serializing_if = "Option::is_none")]
    pub positive_is_negative: Option<bool>,

    #[serde(rename = "positiveOnly", skip_serializing_if = "Option::is_none")]
    pub positive_only: Option<bool>,

    #[serde(rename = "negativeOnly", skip_serializing_if = "Option::is_none")]
    pub negative_only: Option<bool>,

    #[serde(rename = "prefix")]
    pub prefix: String,

    #[serde(rename = "suffix")]
    pub suffix: String,
}
