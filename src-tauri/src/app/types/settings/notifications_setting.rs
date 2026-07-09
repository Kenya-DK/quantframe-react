use super::*;
use crate::app::{DiscordNotify, NotificationSetting, SystemNotify, WebHookNotify};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationsSetting {
    pub custom_sounds: Vec<CustomSound>,
    pub on_new_conversation: NotificationSetting,
    pub on_wfm_chat_message: NotificationSetting,
    pub on_new_trade: NotificationSetting,
}

impl Default for NotificationsSetting {
    fn default() -> Self {
        NotificationsSetting {
            custom_sounds: vec![],
            on_new_conversation: NotificationSetting::new(
                DiscordNotify::new("<MENTION>\n```ansi\n\x1B[1;36m🗨️ New Conversation\n\n\x1B[1;33m👤 From Player:\x1B[0m \x1B[1;37m<PLAYER_NAME>\x1B[0m\n```", "", vec![]),
                SystemNotify::new("New Conversation", "From: <PLAYER_NAME>","cat_meow.mp3", 1.0),
                WebHookNotify::new("<WEBHOOK_URL>"),
            ),
            on_wfm_chat_message: NotificationSetting::new(
                DiscordNotify::new( "<MENTION>\n```ansi\n\x1B[1;36m🗨️ New Warframe Market Message\x1B[0m\n\n\x1B[1;32m📀 Chat Name:        \x1B[0m   \x1B[0;32m<CHAT_NAME>\x1B[0m\n\x1B[1;33m👤 From Player:   \x1B[0m \x1B[1;37m<FROM_USER>\x1B[0m\n\n\x1B[1;34m✨ Context\x1B[0m\n<WFM_MESSAGE>\n\n```", "", vec![]),
                SystemNotify::new("New Warframe Market Message", "Chat Name: <CHAT_NAME> | From: <FROM_USER> | Message: \n <WFM_MESSAGE>","cat_meow.mp3", 1.0),
                WebHookNotify::new("<WEBHOOK_URL>"),
            ),
            on_new_trade: NotificationSetting::new(
                DiscordNotify::new("<MENTION>\n```ansi\n\x1B[1;36m💱 Player Trade\x1B[0m\n\n\x1B[1;33m👤 Player:   \x1B[0m \x1B[1;37m<PLAYER_NAME>\x1B[0m\n\x1B[1;33m🕒 Time:   \x1B[0m   \x1B[0;32m<TIME>\x1B[0m\n\x1B[1;33m📂 Type:   \x1B[0m   \x1B[0;35m<TR_TYPE>\x1B[0m\n\x1B[1;33m💎 Platinum: \x1B[0m \x1B[1;37m<TOTAL_PLAT>\x1B[0m\n\n\x1B[1;34m📤 Offered Items\x1B[0m\n<OF_ITEMS>\n\n\x1B[1;32m📥 Received Items\x1B[0m\n<RE_ITEMS>\n\n```", "", vec![]),
                SystemNotify::new("Item <TR_TYPE>", "From: <PLAYER_NAME>\nOffered: <OF_COUNT> Received: <RE_COUNT> Plat: <TOTAL_PLAT>","cat_meow.mp3", 1.0),
                WebHookNotify::new("<WEBHOOK_URL>"),
            ),
        }
    }
}
