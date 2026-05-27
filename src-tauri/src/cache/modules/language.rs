use std::{
    collections::HashMap,
    ops::Mul,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use utils::{get_location, read_json_file_optional, Error, MultiKeyMap};

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
    pub name: String,
    #[serde(rename = "wfmName", default)]
    pub wfm_name: String,
}

#[derive(Debug)]
pub struct LanguageModule {
    path: PathBuf,
    languages: Mutex<HashMap<String, MultiKeyMap<TranslationEntry>>>,
    current_language: Mutex<MultiKeyMap<TranslationEntry>>,
}

impl LanguageModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("lang"),
            languages: Mutex::new(HashMap::new()),
            current_language: Mutex::new(MultiKeyMap::new()),
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
    ) -> Result<MultiKeyMap<TranslationEntry>, Error> {
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
                let multi_key_map = MultiKeyMap::from_hash_map(data);
                languages_lock.insert(lang.clone(), multi_key_map.clone());
                Ok(multi_key_map)
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }

    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    /// Get a resource item by various identifiers
    ///  # Arguments
    /// - `id`: The identifier to search for (name, url, unique name, or id)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<TranslationEntry, Error> {
        let id: String = id.into().trim_end().to_string();
        let lookup = self.current_language.lock().unwrap();
        if let Some(item) = lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:Language:GetBy",
                format!("Resource item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_mapper(&self, lang_key: impl Into<String>) -> HashMap<String, String> {
        let lang_key = lang_key.into();
        let current_language_lock = self.current_language.lock().unwrap();
        let mut mapper: HashMap<String, String> = HashMap::new();
        for (key, entry) in current_language_lock.to_hash_map() {
            let value = match lang_key.as_str() {
                "full" => entry.full.clone(),
                "short" => entry.short.clone(),
                "text" => entry.text.clone(),
                "name" => entry.name.clone(),
                "wfmName" => entry.wfm_name.clone(),
                _ => continue,
            };
            if !value.is_empty() {
                mapper.insert(key.clone(), value);
            }
        }
        mapper
    }
}
