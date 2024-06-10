use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    qf_client::{client::QFClient, types::user::User},
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger,
        },
    },
};

#[derive(Clone, Debug)]
pub struct AnalyticsModule {
    pub client: QFClient,
    component: String,
}

impl AnalyticsModule {
    pub fn new(client: QFClient) -> Self {
      AnalyticsModule {
            client,
            component: "Analytics".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
  }
