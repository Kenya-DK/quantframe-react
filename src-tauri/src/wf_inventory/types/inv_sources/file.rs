use crate::wf_inventory::WarframeRootObject;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use utils::*;

use serde::{Deserialize, Serialize};

use super::{helpers::*, traits::InventorySource};

const COMPONENT: &str = "WFInvFile";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvFileSource {
    pub path: String,
    #[serde(skip, default = "default_stop_flag")]
    stop_flag: Arc<AtomicBool>,
}

impl PartialEq for WFInvFileSource {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

fn default_stop_flag() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

impl WFInvFileSource {
    fn resolve_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

impl InventorySource for WFInvFileSource {
    fn update(&self, root: &Arc<Mutex<WarframeRootObject>>) -> Result<(), Error> {
        let parsed = load_inventory(&self.resolve_path())?;

        let mut root = root.lock().map_err(|_| {
            Error::new(
                "WFInvFileSource:Lock",
                "Root mutex poisoned",
                get_location!(),
            )
        })?;
        info(
            format!("{COMPONENT}:Update:Complete"),
            "Data file read - root updated",
            &LoggerOptions::default(),
        );
        *root = parsed;
        Ok(())
    }

    fn start(&self, root: &Arc<Mutex<WarframeRootObject>>) {
        let source = self.clone();
        let root = root.clone();
        thread::spawn(move || {
            let path = source.resolve_path();

            if !path.exists() {
                warning(
                    format!("{COMPONENT}:Watcher"),
                    format!("Inventory data file not found at: {}", path.display()),
                    &LoggerOptions::default(),
                );
            }

            let mut last_modified = fs::metadata(&path).and_then(|m| m.modified()).ok();

            if path.exists() {
                if let Err(e) = source.update(&root) {
                    e.log("WFInventoryState.log").with_location(get_location!());
                }
            }

            loop {
                thread::sleep(Duration::from_millis(500));

                if source.stop_flag.load(Ordering::Relaxed) {
                    info(
                        format!("{COMPONENT}:Watcher"),
                        "Watcher stopped",
                        &LoggerOptions::default(),
                    );
                    break;
                }

                match fs::metadata(&path).and_then(|m| m.modified()) {
                    Ok(modified) => {
                        if last_modified.map_or(true, |last| modified > last) {
                            last_modified = Some(modified);
                            if let Err(e) = source.update(&root) {
                                e.log("WFInventoryState.log").with_location(get_location!());
                            }
                        }
                    }
                    Err(_) => {
                        if last_modified.is_some() {
                            last_modified = None;
                            warning(
                                format!("{COMPONENT}:Watcher"),
                                format!(
                                    "Inventory data file no longer accessible at: {}",
                                    path.display()
                                ),
                                &LoggerOptions::default(),
                            );
                        }
                    }
                }
            }
        });
    }

    fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    fn validate(&self) -> Result<(), Error> {
        if self.path.trim().is_empty() {
            return Err(Error::new(
                "WFInvFileSource:Validate",
                "File path is empty",
                get_location!(),
            ));
        }

        let path = self.resolve_path();
        if !path.exists() {
            return Err(Error::new(
                "WFInvFileSource:Validate",
                format!("Inventory data file not found at: {}", path.display()),
                get_location!(),
            ));
        }

        Ok(())
    }
}

fn load_inventory(path: &Path) -> Result<WarframeRootObject, Error> {
    let bytes = fs::read(path).map_err(|e| {
        Error::from_io(
            &format!("{COMPONENT}:Read"),
            &PathBuf::from(path),
            "Failed to read file",
            e,
            get_location!(),
        )
    })?;

    let text = String::from_utf8_lossy(&bytes);
    parse_lastdata(text.trim_start_matches('\u{feff}'))
}
