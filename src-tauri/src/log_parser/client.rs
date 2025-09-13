use crate::{
    app::AdvancedSettings,
    emit_error,
    enums::*,
    helper,
    live_scraper::modules::*,
    log_parser::*,
    utils::{modules::states, OrderListExt},
};
use serde_json::Value;
use std::{
    collections::HashMap,
    num::NonZeroU32,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
    thread,
};
use utils::*;

pub struct LogParserState {
    watcher: FileWatcher,
}

impl LogParserState {
    pub fn new(settings: &AdvancedSettings) -> Arc<Self> {
        let mut path = LogParserState::get_default_path();
        if !settings.wf_log_path.is_empty() {
            path = PathBuf::from(settings.wf_log_path.clone());
        }
        let this = Arc::new(Self {
            watcher: FileWatcher::new(path.to_str().unwrap()),
        });
        this.watcher
            .add_handler(Box::new(OnTradeEvent::new("LogParserState")));
        this.watcher
            .add_handler(Box::new(OnConversationEvent::new("LogParserState")));
        LogParserState::start(this.clone()); // pass Arc
        this
    }
    pub fn get_default_path() -> PathBuf {
        helper::get_local_data_path()
            .join("Warframe")
            .join("EE.log")
    }
    fn start(this: Arc<Self>) {
        thread::spawn(move || match this.watcher.watch() {
            Ok(_) => {}
            Err(e) => {
                e.log("LogParserState.log").with_location(get_location!());
            }
        });
    }
    pub fn set_path(&self, path: &str) -> Result<(), Error> {
        if path.is_empty() {
            self.watcher
                .set_path(LogParserState::get_default_path().to_str().unwrap());
            return Ok(());
        }
        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            return Err(Error::new(
                "LogParserState:set_path",
                &format!("Path does not exist: {}", path),
                get_location!(),
            )
            .with_location(get_location!()));
        }
        self.watcher.set_path(path);
        Ok(())
    }
}
