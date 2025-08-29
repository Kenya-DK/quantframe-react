use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::{get_location, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindByType {
    Name,
    Id,
    Url,
    UniqueName,
}
impl FindByType {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "name" => Ok(FindByType::Name),
            "id" => Ok(FindByType::Id),
            "url" => Ok(FindByType::Url),
            "unique_name" | "uniquename" => Ok(FindByType::UniqueName),
            _ => Err(Error::new(
                "FindByType::from_str",
                format!("Unknown find_by type: {}", s),
                get_location!(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    English,
}
impl Language {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Ok(Language::English),
            _ => Err(Error::new(
                "Language::from_str",
                format!("Unknown language: {}", s),
                get_location!(),
            )),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindBy {
    pub find_by: FindByType,
    pub value: String,
    pub language: Language,
}
impl FindBy {
    pub fn new(find_by: FindByType, value: impl Into<String>) -> Self {
        Self {
            find_by,
            value: value.into(),
            language: Language::English,
        }
    }
}

impl Display for FindBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "was not found searching by: {}", self.value)?;

        if self.find_by == FindByType::Name {
            write!(f, " with language: {:?}", self.language)?;
        } else {
            write!(f, " via {:?}", self.find_by)?;
        }
        Ok(())
    }
}
