use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MultiKeyMap<V> {
    next_id: u64,
    values: HashMap<u64, V>,
    keys: HashMap<String, u64>,
}

impl<V: Clone> MultiKeyMap<V> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            values: HashMap::new(),
            keys: HashMap::new(),
        }
    }

    /// Insert a value and one or more keys pointing to it.
    pub fn insert_value<S: Into<String>>(
        &mut self,
        value: V,
        key_list: impl IntoIterator<Item = S>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.values.insert(id, value);

        for key in key_list {
            let key = key.into().to_lowercase();
            if self.keys.contains_key(&key) {
                continue;
            }
            self.keys.insert(key, id);
        }

        id
    }

    /// Add additional keys to an existing value.
    pub fn add_keys<S: Into<String>>(&mut self, id: u64, key_list: impl IntoIterator<Item = S>) {
        for key in key_list {
            let key = key.into().to_lowercase();
            if self.keys.contains_key(&key) {
                continue;
            }
            self.keys.insert(key, id);
        }
    }

    /// Get value by any of its keys.
    pub fn get(&self, key: &str) -> Option<&V> {
        let id = self.keys.get(&key.to_lowercase())?;
        self.values.get(id)
    }

    /// Mutable access to value by any key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        let id = self.keys.get(&key.to_lowercase())?;
        self.values.get_mut(id)
    }
    pub fn clear(&mut self) {
        self.next_id = 0;
        self.values.clear();
        self.keys.clear();
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn get_all_values(&self) -> Vec<V> {
        self.values.values().cloned().collect()
    }
    pub fn get_all_keys(&self) -> Vec<String> {
        self.keys.keys().cloned().collect()
    }
    pub fn has_key(&self, key: &str) -> bool {
        self.keys.contains_key(&key.to_lowercase())
    }
    pub fn from_hash_map(hash_map: HashMap<String, V>) -> Self {
        let mut multi_key_map = MultiKeyMap::new();
        for (key, value) in hash_map.into_iter() {
            multi_key_map.insert_value(value, vec![key]);
        }
        multi_key_map
    }
    pub fn to_hash_map(&self) -> HashMap<String, V> {
        let mut hash_map = HashMap::new();
        for (key, id) in self.keys.iter() {
            if let Some(value) = self.values.get(id) {
                hash_map.insert(key.clone(), value.clone());
            }
        }
        hash_map
    }
}
