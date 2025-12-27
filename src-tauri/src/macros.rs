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

#[macro_export]
macro_rules! add_metric {
    ($key:expr, $value:expr) => {{
        use crate::utils::modules::states;
        let app = states::app_state().expect("App state should be available");
        app.qf_client.analytics().add_metric($key, $value);
    }};
}

#[macro_export]
macro_rules! play_sound {
    ($file_name:expr, $volume:expr) => {{
        use crate::emit_event;
        emit_event!(
            "play_sound",
            serde_json::json!({"file_name": $file_name, "volume": $volume}),
            "PlaySound"
        );
    }};
}
