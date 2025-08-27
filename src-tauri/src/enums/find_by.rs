use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindByType {
    Name,
    Url,
    UniqueName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    English,
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
