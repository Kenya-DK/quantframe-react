use crate::{notification::client::NotifyClient, APP};
use tauri::Url;
use tauri_plugin_notification::{Attachment, NotificationExt};

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
        app.notification()
            .builder()
            .title(title)
            .body(body)
            .icon(icon.unwrap_or("assets/icons/icon.png"))
            .sound(sound)
            .attachment(Attachment::new(
                "AS".to_string(),
                Url::parse("icon://assets/icons/icon.png").unwrap(),
            ))
            .show()
            .expect("Failed to show notification");
    }
}
