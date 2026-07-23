use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use serde_json::json;
use tauri::Manager;
use utils::{get_location, info, Error, LogLevel, LoggerOptions, OperationSet};
use wf_market::client::Authenticated as WFAuthenticated;
use wf_market::enums::ApiVersion;
use wf_market::types::websocket::{WsClient, WsMessage};
use wf_market::types::{Chat, ChatMessage, UserPrivate as WFUserPrivate};
use wf_market::Client as WFClient;

use crate::app::User;
use crate::utils::modules::states;
use crate::utils::ErrorFromExt;
use crate::{clear_error, emit_error, send_event, APP, HAS_STARTED};
use crate::emit_update_user;

pub static ACTIVE_CHAT_ID: OnceLock<Mutex<Option<String>>> = OnceLock::new();

pub fn set_active_chat_id(chat_id: Option<String>) {
    let active_chat_id = ACTIVE_CHAT_ID.get_or_init(|| Mutex::new(None));
    let mut guard = active_chat_id.lock().unwrap();
    *guard = chat_id;
}

pub fn get_active_chat_id() -> Option<String> {
    let active_chat_id = ACTIVE_CHAT_ID.get_or_init(|| Mutex::new(None));
    let guard = active_chat_id.lock().unwrap();
    guard.clone()
}

pub fn update_user(mut cu_user: User, user: &WFUserPrivate, qf_user: &qf_api::types::UserPrivate) -> User {
    cu_user.anonymous = false;
    cu_user.verification = user.verification;
    cu_user.wfm_banned = user.banned.unwrap_or(false);
    cu_user.wfm_banned_reason = user.ban_message.clone();
    cu_user.wfm_banned_until = user.ban_until.clone();
    cu_user.qf_banned = qf_user.banned;
    cu_user.qf_banned_reason = qf_user.banned_reason.clone();
    cu_user.qf_banned_until = qf_user.banned_until.clone();
    cu_user.patreon_tier = qf_user.patreon_tier.clone();
    cu_user.permissions = qf_user.permissions.clone();
    cu_user.wfm_id = user.id.to_string();
    cu_user.wfm_username = user.ingame_name.clone();
    cu_user.check_code = user.check_code.clone();
    cu_user.locale = user.locale.clone();
    cu_user.platform = user.platform.clone();
    cu_user.unread_messages = user.unread_messages as i64;
    cu_user.wfm_avatar = user.avatar.clone();
    cu_user
}

fn send_ws_state(key: impl Into<String>, data: &WsMessage) {
    let key = key.into();
    let mut current_error = match states::get_app_error().as_mut() {
        Some(err) => err.clone(),
        None => Error::new("WebSocket", "Connection state", get_location!())
            .set_log_level(LogLevel::Warning),
    };

    if current_error.component != "WebSocket" {
        return;
    }

    current_error
        .properties
        .merge_properties(data.payload.clone(), true, true);

    let mut operations = OperationSet::from(
        current_error
            .properties
            .get_property_value::<Vec<String>>("operations", vec![]),
    );

    let ws_type = key.split(':').next().unwrap_or("unknown").to_string();
    operations.remove_prefix(&ws_type);
    operations.add(&key);

    current_error
        .properties
        .set_property_value("operations", operations.operations.clone());

    if !operations.ends_with("Disconnected") {
        clear_error!();
        return;
    }
    current_error.log("websocket_info.log");
    emit_error!(current_error);
}

fn update_user_status(states: impl Into<String>) {
    let states = states.into();
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<crate::app::client::AppState>>();
    let mut guard = state.lock().expect("Failed to lock notification state");
    guard.user.wfm_status = states;
    guard.user.save().expect("Failed to save user status");
    emit_update_user!(json!({"wfm_status": guard.user.wfm_status}));
}

fn handle_new_message(wfm_client: &WFClient<WFAuthenticated>, msg: &WsMessage) {
    let binding = wfm_client.chat();
    let active_chat_id = get_active_chat_id().unwrap_or_default();
    let chat_message = match msg
        .get_payload_as::<ChatMessage>(msg.route.ends_with("MESSAGE_SENT").then_some("message"))
    {
        Ok(chat_msg) => chat_msg,
        Err(e) => {
            let err = Error::from_json(
                "ChatMessage:Received",
                &PathBuf::from("websocket"),
                msg.payload.as_ref().unwrap_or(&json!({})).to_string(),
                "Failed to parse chat message from websocket",
                e,
                get_location!(),
            );
            err.log("websocket_info.log");
            return;
        }
    };
    if chat_message.is_none() {
        return;
    }
    let chat_message = chat_message.unwrap();

    fn handle_notify(
        chat: &Chat,
        chat_message: &ChatMessage,
        active_chat_id: impl Into<String>,
        requirer_refresh: bool,
        un_read: u32,
    ) {
        let active_chat_id = active_chat_id.into();
        let state = states::app_state().expect("Failed to get settings");

        let from_user = match chat.find_user(&chat_message.message_from) {
            Some(c) => c.name.clone(),
            None => "".to_string(),
        };
        let mut chat_payload = json!(chat_message);
        chat_payload["requirer_refresh"] = json!(requirer_refresh);
        send_event!(crate::types::UIEvent::OnWfmChatMessage, chat_payload);

        if active_chat_id == chat.id || state.user.wfm_id == chat_message.message_from {
            return;
        }
        emit_update_user!(json!({ "unread_messages": un_read }));
        state.settings.notifications.on_wfm_chat_message.send(
            &HashMap::from([
                (
                    "<WFM_MESSAGE>".to_string(),
                    chat_message.raw_message.clone(),
                ),
                ("<CHAT_NAME>".to_string(), chat.chat_name.clone()),
                ("<FROM_USER>".to_string(), from_user.clone()),
            ]),
            Some(json!({
                "chatId": chat.id,
                "chatName": chat.chat_name,
                "fromUserId": chat_message.message_from,
                "fromUserName": from_user,
                "message": chat_message.raw_message,
            })),
        );
        info(
            "ChatMessage:Notify",
            &format!("New message in chat {} from {}", chat.chat_name, from_user),
            &LoggerOptions::default(),
        );
    }

    let chat = binding
        .cache_chats_mut()
        .handle_chat_message(&chat_message, &active_chat_id);
    if chat.is_none() {
        tauri::async_runtime::spawn(async move {
            match binding.get_chats().await {
                Ok(mut messages) => {
                    let chat = messages.get_by_id(&chat_message.chat_id, true);
                    if chat.is_some() {
                        handle_notify(
                            &chat.unwrap(),
                            &chat_message,
                            &active_chat_id,
                            true,
                            binding.cache_chats().total_unread_count(),
                        );
                    }
                }
                Err(e) => {
                    let err = Error::from_wfm(
                        "ChatMessage:FetchChat",
                        "Failed to fetch chats after receiving new message",
                        e,
                        get_location!(),
                    );
                    err.log("websocket_info.log");
                }
            }
        });
    } else {
        handle_notify(
            &chat.unwrap(),
            &chat_message,
            &active_chat_id,
            false,
            binding.cache_chats().total_unread_count(),
        );
    }
}

pub async fn setup_socket(
    wfm_client: WFClient<WFAuthenticated>,
) -> Result<(WsClient, WsClient), Error> {
    if wfm_client.get_user().is_err() {
        return Err(Error::new(
            "AppState:SetupSocket",
            "WFM client user is not authenticated, please login first.",
            get_location!(),
        ));
    }

    let ws_client = wfm_client
        .create_websocket(ApiVersion::V2)
        .set_log_unhandled(true)
        .register_callback("internal/connected", move |msg, _, _| {
            send_ws_state("Main:Connected", msg);
            Ok(())
        })
        .unwrap()
        .register_callback("internal/disconnected", move |msg, _, _| {
            send_ws_state("Main:Disconnected", msg);
            Ok(())
        })
        .unwrap()
        .register_callback("internal/reconnecting", move |msg, _, _| {
            send_ws_state("Main:Disconnected", msg);
            Ok(())
        })
        .unwrap()
        .register_callback("event/account/banned", move |msg, _, _| {
            let payload = msg.clone().payload.unwrap();
            emit_update_user!(json!({
                "wfm_banned": true,
                "wfm_banned_reason": payload["banMessage"].as_str().unwrap_or("").to_string(),
                "wfm_banned_until": payload["banUntil"].as_str().unwrap_or("").to_string()
            }));
            Ok(())
        })
        .unwrap()
        .register_callback("event/account/banLifted", move |_, _, _| {
            emit_update_user!(json!({"wfm_banned": false}));
            Ok(())
        })
        .unwrap()
        .register_callback("event/reports/online", move |_, _, _| Ok(()))
        .unwrap()
        .register_callback("cmd/status/set:ok", move |msg, _, _| {
            match msg.payload.as_ref() {
                Some(payload) => update_user_status(
                    payload["status"]
                        .as_str()
                        .unwrap_or("invisible")
                        .to_string(),
                ),

                None => {}
            }
            Ok(())
        })
        .unwrap()
        .register_callback("event/status/set", move |msg, _, _| {
            if !HAS_STARTED.get().cloned().unwrap_or(false) {
                return Ok(());
            }
            match msg.payload.as_ref() {
                Some(payload) => update_user_status(
                    payload["status"]
                        .as_str()
                        .unwrap_or("invisible")
                        .to_string(),
                ),

                None => {}
            }
            Ok(())
        })
        .unwrap()
        .build()
        .await
        .unwrap();
    let ws_client_chat = wfm_client
        .create_websocket(ApiVersion::V1)
        .register_callback("internal/connected", move |msg, _, _| {
            send_ws_state("Chat:Connected", msg);
            Ok(())
        })
        .unwrap()
        .register_callback("internal/disconnected", move |msg, _, _| {
            send_ws_state("Chat:Disconnected", msg);
            Ok(())
        })
        .unwrap()
        .register_callback("internal/reconnecting", move |msg, _, _| {
            send_ws_state("Chat:Disconnected", msg);
            Ok(())
        })
        .unwrap()
        .register_callback("chats/NEW_MESSAGE,chats/MESSAGE_SENT", move |msg, _, _| {
            if !HAS_STARTED.get().cloned().unwrap_or(false) {
                return Ok(());
            }
            handle_new_message(&wfm_client, msg);
            Ok(())
        })
        .unwrap()
        .build()
        .await
        .unwrap();
    Ok((ws_client, ws_client_chat))
}
