use serde::{Deserialize, Serialize};

use crate::app::NotificationSetting;

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
                false,
                true,
                "From: <PLAYER_NAME>",
                "New Conversation",
                None,
                None,
            ),
            on_wfm_chat_message: NotificationSetting::new(
                false,
                true,
                "From: <WFM_MESSAGE>",
                "New WFM Message",
                None,
                None,
            ),
            on_new_trade: NotificationSetting::new(
                false,
                true,
                "From: <PLAYER_NAME>\nOffered: <OF_COUNT> Received: <RE_COUNT> Plat: <TOTAL_PLAT>",
                "Item <TR_TYPE>",
                None,
                None,
            ),
        }
    }
}
