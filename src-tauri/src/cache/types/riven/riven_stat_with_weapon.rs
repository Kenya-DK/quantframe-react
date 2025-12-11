use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cache::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RivenStatWithWeapon {
    pub name: String,
    pub disposition: f64,
    pub by_level: HashMap<i64, Vec<RivenSingleAttribute>>,
}

impl RivenStatWithWeapon {
    pub fn new(name: impl Into<String>, disposition: f64) -> Self {
        Self {
            name: name.into(),
            disposition,
            by_level: HashMap::new(),
        }
    }
    pub fn add_level_attributes(&mut self, level: i64, attributes: Vec<RivenSingleAttribute>) {
        self.by_level.insert(level, attributes);
    }
}
