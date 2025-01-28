use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

use crate::{
    qf_client::{client::QFClient, types::user::User},
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger, states,
        },
    },
};

#[derive(Clone, Debug)]
pub struct RivenModule {
    pub client: QFClient,
    component: String,
}

impl RivenModule {
    pub fn new(client: QFClient) -> Self {
        RivenModule {
            client,
            component: "Riven".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
}
