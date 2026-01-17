use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use utils::{get_location, read_json_file_optional, Error};

use crate::cache::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TranslationEntry {
    #[serde(default)]
    full: String,
    #[serde(default)]
    short: String,
    #[serde(default)]
    effect: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    wfm_name: String,
}

pub enum LanguageKey {
    Full,
    Short,
    Text,
    Name,
    WfmName,
}

#[derive(Debug)]
pub struct LanguageModule {
    path: PathBuf,
    languages: Mutex<HashMap<String, HashMap<String, TranslationEntry>>>,
    current_language: Mutex<HashMap<String, TranslationEntry>>,
}

impl LanguageModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("lang"),
            languages: Mutex::new(HashMap::new()),
            current_language: Mutex::new(HashMap::new()),
        })
    }

    pub fn load(&self, lang: impl Into<String>) -> Result<(), Error> {
        let data = self.get_language(lang.into())?;
        let mut current_language_lock = self.current_language.lock().unwrap();
        *current_language_lock = data;
        Ok(())
    }

    pub fn get_language(
        &self,
        lang: impl Into<String>,
    ) -> Result<HashMap<String, TranslationEntry>, Error> {
        let lang = lang.into();
        let mut languages_lock = self.languages.lock().unwrap();
        if let Some(lang_content) = languages_lock.get(&lang) {
            return Ok(lang_content.clone());
        }

        let path = self.path.join(format!("{}.json", lang));
        if !path.exists() {
            return Err(Error::new(
                "LanguageModule::GetLanguage",
                format!(
                    "Language file for '{}' does not exist at path: {:?}",
                    lang, path
                ),
                get_location!(),
            ));
        }

        match read_json_file_optional::<HashMap<String, TranslationEntry>>(&path) {
            Ok(data) => {
                languages_lock.insert(lang.clone(), data.clone());
                Ok(data)
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }
    pub fn translate(&self, key: impl Into<String>, lang_key: LanguageKey) -> Option<String> {
        let current_language_lock = self.current_language.lock().unwrap();
        if let Some(entry) = current_language_lock.get(&key.into()) {
            let value = match lang_key {
                LanguageKey::Full => entry.full.clone(),
                LanguageKey::Short => entry.short.clone(),
                LanguageKey::Text => entry.text.clone(),
                LanguageKey::Name => entry.name.clone(),
                LanguageKey::WfmName => entry.wfm_name.clone(),
            };
            if !value.is_empty() {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_mapper(&self, lang_key: LanguageKey) -> HashMap<String, String> {
        let current_language_lock = self.current_language.lock().unwrap();
        let mut mapper: HashMap<String, String> = HashMap::new();
        for (key, entry) in current_language_lock.iter() {
            let value = match lang_key {
                LanguageKey::Full => entry.full.clone(),
                LanguageKey::Short => entry.short.clone(),
                LanguageKey::Text => entry.text.clone(),
                LanguageKey::Name => entry.name.clone(),
                LanguageKey::WfmName => entry.wfm_name.clone(),
            };
            if !value.is_empty() {
                mapper.insert(key.clone(), value);
            }
        }
        mapper
    }
    /**
     * Creates a new `LanguageModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &LanguageModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            current_language: Mutex::new(old.current_language.lock().unwrap().clone()),
            languages: Mutex::new(old.languages.lock().unwrap().clone()),
        })
    }
}
