use std::collections::HashMap;

use serde_json::json;
use utils::{info, Error, LineEntry, LineHandler, LoggerOptions};

use crate::{add_metric, utils::modules::states};

#[derive(Clone, Debug)]
pub struct OnConversationEvent {}

impl OnConversationEvent {
    pub fn new() -> Self {
        OnConversationEvent {}
    }
}

impl LineHandler for OnConversationEvent {
    fn process_line(&mut self, entry: &LineEntry) -> Result<(bool, bool), Error> {
        if entry
            .line
            .contains("ChatRedux::AddTab: Adding tab with channel name")
        {
            // Extract channel name from the line
            if let Some(start_pos) = entry.line.find("channel name: ") {
                let parsedLineContent = &entry.line[start_pos + 14..]; // Skip "channel name: "

                if let Some(end_pos) = parsedLineContent.find(" to index") {
                    let mut player_name = parsedLineContent[..end_pos].to_string();

                    // Check if channel name starts with "F"
                    if !player_name.starts_with('F') {
                        return Ok((false, false));
                    }

                    // Handle multi-byte UTF-8 characters at the end
                    if let Some(last_char) = player_name.chars().last() {
                        if last_char.len_utf8() != 1 {
                            // Remove the last character if it's multi-byte
                            player_name.pop();
                        }
                    }

                    // Remove the first character (equivalent to str2.Substring(1))
                    if !player_name.is_empty() {
                        player_name = player_name.chars().skip(1).collect();
                    }
                    add_metric!("on_conversation_event", "new_conversation");
                    notify(&player_name);
                }
            }
        }
        Ok((false, false)) // no match → process normally
    }
}

fn notify(player_name: &str) {
    let settings = states::get_settings().expect("Failed to get settings");
    info(
        "OnConversationEvent",
        format!("OnConversationEvent: New conversation from {}", player_name,),
        &LoggerOptions::default(),
    );
    let mut variables = HashMap::new();
    variables.insert("<PLAYER_NAME>".to_string(), player_name.to_string());
    settings
        .notifications
        .on_new_conversation
        .send(&variables, Some(json!({ "playerName": player_name })));
}
