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
        let settings = self.settings.lock()?.clone().whisper_scraper;
        let helper = self.helper.lock()?;
        if !settings.enable {
            return Ok(false);
        }
        let (found, captures) = crate::wf_ee_log_parser::events::helper::match_pattern(
            input,
            Events::Conversation.as_str_list(),
        )
        .map_err(|e| AppError::new("OnNewConversationEvent", eyre!(e)))?;
        if found {
            let username = captures.get(0).unwrap().clone().unwrap();
            // &app.config().tauri.bundle.identifier
            helper.show_notification(
                "New Conversation",
                &format!("You have whisper(s) from {}", username.as_str()),
                Some("assets/icons/icon.png"),
                Some("Default"),
            );
            logger::info_con("WhisperScraper", &format!("ReceivedMessage: {} received at {}", username.as_str(), chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
            crate::helper::send_message_to_window(
                "WhisperScraper:ReceivedMessage",
                Some(json!({ "name": username })),
            );
            if settings.webhook != "" {
                crate::helper::send_message_to_discord(
                    settings.webhook.clone(),
                    format!("You have whisper(s) from {}", username.as_str()),
                    settings.ping_on_notif,
                );
            }
            
        }
        Ok(found)
    }
}
