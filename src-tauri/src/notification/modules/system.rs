use crate::notification::client::NotifyClient;
use tauri::api::notification::Notification;

#[derive(Clone, Debug)]
pub struct SystemModule {
    client: NotifyClient,
}

impl SystemModule {
    pub fn new(client: NotifyClient) -> Self {
        SystemModule { client }
    }

    pub fn send_notification(
        &self,
        title: &str,
        body: &str,
        icon: Option<&str>,
        sound: Option<&str>,
    ) {
        let sound = match sound {
            Some(s) => s,
            None => "Default",
        };
        let notification =
            Notification::new(&self.client.app_handler.config().tauri.bundle.identifier)
                .title(title)
                .body(body)
                .icon(icon.unwrap_or("assets/icons/icon.png"))
                .sound(sound);
        notification.show().unwrap();
    }
}
