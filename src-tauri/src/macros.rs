/// Macro to emit events with automatic logging
#[macro_export]
macro_rules! emit_event {
    ($event_name:expr, $payload:expr, $log_context:expr) => {{
        let app = APP.get().expect("App not initialized");
        match app.emit($event_name, $payload) {
            Ok(_) => {
                info(
                    &format!("Notification:GuiModel:{}", $log_context),
                    &format!("Event: {}", $event_name),
                    LoggerOptions::default(),
                );
            }
            Err(e) => {
                error(
                    &format!("Notification:GuiModel:{}", $log_context),
                    &format!("Event error: {}", e),
                    LoggerOptions::default(),
                );
            }
        }
    }};
}
#[macro_export]
macro_rules! emit_error {
    ($err:expr) => {{
        use crate::notification::enums::UIEvent;
        use crate::utils::modules::states;
        use serde_json::json;
        let notify = states::notify_client().expect("Failed to get notification client state");
        notify.gui().send_event(UIEvent::OnError, Some(json!($err)));
    }};
}

#[macro_export]
macro_rules! emit_startup {
    ($i18n_key:expr, $Option:expr) => {{
        use crate::notification::enums::UIEvent;
        use crate::utils::modules::states;
        use serde_json::json;
        let notify = states::notify_client().expect("Failed to get notification client state");
        notify.gui().send_event(UIEvent::OnStartingUp, Some(json!({"i18n_key": $i18n_key, "values": $Option})));
    }};
}
#[macro_export]
macro_rules! emit_update_user {
    ($user:expr) => {{
        use crate::notification::enums::*;
        use crate::utils::modules::states;
        use serde_json::json;
        let notify = states::notify_client().expect("Failed to get notification client state");
        notify.gui().send_event_update(
            UIEvent::UpdateUser,
            UIOperationEvent::CreateOrUpdate,
            Some(json!($user)),
        );
    }};
}
