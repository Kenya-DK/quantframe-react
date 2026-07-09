use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::play_sound;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemNotify {
    pub enabled: bool,
    pub title: String,
    pub content: String,
    pub sound_file: String,
    pub volume: f32,
}
impl SystemNotify {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
        sound_file: impl Into<String>,
        volume: f32,
    ) -> Self {
        Self {
            enabled: true,
            title: title.into(),
            content: content.into(),
            sound_file: sound_file.into(),
            volume,
        }
    }
    pub fn send(&self, variables: &HashMap<String, String>) {
        let mut title = self.title.clone();
        let mut content = self.content.clone();
        for (k, v) in variables.iter() {
            title = title.replace(&format!("{}", k), v);
            content = content.replace(&format!("{}", k), v);
        }
        if !self.sound_file.is_empty() && self.sound_file != "none" {
            play_sound!(self.sound_file.clone(), self.volume);
        }
        #[cfg(target_os = "windows")]
        {
            use crate::send_system_notification;
            send_system_notification!(&title, &content, None, None);
        }
    }
}
