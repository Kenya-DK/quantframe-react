use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{error::AppError, handler::MonitorHandler, settings::SettingsState};
use eyre::eyre;

enum Events {
    Conversation,
}
impl Events {
    fn as_str_list(&self) -> Vec<String> {
        match self {
            Events::Conversation => vec![
                r"Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F(?<name>.+) to index.+".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct OnNewConversationEvent {
    settings: Arc<Mutex<SettingsState>>,
    helper: Arc<Mutex<MonitorHandler>>,
}

impl OnNewConversationEvent {
    pub fn new(
        settings: Arc<Mutex<SettingsState>>,
        helper: Arc<Mutex<MonitorHandler>>,
        _: PathBuf,
    ) -> Self {
        Self { settings, helper }
    }

    pub fn check(&self, _: usize, input: &str) -> Result<bool, AppError> {
        let settings = self
            .settings
            .lock()?
            .clone()
            .notifications
            .on_new_conversation;
        let helper = self.helper.lock()?;

        if !settings.system_notify && !settings.discord_notify {
            return Ok(false);
        }
        let (found, captures) = crate::wf_ee_log_parser::events::helper::match_pattern(
            input,
            Events::Conversation.as_str_list(),
        )
        .map_err(|e| AppError::new("OnNewConversationEvent", eyre!(e)))?;
        if found {
            let username = captures.get(0).unwrap().clone().unwrap();
            let content = settings.content.replace("<PLAYER_NAME>", username.as_str());
            // If system notification is enabled, show it
            if settings.system_notify {
                helper.show_notification(
                    settings.title.as_str(),
                    &content,
                    Some("assets/icons/icon.png"),
                    Some("Default"),
                );
            }
            // If discord webhook is enabled, send it
            if settings.discord_notify && settings.webhook.is_some() {
                crate::helper::send_message_to_discord(
                    settings.webhook.unwrap_or("".to_string()),
                    settings.title,
                    content,
                    settings.user_ids.clone(),
                );
            }
        }
        Ok(found)
    }
}
