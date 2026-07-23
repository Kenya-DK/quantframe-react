use std::sync::Arc;
use std::sync::{Mutex, OnceLock};

use crate::app::{Settings, User};
use crate::http_server::HttpServer;
use qf_api::Client as QFClient;
use wf_market::client::Authenticated as WFAuthenticated;
use wf_market::types::websocket::WsClient;
use wf_market::Client as WFClient;

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

#[derive(Clone)]
pub struct AppState {
    pub user: User,
    pub settings: Settings,
    pub wfm_client: WFClient<WFAuthenticated>,
    pub qf_client: QFClient,
    pub is_development: bool,
    pub is_pre_release: bool,
    pub use_temp_db: bool,
    pub wfm_socket: Option<WsClient>,
    pub wfm_chat_socket: Option<WsClient>,
    pub http_server: Arc<HttpServer>,
}
