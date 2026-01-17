use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RivenAttribute {
    pub positive: bool,
    pub value: f64,
    pub url_name: String,
}
impl RivenAttribute {
    pub fn new(positive: bool, value: f64, url_name: String) -> Self {
        Self {
            positive,
            value,
            url_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct RivenAttributeVec(pub Vec<RivenAttribute>);
