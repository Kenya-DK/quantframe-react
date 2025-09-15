/// Macro to emit events with automatic logging
#[macro_export]
macro_rules! emit_event {
    ($event_name:expr, $payload:expr, $log_context:expr) => {{
        use crate::APP;
        use ::utils::*;
        use tauri::Emitter; // Bring the trait with `emit` into scope
        let app = APP.get().expect("App not initialized");
        match app.emit($event_name, $payload) {
            Ok(_) => {
                info(
                    &format!("Emit:{}", $log_context),
                    &format!("Event: {}", $event_name),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => {
                error(
                    &format!("Notification:GuiModel:{}", $log_context),
                    &format!("Event error: {}", e),
                    &LoggerOptions::default(),
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! send_event {
    ($event:expr, $data:expr) => {{
        use serde_json::json;
        use crate::emit_event;
        emit_event!(
            "message",
            json!({ "event": $event.as_str(), "data": $data }),
            format!("SendEvent:{}", $event.as_str())
        );
    }};
}

#[macro_export]
macro_rules! send_event_update {
    ($event:expr, $operation:expr, $data:expr) => {{
        use crate::types::*;
        use serde_json::json;
        use crate::emit_event;
        emit_event!(
            "message_update",
            json!({ "event": $event.as_str(), "operation": $operation.as_str(), "data": $data }),
            format!("SendEventUpdate:{}", $event.as_str())
        );
    }};
}

#[macro_export]
macro_rules! emit_error {
    ($err:expr) => {{
        use crate::send_event;
        use crate::types::*;
        use serde_json::json;
        send_event!(UIEvent::OnError, Some(json!($err)));
    }};
}

#[macro_export]
macro_rules! emit_startup {
    ($i18n_key:expr, $Option:expr) => {{
        use crate::types::*;
        use crate::send_event;
        send_event!(UIEvent::OnStartingUp, Some(json!({"i18n_key": $i18n_key, "values": $Option})))        ;
    }};
}

#[macro_export]
macro_rules! emit_update_user {
    ($user:expr) => {{
        use crate::send_event_update;
        send_event_update!(
            UIEvent::UpdateUser,
            UIOperationEvent::CreateOrUpdate,
            Some(json!($user))
        );
    }};
}

#[macro_export]
macro_rules! notify_gui {
    ($i18n_key:expr, $color:expr, $notify_type:expr, $values:expr) => {{
        use crate::send_event;
        use crate::types::*;
        send_event!(
            UIEvent::OnNotify,
            Some(json!({"i18n_key": $i18n_key, "color": $color, "type": $notify_type, "values": $values}))
        );
    }};
}

#[macro_export]
macro_rules! send_system_notification {
    ($title:expr, $body:expr, $icon:expr, $sound:expr) => {{
        use crate::APP;
        use tauri_plugin_notification::NotificationExt;
        let app = APP.get().expect("App not initialized");
        app.notification()
            .builder()
            .title($title)
            .body($body)
            .icon($icon.unwrap_or("assets/icons/icon.png"))
            .sound($sound.unwrap_or("Ping"))
            .show()
            .expect("Failed to show notification");
    }};
}
// #[macro_export]
// macro_rules! send_discord_notification {
//     ($webhook:expr, $title:expr, $body:expr, $embeds:expr, $user_ids:expr) => {{
//         use reqwest::Client;
//         use serde_json::{json, Value};
//         use std::collections::HashMap;
//         use utils::*;
//         let mut content = String::new();
//         if let Some(user_ids) = $user_ids {
//             for user_id in user_ids {
//                 content.push_str(&format!("<@{}> ", user_id));
//             }
//         }
//         let payload = json!({
//             "content": content,
//             "username": "Quantframe",
//             "embeds": [{
//                 "title": $title,
//                 "description": $body,
//                 "color": 5814783,
//                 "fields": $embeds.unwrap_or(vec![]),
//                 "footer": {
//                     "text": "Quantframe",
//                     "icon_url": "https://quantframe.com/assets/icons/icon.png"
//                 }
//             }]
//         });
//         let webhook = $webhook;
//         tauri::async_runtime::spawn(async move {
//             let client = Client::new();
//             let res = client.post(webhook).json(&payload).send().await;
//             match res {
//                 Ok(response) => {
//                     if response.status().is_success() {
//                         info("DiscordNotification", "Notification sent successfully", &LoggerOptions::default());
//                     } else {
//                         info(
//                             "DiscordNotification",
//                             &format!("Failed to send notification: {}", response.status()),
//                             &LoggerOptions::default()
//                         );
//                     }
//                 }
//                 Err(e) => {
//                     info(
//                         "DiscordNotification",
//                         &format!("Error sending notification: {}", e),
//                         &LoggerOptions::default()
//                     );
//                 }
//             }
//         });
//     }};
// }
