use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::{get_location, read_json_file_optional, validate_json, Error};

use crate::helper;

fn get_path() -> PathBuf {
    helper::get_app_storage_path().join("cache_version.json")
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheVersion {
    pub id: String,
    pub id_price: String,
    pub db_version: String,
}
impl Default for CacheVersion {
    fn default() -> Self {
        CacheVersion {
            id: "N/A".to_string(),
            id_price: "N/A".to_string(),
            db_version: "1".to_string(),
        }
    }
}
impl CacheVersion {
    pub fn load() -> Result<Self, Error> {
        let path = get_path();
        match read_json_file_optional::<CacheVersion>(&path) {
            Ok(version) => Ok(version),
            Err(e) => {
                e.with_location(get_location!()).log("cache_version.log");
                Ok(CacheVersion::default())
            }
        }
    }
    pub fn save(&self) -> Result<(), Error> {
        let path = &get_path();
        let content = serde_json::to_string(self).map_err(|e| {
            Error::from_json(
                "CacheVersion",
                path,
                "N/A",
                "Failed to serialize cache version",
                e,
                get_location!(),
            )
        })?;
        std::fs::write(path, content).map_err(|e| {
            Error::from_io(
                "CacheVersion",
                path,
                "Failed to write cache version to file",
                e,
                get_location!(),
            )
        })?;
        Ok(())
    }
}
