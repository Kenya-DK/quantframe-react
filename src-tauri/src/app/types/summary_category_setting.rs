use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummaryCategorySetting {
    pub icon: String,
    pub name: String,
    pub types: Vec<String>,
    pub tags: Vec<String>,
}
impl SummaryCategorySetting {
    pub fn new(icon: &str, name: &str, types: Vec<&str>, tags: Vec<&str>) -> Self {
        Self {
            icon: icon.to_string(),
            name: name.to_string(),
            types: types.iter().map(|s| s.to_string()).collect(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
        }
    }
}
