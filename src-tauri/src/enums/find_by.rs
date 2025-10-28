use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::{get_location, is_match, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindByType {
    Name,
    Id,
    Url,
    UniqueName,
    Category,
}
impl FindByType {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "name" => Ok(FindByType::Name),
            "id" => Ok(FindByType::Id),
            "url" => Ok(FindByType::Url),
            "unique_name" | "uniquename" => Ok(FindByType::UniqueName),
            "category" | "categories" => Ok(FindByType::Category),
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
    pub category: String,
    pub language: Language,
    pub remove_str: String,
    pub case_insensitive: bool,
}
impl FindBy {
    pub fn new(find_by: FindByType, value: impl Into<String>) -> Self {
        Self {
            find_by,
            value: value.into(),
            category: String::new(),
            language: Language::English,
            remove_str: String::new(),
            case_insensitive: false,
        }
    }
    
    pub fn is_match(&self, input: impl Into<String>) -> bool {
        let r = if self.remove_str.is_empty() {
            None
        } else {
            Some(self.remove_str.clone())
        };
        is_match(&self.value, input, self.case_insensitive, r)
    }
    pub fn from_str(by: impl Into<String>, value: impl Into<String>) -> Result<Self, Error> {
        let mut find_by: Option<FindByType> = None;
        let mut category: String = String::new();
        let mut language: Language = Language::English;
        let mut remove_str: String = String::new();
        let mut case_insensitive: bool = false;
        let by = by.into();
        let mut iter = by.split_whitespace().peekable();
        while let Some(token) = iter.next() {
            match token {
                "--by" => {
                    if let Some(val) = iter.next() {
                        find_by = Some(FindByType::from_str(val)?);
                    }
                }
                "--cat" => {
                    if let Some(val) = iter.next() {
                        category = val.to_string();
                    }
                }
                "--lang" => {
                    if let Some(val) = iter.next() {
                        language = Language::from_str(val)?;
                    }
                }
                "--remove_str" => {
                    if let Some(val) = iter.next() {
                        remove_str = val.to_string();
                    }
                }
                "--case-insensitive" => {
                    case_insensitive = true;
                }
                _ => {}
            }
        }

        Ok(Self {
            find_by: find_by.ok_or_else(|| {
                Error::new(
                    "FindBy::from_str",
                    "Missing required flag --by",
                    get_location!(),
                )
            })?,
            value: value.into(),
            category,
            language,
            remove_str,
            case_insensitive,
        })
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
