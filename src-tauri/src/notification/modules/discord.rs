use std::sync::{Arc, Weak};

use crate::notification::client::NotificationState;

#[derive(Debug)]
pub struct DiscordModel {
    client: Weak<NotificationState>,
}

impl DiscordModel {
    /**
     * Creates a new `DiscordModel` with an empty Authentication list.
     * The `client` parameter is an `Arc<NotificationState<State>>` that allows the route
     */
    pub fn new(client: Arc<NotificationState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    /**
     * Creates a new `DiscordModel` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &DiscordModel, client: Arc<NotificationState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
