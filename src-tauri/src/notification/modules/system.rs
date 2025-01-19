use crate::{notification::client::NotifyClient, APP};
use tauri::api::notification::Notification;

#[derive(Clone, Debug)]
pub struct SystemModule {}

impl SystemModule {
    pub fn new() -> Self {
        SystemModule {}
    }

    pub fn send_notification(
        &self,
        title: &str,
        body: &str,
        icon: Option<&str>,
        sound: Option<&str>,
    ) {
        let app = APP.get().expect("App not initialized");
        let sound = match sound {
            Some(s) => s,
            None => "Default",
        };
        let notification = Notification::new(&app.config().tauri.bundle.identifier)
            .title(title)
            .body(body)
            .icon(icon.unwrap_or("assets/icons/icon.png"))
            .sound(sound);
        notification.show().unwrap();
    }
}
