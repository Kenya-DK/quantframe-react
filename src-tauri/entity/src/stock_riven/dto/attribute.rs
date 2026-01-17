use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RivenAttribute {
    pub positive: bool,
    pub value: f64,
    pub effect: Option<String>,
    pub url_name: String,
}
impl RivenAttribute {
    pub fn new(positive: bool, value: f64, url_name: String) -> Self {
        Self {
            effect: None,
            url_name,
            positive,
            value,
        }
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
}
