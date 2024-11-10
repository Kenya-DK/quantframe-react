use eyre::eyre;
use serde_json::json;

use crate::{
    utils::{
        enums::{
            log_level::LogLevel,
            ui_events::{UIEvent, UIOperationEvent},
        },
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
    pub active_chat: String,
    pub chats: Vec<ChatData>,
    component: String,
}

impl ChatModule {
    pub fn new(client: WFMClient) -> Self {
        ChatModule {
            client,
            debug_id: "wfm_client_chat".to_string(),
            active_chat: "".to_string(),
            chats: vec![],
            component: "Chats".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_chat_module(self.clone());
    }
    pub async fn get_chats(&mut self) -> Result<Vec<ChatData>, AppError> {
        self.client.auth().is_logged_in()?;
        match self
            .client
            .get::<Vec<ChatData>>("im/chats", Some("chats"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.chats = payload.clone();
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetChats"),
                    format!("{} was fetched.", payload.len()).as_str(),
                    None,
                );
                self.update_state();
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

    pub async fn get_chat_messages(&self, id: String) -> Result<Vec<ChatMessage>, AppError> {
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

    pub async fn delete(&mut self, id: String) -> Result<String, AppError> {
        let url = format!("im/chats/{}", id);
        self.client.auth().is_logged_in()?;
        match self.client.delete(&url, Some("chat_id")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.chats.retain(|chat| chat.id != id);
                self.update_state();
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
    pub async fn receive_message(&mut self, msg: ChatMessage) -> Result<(), AppError> {
        let mut auth = self.client.auth.lock()?;
        let notify = self.client.notify.lock()?;

        let mut chat_payload = json!({
            "id": msg.chat_id,
            "messages": vec![msg.clone()],
            "last_update": msg.send_date,
        });
        if self.active_chat != msg.chat_id {
            auth.unread_messages += 1;
            notify.gui().send_event_update(
                UIEvent::UpdateUser,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(auth.clone())),
            );
            chat_payload["unread_count"] = json!(1);
        } else if self.active_chat == msg.chat_id {
            notify
                .gui()
                .send_event(UIEvent::ReceiveMessage, Some(json!(msg.clone())));
            chat_payload["unread_count"] = json!(0);
        }
        notify.gui().send_event_update(
            UIEvent::UpdateChats,
            UIOperationEvent::CreateOrUpdate,
            Some(chat_payload),
        );
        Ok(())
    }

    pub async fn set_active_chat(
        &mut self,
        id: Option<String>,
        unread: i64,
    ) -> Result<(), AppError> {
        let mut auth = self.client.auth.lock()?;

        let notify = self.client.notify.lock()?;
        self.active_chat = id.clone().unwrap_or("".to_string());
        self.update_state();
        if let Some(id) = id {
            auth.unread_messages = auth.unread_messages - unread;
            notify.gui().send_event_update(
                UIEvent::UpdateUser,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(auth.clone())),
            );
            notify.gui().send_event_update(
                UIEvent::UpdateChats,
                UIOperationEvent::CreateOrUpdate,
                Some(json!({ "id": id, "unread_count": 0 })),
            );
        }
        Ok(())
    }
}
