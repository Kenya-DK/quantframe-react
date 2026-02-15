use crate::{
    app::Settings,
    utils::modules::states,
    wf_inventory::{modules::*, WarframeRootObject},
};
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
type DecryptThingy = cbc::Decryptor<aes::Aes128>;
use serde_json::Value;
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::{Duration, Instant, SystemTime},
};
use utils::*;

const COMPONENT: &str = "WFInventory";

pub struct WFInventoryState {
    pub root: Mutex<WarframeRootObject>,
    pub last_update: Mutex<Instant>,
    path: PathBuf,
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
}

impl WFInventoryState {
    pub fn new(settings: &Settings) -> Arc<Self> {
        let mut path = WFInventoryState::get_default_path();
        if !settings.wf_inventory_data_path.is_empty() {
            path = PathBuf::from(settings.wf_inventory_data_path.clone());
        }
        let state = Arc::new(Self {
            root: Mutex::new(WarframeRootObject::default()),
            last_update: Mutex::new(Instant::now() - Duration::from_secs(1000)),
            path,
            item_module: OnceLock::new(),
            riven_module: OnceLock::new(),
        });

        // Turn off watcher for now
        // Self::start_watcher(state.clone());
        state.init_modules();
        state
    }
    fn get_default_path() -> PathBuf {
        crate::helper::get_local_data_path()
            .join("AlecaFrame")
            .join("lastData.dat")
    }
    fn start_watcher(state: Arc<Self>) {
        thread::spawn(move || {
            let path = state.path.clone();

            // Check if file exists before starting
            if !path.exists() {
                warning(
                    format!("{}:Watcher", COMPONENT),
                    format!("Inventory data file not found at: {}", path.display()),
                    &LoggerOptions::default(),
                );
            }

            let mut last_modified = fs::metadata(&path).and_then(|m| m.modified()).ok();
            let rt = tokio::runtime::Runtime::new().unwrap();

            // Initial load if file exists
            if path.exists() {
                match rt.block_on(state.on_data_file_modified(&path, SystemTime::now())) {
                    Ok(_) => {}
                    Err(e) => {
                        e.log("WFInventoryState.log").with_location(get_location!());
                    }
                }
            }

            loop {
                thread::sleep(Duration::from_millis(500));

                match fs::metadata(&path).and_then(|m| m.modified()) {
                    Ok(modified) => {
                        if last_modified.map_or(true, |last| modified > last) {
                            last_modified = Some(modified);

                            if let Err(e) =
                                rt.block_on(state.on_data_file_modified(&path, modified))
                            {
                                e.log("WFInventoryState.log").with_location(get_location!());
                            }
                        }
                    }
                    Err(_) => {
                        // File doesn't exist or can't be accessed - silently skip
                        // Reset last_modified so we catch it when it appears
                        if last_modified.is_some() {
                            last_modified = None;
                            warning(
                                format!("{}:Watcher", COMPONENT),
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

    pub fn get_root(&self) -> WarframeRootObject {
        self.root.lock().unwrap().clone()
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
            .expect("ItemModule not initialized")
            .clone()
    }

    pub fn riven(&self) -> Arc<RivenModule> {
        self.riven_module
            .get()
            .expect("RivenModule not initialized")
            .clone()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn update_path(&mut self, new_path: PathBuf) {
        self.path = new_path;
    }

    fn update_modules(&mut self) {}
    async fn on_data_file_modified(&self, path: &Path, _modified: SystemTime) -> Result<(), Error> {
        let bytes = read_file(path)?;
        let data = decrypt_lastdata(&bytes).await?;
        let parsed = parse_lastdata(&data)?;

        let mut root = self.root.lock().map_err(|_| {
            Error::new(
                "WFInventoryState:Lock",
                "Root mutex poisoned",
                get_location!(),
            )
        })?;
        info(
            format!("{}:DataFileModified", COMPONENT),
            "Data file modified",
            &LoggerOptions::default(),
        );
        *self.last_update.lock().unwrap() = Instant::now();
        *root = parsed;
        Ok(())
    }
}

/* ========================== */
/*        HELPERS             */
/* ========================== */

fn read_file(path: &Path) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path).map_err(|e| {
        Error::from_io(
            &format!("{COMPONENT}:Open"),
            &PathBuf::from(path),
            "Failed to open file",
            e,
            get_location!(),
        )
    })?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|e| {
        Error::from_io(
            &format!("{COMPONENT}:Read"),
            &PathBuf::from(path),
            "Failed to read file",
            e,
            get_location!(),
        )
    })?;

    Ok(buf)
}

async fn decrypt_lastdata(data: &[u8]) -> Result<String, Error> {
    let af_api = states::app_state()?;

    let keys = match af_api.qf_client.alecaframe().get_decrypt_keys().await {
        Ok(keys) => keys,
        Err(err) => {
            return Err(Error::new(
                "DecryptLastData:GetKeys",
                format!("Failed to get decrypt keys: {err:?}"),
                get_location!(),
            ))
        }
    };

    let key: &[u8; 16] = keys.key.as_slice().try_into().map_err(|_| {
        Error::new(
            "DecryptLastData:KeySize",
            "Key must be 16 bytes",
            get_location!(),
        )
    })?;
    let iv: &[u8; 16] = keys.iv.as_slice().try_into().map_err(|_| {
        Error::new(
            "DecryptLastData:IvSize",
            "IV must be 16 bytes",
            get_location!(),
        )
    })?;

    let decrypted = DecryptThingy::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<NoPadding>(data)
        .map_err(|e| {
            Error::new(
                "DecryptLastData:Decrypt",
                format!("Decrypt failed: {e:?}"),
                get_location!(),
            )
        })?;

    String::from_utf8(decrypted).map_err(|e| {
        Error::new(
            "DecryptLastData:Utf8",
            format!("UTF-8 parse failed: {e:?}"),
            get_location!(),
        )
    })
}

fn parse_lastdata(raw: &str) -> Result<WarframeRootObject, Error> {
    let mut json = raw.trim_end_matches(|c| c != '}').to_string();

    let json_value = serde_json::from_str::<Value>(&json).map_err(|e| Error::from(e))?;
    if json_value.get("InventoryJson").is_some() {
        json = json_value["InventoryJson"]
            .to_string()
            .replace("\\\"", "\"")
            .replace("\\\"", "\"");
        json = json[1..json.len() - 1].to_string();
    }
    serde_json::from_str(&json).map_err(|e| Error::from(e))
}
