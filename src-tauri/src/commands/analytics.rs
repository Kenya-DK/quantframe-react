use std::sync::Mutex;

use qf_api::enums::app_events::ApplicationEvent as EventType;
use serde_json::Value;
use std::collections::HashMap;
use utils::{get_location, Error};

use crate::app::AppState;

#[tauri::command]
pub fn track_event(
    key: String,
    value: HashMap<String, String>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let event_type: EventType =
        serde_json::from_value(Value::String(key.clone())).map_err(|_| {
            Error::new(
                "Command::TrackEvent",
                format!("Unknown event type: {}", key),
                get_location!(),
            )
        })?;
    let app = app.lock()?.clone();
    app.analytics.track_event(event_type, value);
    Ok(())
}
