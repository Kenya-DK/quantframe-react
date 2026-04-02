use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use utils::Properties;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RivenAttribute {
    pub url_name: String,
    pub positive: bool,
    pub value: f64,
    #[serde(default)]
    pub localized_text: String,
    // Extra properties
    #[serde(flatten)]
    pub properties: Properties,
}
impl RivenAttribute {
    pub fn new(positive: bool, value: f64, url_name: String, localized_text: String) -> Self {
        Self {
            localized_text,
            url_name,
            positive,
            value,
            properties: Properties::default(),
        }
    }
    pub fn to_raw(&self) -> (String, f64, bool) {
        (self.url_name.clone(), self.value, self.positive)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct RivenAttributeVec(pub Vec<RivenAttribute>);

impl RivenAttributeVec {
    pub fn total_buff_curse_count(&self) -> (usize, usize) {
        let mut buff_count = 0;
        let mut curse_count = 0;
        for att in self.0.iter() {
            if att.positive {
                buff_count += 1;
            } else {
                curse_count += 1;
            }
        }
        (buff_count, curse_count)
    }
    pub fn to_raw(&self) -> Vec<(String, f64, bool)> {
        self.0.iter().map(|att| att.to_raw()).collect()
    }
}

pub trait IntoRawVec {
    fn into_raw(self) -> Vec<(String, f64, bool)>;
}

impl IntoRawVec for Vec<RivenAttribute> {
    fn into_raw(self) -> Vec<(String, f64, bool)> {
        self.into_iter().map(|att| att.to_raw()).collect()
    }
}
