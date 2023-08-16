use std::sync::Mutex;

use once_cell::sync::Lazy;
use serde_json::{json, Value};
use tauri::Window;

pub static WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));

pub fn send_message_to_window(event: &str, data: Option<Value>) {
    let window = WINDOW.lock().unwrap();
    if let Some(window) = &*window {
      let rep=  window.emit("message", json!({ "event": event, "data": data }));
      match rep {
          Ok(_) => {}
          Err(e) => {
              println!("Error while sending message to window {:?}", e);
          }
      }
    }
}
