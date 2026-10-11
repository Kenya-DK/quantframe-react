use std::sync::{Arc, Weak};

use reqwest::Method;

use crate::{
    client::Client,
    enums::{ApiResponse, ResponseFormat},
    errors::ApiError,
    types::*,
};

#[derive(Debug)]
pub struct SurveyRoute {
    client: Weak<Client>,
}

impl SurveyRoute {
    /** Creates a new `SurveyRoute` bound to the given client. */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }

    /** Returns the active survey for the current user, or `None` if there is none. */
    pub async fn get_active(&self) -> Result<Option<Survey>, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match client
            .as_ref()
            .call_api::<ActiveSurvey>(
                Method::GET,
                "/survey/active",
                None,
                None,
                ResponseFormat::Json,
            )
            .await
        {
            Ok((ApiResponse::Json(active), _, _)) => Ok(active.survey),
            Err(e) => Err(e),
            _ => Err(ApiError::Unknown("Unexpected response format".to_string())),
        }
    }

    /** Submits answers for a survey. `answers` is keyed by question id. */
    pub async fn submit(
        &self,
        survey_id: impl Into<String>,
        answers: serde_json::Value,
    ) -> Result<SurveyResponse, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let body = serde_json::json!({ "answers": answers });
        match client
            .as_ref()
            .call_api::<SurveyResponse>(
                Method::POST,
                &format!("/survey/{}/respond", survey_id.into()),
                Some(body),
                None,
                ResponseFormat::Json,
            )
            .await
        {
            Ok((ApiResponse::Json(response), _, _)) => Ok(response),
            Err(e) => Err(e),
            _ => Err(ApiError::Unknown("Unexpected response format".to_string())),
        }
    }

    /** Creates a new `SurveyRoute` from an existing one, sharing the client. */
    pub fn from_existing(_old: &SurveyRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
