use std::sync::{Arc, Mutex};

use eyre::eyre;

use crate::{
    helper,
    log_parser::client::LogParser,
    qf_client::client::QFClient,
    utils::modules::{error::AppError, logger},
};

#[derive(Clone, Debug)]
pub struct OnConversationEvent {
    pub client: LogParser,
    component: String,
    regex: Vec<String>,
}

impl OnConversationEvent {
    pub fn new(client: LogParser) -> Self {
        OnConversationEvent {
            client,
            component: "OnConversationEvent".to_string(),
            regex: vec![r"Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F(?<name>.+) to index.+".to_string()],
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    pub fn process_line(&self, line: &str, _pos: u64) -> Result<bool, AppError> {
        let component = self.get_component("ProcessLine");
        let settings = self.client.settings.lock().unwrap();
        let notify = self.client.notify.lock().unwrap();

        if !line.contains("ChatRedux::AddTab: Adding tab with channel name") {
            return Ok(false);
        }

        let (found, captures) = helper::match_pattern(line, self.regex.clone())
            .map_err(|e| AppError::new("OnNewConversationEvent", eyre!(e)))?;
        if found {
            let username = captures.get(0).unwrap().clone().unwrap();
            let content = settings
                .notifications
                .on_new_conversation
                .content
                .replace("<PLAYER_NAME>", username.as_str());

            logger::info_con(
                &self.get_component(&component),
                &format!("New conversation with {}", username),
            );
            helper::add_metric("EE_NewConversation", "");
            // Send a notification to the system
            if settings.notifications.on_new_conversation.system_notify
                || settings.notifications.on_new_conversation.discord_notify
            {
                let info = settings.notifications.on_new_conversation.clone();
                if settings.notifications.on_new_conversation.system_notify {
                    notify
                        .system()
                        .send_notification(&info.title, &content, None, None);
                }
                if settings.notifications.on_new_conversation.discord_notify
                    && info.webhook.clone().unwrap_or("".to_string()) != ""
                {
                    notify.discord().send_notification(
                        &info.webhook.unwrap(),
                        &info.title,
                        &content,
                        info.user_ids,
                    );
                }
            }
        }
        Ok(false)
    }
}
