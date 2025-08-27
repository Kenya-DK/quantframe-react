use crate::notification::{enums::*, modules::*};
use std::{
    num::{NonZero, NonZeroU32},
    sync::{Arc, OnceLock},
};

#[derive(Debug, Clone)]
pub struct NotificationState {
    self_arc: OnceLock<Arc<NotificationState>>,
    // Routes
    discord_model: OnceLock<Arc<DiscordModel>>,
    system_model: OnceLock<Arc<SystemModel>>,
}
impl NotificationState {
    fn arc(&self) -> Arc<Self> {
        self.self_arc.get_or_init(|| Arc::new(Self::new())).clone()
    }
    pub fn new() -> Self {
        Self {
            self_arc: OnceLock::new(),
            discord_model: OnceLock::new(),
            system_model: OnceLock::new(),
        }
    }

    // Endpoint methods to access routes
    pub fn discord(&self) -> Arc<DiscordModel> {
        self.discord_model
            .get_or_init(|| DiscordModel::new(self.arc()))
            .clone()
    }
    pub fn system(&self) -> Arc<SystemModel> {
        self.system_model
            .get_or_init(|| SystemModel::new(self.arc()))
            .clone()
    }
}
