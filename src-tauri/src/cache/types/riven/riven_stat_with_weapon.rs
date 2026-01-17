use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cache::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RivenStatWithWeapon {
    pub name: String,
    pub disposition: f64,
    pub disposition_rank: i64,
    pub unique_name: String,
    pub by_level: HashMap<i64, Vec<RivenSingleAttribute>>,
}

impl RivenStatWithWeapon {
    pub fn new(
        name: impl Into<String>,
        unique_name: impl Into<String>,
        disposition: f64,
        disposition_rank: i64,
    ) -> Self {
        Self {
            name: name.into(),
            disposition,
            disposition_rank,
            unique_name: unique_name.into(),
            by_level: HashMap::new(),
        }
    }
    pub fn add_level_attributes(&mut self, level: i64, attributes: Vec<RivenSingleAttribute>) {
        self.by_level.insert(level, attributes);
    }
}
