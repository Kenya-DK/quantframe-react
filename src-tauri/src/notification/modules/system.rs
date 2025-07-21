use std::sync::{Arc, Weak};

use crate::notification::client::NotificationState;

#[derive(Debug)]
pub struct SystemModel {
    client: Weak<NotificationState>,
}

impl SystemModel {
    /**
     * Creates a new `SystemModel` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<NotificationState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    /**
     * Creates a new `SystemModel` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &SystemModel, client: Arc<NotificationState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
