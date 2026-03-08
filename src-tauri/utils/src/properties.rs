use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct Properties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl Properties {
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
        if let Some(props) = &self.properties {
            if let Some(value) = props.get(&key.into()) {
                return serde_json::from_value(value.clone()).unwrap();
            }
        }
        default
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
}

impl From<serde_json::Value> for Properties {
    fn from(value: serde_json::Value) -> Self {
        Self {
            properties: Some(value),
        }
    }
}
