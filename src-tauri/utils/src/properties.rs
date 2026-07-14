use sea_orm::sea_query::ValueType;
use sea_orm::{TryGetError, TryGetable, Value, sea_query};
use serde::{Deserialize, Serialize};

use crate::{LoggerOptions, critical};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct Properties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl Properties {
    pub fn new(pairs: Vec<(String, impl Serialize)>) -> Self {
        let mut map = serde_json::Map::new();
        for (k, v) in pairs {
            map.insert(k, serde_json::to_value(v).unwrap());
        }
        Self {
            properties: Some(serde_json::Value::Object(map)),
        }
    }

    pub fn set_properties(&mut self, properties: Option<serde_json::Value>) {
        self.properties = properties;
    }

    pub fn get_properties<T>(&self, default: T) -> T
    where
        T: Default + serde::de::DeserializeOwned,
    {
        if let Some(props) = &self.properties {
            if let Ok(value) = serde_json::from_value(props.clone()) {
                return value;
            }
        }
        default
    }

    pub fn get_property_value<T>(&self, key: impl Into<String>, default: T) -> T
    where
        T: Default + serde::de::DeserializeOwned,
    {
        let key = key.into();
        if let Some(props) = &self.properties {
            if let Some(value) = props.get(&key) {
                if let Ok(value) = serde_json::from_value(value.clone()) {
                    return value;
                } else {
                    critical(
                        format!("{}:GetPropertyValue", "Properties"),
                        format!(
                            "Failed to deserialize property '{}' with value: {:?}",
                            key, value
                        ),
                        &LoggerOptions::default(),
                    );
                }
            }
        }
        default
    }
    pub fn has_property(&self, key: impl Into<String>) -> bool {
        let key = key.into();
        if let Some(props) = &self.properties {
            return props.get(&key).is_some();
        }
        false
    }
    pub fn set_property_value<T>(&mut self, key: impl Into<String>, value: T)
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_value(value).unwrap();

        if let Some(props) = &mut self.properties {
            if let Some(map) = props.as_object_mut() {
                map.insert(key.into(), value);
            }
        } else {
            let mut map = serde_json::Map::new();
            map.insert(key.into(), value);
            self.properties = Some(serde_json::Value::Object(map));
        }
    }
    pub fn remove_property_value(&mut self, key: impl Into<String>) {
        let key = key.into();
        if let Some(props) = &mut self.properties {
            if let Some(map) = props.as_object_mut() {
                map.remove(&key);
            }
        }
    }
    pub fn remove_property_values(&mut self, keys: &[&str]) {
        if let Some(props) = &mut self.properties {
            if let Some(map) = props.as_object_mut() {
                let keys_set: std::collections::HashSet<_> = keys.iter().cloned().collect();
                map.retain(|k, _| !keys_set.contains(k.as_str()));
            }
        }
    }
    pub fn keep_property_values(&mut self, keys: &[&str]) {
        if let Some(props) = &mut self.properties {
            if let Some(map) = props.as_object_mut() {
                let keys_set: std::collections::HashSet<_> = keys.iter().cloned().collect();
                map.retain(|k, _| keys_set.contains(k.as_str()));
            }
        }
    }
    pub fn update_property<T, F>(&mut self, key: impl Into<String>, mut f: F)
    where
        T: Default + serde::de::DeserializeOwned + serde::Serialize,
        F: FnMut(&mut T),
    {
        let key = key.into();

        let mut value: T = self.get_property_value(&key, T::default());

        f(&mut value);

        self.set_property_value(key, value);
    }

    pub fn merge_properties(
        &mut self,
        new_props: Option<serde_json::Value>,
        overwrite: bool,
        remove_if_null: bool,
    ) {
        if let Some(new_props) = new_props {
            if let Some(props) = &mut self.properties {
                if let (Some(map), Some(new_map)) = (props.as_object_mut(), new_props.as_object()) {
                    for (k, v) in new_map {
                        if overwrite || !map.contains_key(k) {
                            if remove_if_null && v.is_null() {
                                map.remove(k);
                            } else {
                                map.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
            } else {
                self.properties = Some(new_props);
            }
        }
    }
    pub fn is_type<T: serde::de::DeserializeOwned>(&self, key: impl Into<String>) -> bool {
        let key = key.into();
        if let Some(props) = &self.properties {
            if let Some(value) = props.get(&key) {
                return serde_json::from_value::<T>(value.clone()).is_ok();
            }
        }
        false
    }
    pub fn nullify_zeroed_properties(&mut self, keys: &[&str]) {
        for key in keys {
            let num: f64 = self.get_property_value(*key, f64::default());
            if num == 0.0 && self.has_property(*key) {
                self.set_property_value(*key, serde_json::Value::Null);
            }
        }
    }
    pub fn mask_sensitive_data(&mut self, properties: &[&str]) {
        if let Some(props) = &mut self.properties {
            if let Some(map) = props.as_object_mut() {
                crate::helper::mask_sensitive_data(map, properties);
            }
        }
    }
}

impl From<serde_json::Value> for Properties {
    fn from(value: serde_json::Value) -> Self {
        Self {
            properties: Some(value),
        }
    }
}

impl TryGetable for Properties {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let val: Option<serde_json::Value> = res.try_get_by(index)?;
        match val {
            Some(v) if !v.is_null() => Ok(Properties {
                properties: Some(v),
            }),
            _ => Ok(Properties::default()),
        }
    }
}

impl From<Properties> for Value {
    fn from(value: Properties) -> Self {
        Value::Json(value.properties.map(Box::new))
    }
}

impl ValueType for Properties {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        match v {
            Value::Json(Some(v)) => {
                let inner = *v;
                if inner.is_null() {
                    Ok(Properties::default())
                } else {
                    Ok(Properties {
                        properties: Some(inner),
                    })
                }
            }
            Value::Json(None) => Ok(Properties::default()),
            _ => Err(sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        String::from("json")
    }

    fn column_type() -> sea_query::ColumnType {
        sea_query::ColumnType::Json
    }

    fn array_type() -> sea_query::ArrayType {
        sea_query::ArrayType::Json
    }
}
