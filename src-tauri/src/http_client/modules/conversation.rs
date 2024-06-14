use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};



use tauri::{Manager, State};



use crate::{
    wfm_client::client::WFMClient,
    http_client::types::conversation::Conversation,
    notification::client::NotifyClient,
    APP,
};

#[post("/new_conversation")]
pub async fn new_conversation(input: web::Json<Conversation>) -> impl Responder {
    let app_handle = APP.get().expect("failed to get app handle");
    let wfm_state: State<Arc<Mutex<WFMClient>>> = app_handle.state();
    let wfm = wfm_state.lock().expect("failed to lock notify state");

    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let _notify = notify_state.lock().expect("failed to lock notify state");



    // Look up the user on the Warframe Market API
    let _user = match wfm.user().user_profile(&input.user_name).await {
        Ok(user) => {
            Some(user)
        }
        Err(_e) => {
            None
        }
    };
    println!("User: {:?}", _user);
    HttpResponse::Ok().body(serde_json::to_string(&input).unwrap())
}
