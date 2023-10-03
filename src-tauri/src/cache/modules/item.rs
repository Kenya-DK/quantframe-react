use crate::{
    error::AppError,
    logger,
    structs::{Item, ItemDetails},
    cache::client::CacheClient,
};

pub struct ItemModule<'a> {
    pub client: &'a CacheClient,
    pub items: Arc<Mutex<Vec<Item>>>,
}

impl<'a> ItemModule<'a> {
    // Refrece
    pub async fn refresh(&self) -> Result<(), AppError> {
        self.refresh_types().await?;   
    }
    pub async fn refresh_types(&self) -> Result<Vec<Item>, AppError> {
        let wfm = self.wfm.lock()?.clone();
        let wfm_items = wfm.items().get_all_items().await?;
        let response: HashMap<String, Value> =
            reqwest::get("https://relics.run/history/item_data/item_info.json")
                .await
                .map_err(|e| AppError::new("CacheItems", eyre!(e.to_string())))?
                .json()
                .await
                .map_err(|e| AppError::new("CacheItems", eyre!(e.to_string())))?;

        let mut items: Vec<Item> = Vec::new();
        for item in wfm_items.clone() {
            let relic_data = response.get(&item.id.clone());
            if relic_data.is_some() {
                let mut new = item.clone();
                let set_items =self. get_string_arry_from_json(relic_data.unwrap(), "set_items");
                let tags = self.get_string_arry_from_json(relic_data.unwrap(), "tags");
                let subtypes = self.get_string_arry_from_json(relic_data.unwrap(), "subtypes");
                let mod_max_rank = relic_data.unwrap().get("mod_max_rank").unwrap().as_i64();
                new.set_items = Some(set_items);
                new.tags = Some(tags);
                new.subtypes = Some(subtypes);
                new.mod_max_rank = mod_max_rank;
                items.push(new.clone());
            }
        }

        let mut sitems = self.items.lock()?;
        *sitems = items.clone();
        Ok(items)
    }

    pub async fn get_types(&self) -> Result<Vec<Item>, AppError> {
        let items = self.items.lock()?;
        Ok(items.clone())
    }

    pub async fn find_type(&self, url_name: &str) -> Result<Option<RivenTypeInfo>, AppError> {
        let types = self.riven_types.lock()?;
        let item_type = types.iter().find(|&x| x.url_name == url_name);
        if item_type.is_some() {
            Ok(item_type.unwrap().clone())
        } else {
            logger::warning_con(
                "CacheItems",
                format!("Item Type: {} not found", url_name).as_str()
            );
            Ok(None)
        }
    }

    pub fn emit(&self) {
        helper::send_message_to_window(
            "Cache:Update:Items",
            Some(serde_json::to_value(self.items.clone()).unwrap()),
        );
    }
    fn get_string_arry_from_json(json: &Value, key: &str) -> Vec<String> {
        let mut string_vec = vec![];
        if let Some(array) = json.get(key).unwrap().as_array() {
            string_vec = array
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
        }
        string_vec
    }
}
