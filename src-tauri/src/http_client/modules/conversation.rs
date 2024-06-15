use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};



use tauri::{Manager, State};



use crate::{
    http_client::types::conversation::Conversation, notification::client::NotifyClient, settings::SettingsState, wfm_client::client::WFMClient, APP
};

#[post("/new_conversation")]
pub async fn new_conversation(input: web::Json<Conversation>) -> impl Responder {
    let app_handle = APP.get().expect("failed to get app handle");

    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let notify = notify_state.lock().expect("failed to lock notify state");
    let settings_state: State<Arc<Mutex<SettingsState>>> = app_handle.state();
    let settings = settings_state.lock().expect("failed to lock settings state");

    let content = settings.notifications.on_new_conversation.content.replace("<PLAYER_NAME>", input.user_name.as_str());

    // Send a notification to the system
    if settings.notifications.on_new_conversation.system_notify || settings.notifications.on_new_conversation.discord_notify{
        let info = settings.notifications.on_new_conversation.clone();
        if settings.notifications.on_new_conversation.system_notify {
            notify.system().send_notification(&info.title, &content, None, None);
        }
        if settings.notifications.on_new_conversation.discord_notify && info.webhook.clone().unwrap_or("".to_string()) != "" {
            notify.discord().send_notification(info.webhook.unwrap(), info.title, content, info.user_ids);
        }
    }
    HttpResponse::Ok().body(serde_json::to_string(&input).unwrap())
}
