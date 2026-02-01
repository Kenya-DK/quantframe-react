use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldChange<T> {
    Ignore,   // don’t touch this field
    Value(T), // set to a new value
    Null,     // explicitly clear (if nullable)
}

impl<T> Default for FieldChange<T> {
    fn default() -> Self {
        FieldChange::Ignore
    }
}
impl<T> FieldChange<T> {
    pub fn get_value(&self, default: T) -> T
    where
        T: Clone,
    {
        match self {
            FieldChange::Value(v) => v.clone(),
            _ => default,
        }
    }
}
