use std::sync::{Arc, Weak};

use utils::*;

use crate::{cache::CacheSyndicateTitle, utils::modules::states, wf_inventory::*};

#[derive(Debug)]
pub struct SyndicateModule {
    client: Weak<WFInventoryState>,
}

impl SyndicateModule {
    pub fn get_syndicates(&self) -> Result<Vec<WFInvItemBase>, Error> {
        let client = self.client.upgrade().unwrap();
        let root = client.get_root();
        let cache = states::cache_client()?;

        let affiliations = root.affiliations;
        let mut items = vec![];
        for affiliation in affiliations.iter() {
            let syndicate = cache.syndicate().get_by(affiliation.id.clone())?;

            let title = syndicate
                .get_title_by_level(affiliation.rank as i32)
                .unwrap_or(CacheSyndicateTitle::default());

            let standing = affiliation.standing;
            let min_standing = if affiliation.rank < 0 {
                title.max_standing
            } else {
                title.min_standing
            };

            let mut item = WFInvItemBase::default();
            item.id = affiliation.id.clone();
            item.name = syndicate.name.clone();
            item.unique_name = syndicate.unique_name.clone();
            item.sub_type = Some(entity::dto::SubType::rank(affiliation.rank as i64));

            item.quantity = standing - min_standing;
            item.properties
                .set_property_value("total".to_string(), standing);
            item.properties
                .set_property_value("min_standing".to_string(), title.min_standing);
            item.properties
                .set_property_value("max_standing".to_string(), title.max_standing);
            item.properties.set_property_value(
                "background_colour".to_string(),
                syndicate.background_colour.value.clone(),
            );
            item.properties
                .set_property_value("colour".to_string(), syndicate.colour.value.clone());
            items.push(item);
        }
        Ok(items)
    }

    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
