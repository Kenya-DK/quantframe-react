use serde::{Deserialize, Serialize};

use crate::app::{DiscordNotify, NotificationSetting, SystemNotify};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationsSetting {
    pub on_new_conversation: NotificationSetting,
    pub on_wfm_chat_message: NotificationSetting,
    pub on_new_trade: NotificationSetting,
}

impl Default for NotificationsSetting {
    fn default() -> Self {
        NotificationsSetting {
            on_new_conversation: NotificationSetting::new(
                DiscordNotify::new(r#"<MENTION>\n```ansi\n\u001b[1;36m🗨️ New Conversation\n\n\u001b[1;33m👤 From Player:\u001b[0m \u001b[1;37m<PLAYER_NAME>\u001b[0m\n```"#,"", vec![]),
                SystemNotify::new("New Conversation", "From: <PLAYER_NAME>"),
            ),
            on_wfm_chat_message: NotificationSetting::new(
                DiscordNotify::new( "From: <WFM_MESSAGE>", "", vec![]),
                SystemNotify::new("New WFM Message", "From: <WFM_MESSAGE>"),
            ),
            on_new_trade: NotificationSetting::new(
                DiscordNotify::new(r#"<MENTION>\n```ansi\n\u001b[1;36m💱 Player Trade\u001b[0m\n\n\u001b[1;33m👤 Player:   \u001b[0m \u001b[1;37m<PLAYER_NAME>\u001b[0m\n\u001b[1;33m🕒 Time:   \u001b[0m   \u001b[0;32m<TIME>\u001b[0m\n\u001b[1;33m📂 Type:   \u001b[0m   \u001b[0;35m<TR_TYPE>\u001b[0m\n\u001b[1;33m💎 Platinum: \u001b[0m \u001b[1;37m<TOTAL_PLAT>\u001b[0m\n\n\u001b[1;34m📤 Offered Items\u001b[0m\n<OF_ITEMS>\n\n\u001b[1;32m📥 Received Items\u001b[0m\n<RE_ITEMS>\n\n```"#, "", vec![]),
                SystemNotify::new("Item <TR_TYPE>", "From: <PLAYER_NAME>\nOffered: <OF_COUNT> Received: <RE_COUNT> Plat: <TOTAL_PLAT>"),
            ),
        }
    }
}
