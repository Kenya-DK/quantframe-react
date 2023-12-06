use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{error::AppError, helper, settings::SettingsState, logger, handler::MonitorHandler};
use eyre::eyre;
use serde_json::json;

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

#[derive(Clone,Debug)]
pub struct OnNewConversationEvent {
    wf_ee_path: PathBuf,
    settings: Arc<Mutex<SettingsState>>,
    helper: Arc<Mutex<MonitorHandler>>,
}

impl OnNewConversationEvent {
    pub fn new(settings: Arc<Mutex<SettingsState>>,helper: Arc<Mutex<MonitorHandler>>, wf_ee_path: PathBuf) -> Self {
        Self {
            settings,
            helper,
            wf_ee_path,
        }
    }

    pub fn check(&self, _: usize, input: &str) -> Result<(bool), AppError> {
        let settings = self.settings.lock()?.clone().whisper_scraper.on_new_conversation;
        let helper = self.helper.lock()?;


        if !settings.discord.enable && !settings.system.enable {
            return Ok(false);
        }
        let (found, captures) = crate::wf_ee_log_parser::events::helper::match_pattern(
            input,
            Events::Conversation.as_str_list(),
        )
        .map_err(|e| AppError::new("OnNewConversationEvent", eyre!(e)))?;
        if found {
            let username = captures.get(0).unwrap().clone().unwrap();

            // If system notification is enabled, show it
            if settings.system.enable {
                helper.show_notification(
                    settings.system.title.as_str(),
                    &settings.system.content.replace("<PLAYER_NAME>", username.as_str()),
                    Some("assets/icons/icon.png"),
                    Some("Default"),
                );
            }
            // If discord webhook is enabled, send it
            if settings.discord.enable {
                crate::helper::send_message_to_discord(
                    settings.discord.webhook.unwrap_or("".to_string()),
                    settings.discord.title,
                    settings.discord.content.replace("<PLAYER_NAME>", username.as_str()),
                    settings.discord.user_ids.clone(),
                );
            }            
        }
        Ok(found)
    }
}
