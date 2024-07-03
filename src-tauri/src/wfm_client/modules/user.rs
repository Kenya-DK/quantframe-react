use crate::{
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
    wfm_client::{
        client::WFMClient,
        types::user_profile::UserProfile,
    },
};

use eyre::eyre;
#[derive(Clone, Debug)]
pub struct UserModule {
    pub client: WFMClient,
    pub debug_id: String,
    component: String,
}

impl UserModule {
    pub fn new(client: WFMClient) -> Self {
        UserModule {
            client,
            debug_id: "wfm_client_user".to_string(),
            component: "Users".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }

    pub async fn user_profile(&self, user_name: &str) -> Result<UserProfile, AppError> {
        let url: String;
        if user_name == "" {
            url = "profile".to_string();
        } else {
            url = format!("profile/{}", user_name);
        }

        match self
            .client
            .get::<UserProfile>(&url, Some("profile"))
            .await
        {
            Ok(ApiResult::Success(user, _)) => {
                return Ok(user);
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Login"),
                    e,
                    eyre!("There was an error fetching user profile"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        };
    }
}
