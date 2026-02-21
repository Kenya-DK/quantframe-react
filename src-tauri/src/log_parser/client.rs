use crate::{app::AdvancedSettings, helper, log_parser::*};
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
    thread,
};
use utils::*;

pub struct LogParserState {
    watcher: FileWatcher,
    // Modules's'
    warframe_gdpr_module: OnceLock<Arc<WarframeGDPRModule>>,
}

impl LogParserState {
    pub fn new(settings: &AdvancedSettings) -> Arc<Self> {
        let mut path = LogParserState::get_default_path();
        if !settings.wf_log_path.is_empty() {
            path = PathBuf::from(settings.wf_log_path.clone());
        }
        let this = Arc::new(Self {
            watcher: FileWatcher::new(path.to_str().unwrap()),
            warframe_gdpr_module: OnceLock::new(),
        });
        this.watcher
            .add_handler(Box::new(OnTradeEvent::new("LogParserState")));
        this.watcher
            .add_handler(Box::new(OnConversationEvent::new()));
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
    pub fn warframe_gdpr(&self) -> Arc<WarframeGDPRModule> {
        self.warframe_gdpr_module
            .get_or_init(|| WarframeGDPRModule::new())
            .clone()
    }
    pub fn get_all_cached_lines(&self) -> Vec<LineEntry> {
        self.watcher.get_all_cached_lines()
    }
    pub fn get_cached_lines_between(&self, start: usize, end: usize) -> Vec<LineEntry> {
        self.watcher.get_cached_lines_between(start, end)
    }
}
