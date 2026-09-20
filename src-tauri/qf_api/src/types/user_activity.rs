use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveUsersChartDto {
    #[serde(rename = "labels")]
    pub labels: Vec<String>,

    #[serde(rename = "registered_users_chart")]
    pub registered_users_chart: Vec<f64>,

    #[serde(rename = "total_users_chart")]
    pub total_users_chart: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserActivityDto {
    #[serde(rename = "labels")]
    pub labels: Vec<String>,

    #[serde(rename = "registered_users_chart")]
    pub registered_users_chart: Vec<f64>,

    #[serde(rename = "total_users_chart")]
    pub total_users_chart: Vec<f64>,
}
