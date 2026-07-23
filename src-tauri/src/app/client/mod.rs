mod app_state;
mod ws;

pub use app_state::AppState;
pub use ws::{get_active_chat_id, set_active_chat_id, setup_socket, update_user};
