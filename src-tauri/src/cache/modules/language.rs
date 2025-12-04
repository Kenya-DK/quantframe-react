use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::*;

#[derive(Debug)]
pub struct LanguageModule {
    path: PathBuf,
    languages: Mutex<HashMap<String, Value>>,
}

impl LanguageModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/lang"),
            languages: Mutex::new(HashMap::new()),
        })
    }
    pub fn load(&self) -> Result<(), Error> {
        // Get All language files in the directory
        let mut languages_lock = self.languages.lock().unwrap();
        let entries = std::fs::read_dir(&self.path).map_err(|e| {
            Error::new(
                "Cache:Language:Load",
                format!("Failed to read language directory: {}", e),
                get_location!(),
            )
        })?;
        for entry in entries {
            let entry = entry.map_err(|e| {
                Error::new(
                    "Cache:Language:Load",
                    format!("Failed to read language file entry: {}", e),
                    get_location!(),
                )
            })?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        if let Some(file_stem) = path.file_stem() {
                            let lang_code = file_stem.to_string_lossy().to_string();
                            let lang_content =
                                read_json_file_optional::<Value>(&path).map_err(|e| {
                                    Error::new(
                                        "Cache:Language:Load",
                                        format!(
                                            "Failed to read language file {}: {}",
                                            lang_code, e
                                        ),
                                        get_location!(),
                                    )
                                })?;
                            languages_lock.insert(lang_code, lang_content);
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn get_language(&self, lang: &str) -> Result<Value, Error> {
        let languages_lock = self.languages.lock().unwrap();
        if let Some(lang_content) = languages_lock.get(lang) {
            Ok(lang_content.clone())
        } else {
            Err(Error::new(
                "LanguageModule::GetLanguage",
                format!("Language '{}' not found in cache", lang),
                get_location!(),
            ))
        }
    }
    /**
     * Creates a new `LanguageModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &LanguageModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            languages: Mutex::new(old.languages.lock().unwrap().clone()),
        })
    }
}
