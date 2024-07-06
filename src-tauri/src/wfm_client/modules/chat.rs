use eyre::eyre;

use crate::{
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
    wfm_client::{
        client::WFMClient,
        types::{chat_data::ChatData, chat_message::ChatMessage},
    },
};
#[derive(Clone, Debug)]
pub struct ChatModule {
    pub client: WFMClient,
    pub debug_id: String,
    component: String,
}

impl ChatModule {
    pub fn new(client: WFMClient) -> Self {
        ChatModule {
            client,
            debug_id: "wfm_client_chat".to_string(),
            component: "Chats".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    pub async fn get_chats(&self) -> Result<Vec<ChatData>, AppError> {
        self.client.auth().is_logged_in()?;
        match self
            .client
            .get::<Vec<ChatData>>("im/chats", Some("chats"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetChats"),
                    format!("{} was fetched.", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetChats"),
                    error,
                    eyre!("There was an error fetching chats"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn get_chat(&self, id: String) -> Result<Vec<ChatMessage>, AppError> {
        let url = format!("im/chats/{}", id);
        self.client.auth().is_logged_in()?;
        match self
            .client
            .get::<Vec<ChatMessage>>(&url, Some("messages"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetChatById"),
                    format!("{} chat messages were fetched.", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetChatById"),
                    error,
                    eyre!("There was an error fetching chat messages for chat {}", id),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn delete(&self, id: String) -> Result<String, AppError> {
        let url = format!("im/chats/{}", id);
        self.client.auth().is_logged_in()?;
        match self.client.delete(&url, Some("chat_id")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Delete"),
                    format!("Chat {} was deleted.", id).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Delete"),
                    error,
                    eyre!("There was an error deleting chat {}", id),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}
