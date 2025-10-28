use serde_json::json;
use std::{
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, read_json_file, Error, LoggerOptions};

use crate::{
    cache::{client::CacheState, types::CacheTheme},
    helper, APP,
};

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
#[derive(Debug)]
pub struct ThemeModule {
    path: PathBuf,
    items: Mutex<Vec<CacheTheme>>,
}
impl ThemeModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("themePresets"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheTheme>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn get_theme_folder(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn load(&self) -> Result<(), Error> {
        if !self.path.exists() && !self.path.is_dir() {
            info(
                "Cache:Theme:load",
                "Theme cache path does not exist, creating it.",
                &LoggerOptions::default(),
            );
            return Err(Error::new(
                "Cache:Theme:load",
                "Theme cache path does not exist or is not a directory.",
                get_location!(),
            ));
        }

        // Get All files in the path
        let files = match std::fs::read_dir(&self.path) {
            Ok(files) => files,
            Err(e) => {
                return Err(Error::from(e).with_location(get_location!()));
            }
        };
        let mut items_lock = self.items.lock().unwrap();
        items_lock.clear(); // Clear existing items before loading new ones
        for file in files {
            let file = file.map_err(|e| Error::from(e).with_location(get_location!()))?;
            match read_json_file::<CacheTheme>(&file.path()) {
                Ok(items) => {
                    items_lock.push(items);
                    info(
                        "Cache:Theme:load",
                        "Loaded Theme items from cache",
                        &LoggerOptions::default(),
                    );
                }
                Err(e) => return Err(e.with_location(get_location!())),
            }
        }

        Ok(())
    }
    fn base64_encode_bytes(data: &[u8]) -> String {
        let mut encoded = String::new();
        let mut i = 0;

        while i < data.len() {
            let b1 = data[i];
            let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
            let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

            let triple = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

            encoded.push(BASE64_CHARS[((triple >> 18) & 0x3F) as usize] as char);
            encoded.push(BASE64_CHARS[((triple >> 12) & 0x3F) as usize] as char);

            if i + 1 < data.len() {
                encoded.push(BASE64_CHARS[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                encoded.push('=');
            }

            if i + 2 < data.len() {
                encoded.push(BASE64_CHARS[(triple & 0x3F) as usize] as char);
            } else {
                encoded.push('=');
            }

            i += 3;
        }

        encoded
    }
    pub fn pick_icon() -> Result<(PathBuf, String), Error> {
        let app = APP.get().expect("App handle not found");
        let image_path = app
            .dialog()
            .file()
            .set_title("Select Theme Icon max 15KB")
            .add_filter("PNG Image", &["png"])
            .blocking_pick_file();

        if image_path.is_none() {
            return Err(Error::new(
                "Cache:Theme:pick_icon",
                "User did not select an image",
                get_location!(),
            ));
        }
        let image_path = image_path.unwrap().into_path().unwrap();
        let metadata = std::fs::metadata(&image_path).map_err(|e| {
            Error::new(
                "Command:SaveTheme",
                format!("Error getting file metadata: {}", e),
                get_location!(),
            )
        })?;
        if metadata.len() > 15000 {
            return Err(Error::new(
                "Command:SaveTheme",
                "file.exceeds_max_size",
                get_location!(),
            )
            .set_log_level(utils::LogLevel::Warning));
        }
        let mut image_data = Vec::new();
        std::fs::File::open(&image_path)
            .and_then(|mut file| file.read_to_end(&mut image_data))
            .map_err(|e| {
                Error::new(
                    "Command:SaveTheme",
                    format!("Error reading image file: {}", e),
                    get_location!(),
                )
            })?;
        let image_base64 = Self::base64_encode_bytes(&image_data);
        Ok((image_path, image_base64))
    }

    pub fn create_theme(
        &self,
        name: impl Into<String>,
        author: impl Into<String>,
        properties: impl Into<serde_json::Value>,
    ) -> Result<(), Error> {
        let app = APP.get().expect("App handle not found");
        let (_, image_base64) = Self::pick_icon()?;
        let name = name.into();
        let author = author.into();
        let properties = properties.into();
        let theme_data = json!({
            "name": name,
            "author": author,
            "iconBase64": image_base64,
            "properties": properties,
        });

        let file_path = app
            .dialog()
            .file()
            .set_file_name(name.clone())
            .add_filter(name.clone(), &["json"])
            .set_directory(
                helper::get_app_storage_path()
                    .join("cache")
                    .join("themePresets"),
            )
            .blocking_save_file();
        if file_path.is_none() {
            return Err(Error::new(
                "Cache:Theme:save",
                "User did not select a file to save the theme",
                get_location!(),
            ));
        }
        let file_path = file_path.unwrap().into_path().unwrap();
        std::fs::write(
            &file_path,
            serde_json::to_string_pretty(&theme_data).unwrap(),
        )
        .map_err(|e| {
            Error::from(e)
                .with_location(get_location!())
                .set_message("Failed to write theme data to file")
        })?;
        Ok(())
    }
    /**
     * Creates a new `ThemeModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ThemeModule, _client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
