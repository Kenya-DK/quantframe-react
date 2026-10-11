use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurveyOption {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurveyQuestion {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub question_type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "required", default)]
    pub required: Option<bool>,
    #[serde(rename = "options", default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SurveyOption>>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "max_length", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Survey {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "questions", default)]
    pub questions: Vec<SurveyQuestion>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "dismissible")]
    pub dismissible: bool,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurveyResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "survey_id")]
    pub survey_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "answers", default)]
    pub answers: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveSurvey {
    #[serde(rename = "survey", default)]
    pub survey: Option<Survey>,
}
