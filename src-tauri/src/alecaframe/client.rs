use crate::{
    emit_error,
    enums::*,
    live_scraper::modules::*,
    play_sound, send_event,
    types::UIEvent,
    utils::{modules::states, OrderListExt},
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
    time::{Duration, Instant},
};
use utils::{get_location, warning, LogLevel, LoggerOptions};

#[derive(Debug)]
pub struct AlecaframeState {
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
}

impl AlecaframeState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            item_module: OnceLock::new(),
            riven_module: OnceLock::new(),
        })
    }

    fn init_modules(self: &Arc<Self>) {
        self.item_module
            .get_or_init(|| ItemModule::new(self.clone()));
        self.riven_module
            .get_or_init(|| RivenModule::new(self.clone()));
    }
    
    pub fn item(&self) -> Arc<ItemModule> {
        self.item_module
            .get()
            .expect("Item module not initialized")
            .clone()
    }

    pub fn riven(&self) -> Arc<RivenModule> {
        self.riven_module
            .get()
            .expect("Riven module not initialized")
            .clone()
    }
}
