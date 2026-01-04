use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldChange<T> {
    Ignore,   // don’t touch this field
    Value(T), // set to a new value
    Null,     // explicitly clear (if nullable)
}

// get_or_default method to return the value or a default
impl<T: Default + Clone> FieldChange<T> {
    pub fn get_or_default(&self, default: T) -> T {
        match self {
            FieldChange::Value(val) => val.clone(),
            _ => default,
        }
    }
}

impl<T> Default for FieldChange<T> {
    fn default() -> Self {
        FieldChange::Ignore
    }
}
