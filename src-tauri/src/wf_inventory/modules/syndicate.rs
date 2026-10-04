use std::sync::{Arc, Weak};

use entity::{dto::PaginatedResult, enums::FieldChange};
use utils::*;

use crate::{
    helper::paginate,
    utils::modules::states,
    wf_inventory::{item_base::WFInvItemBase, *},
};

#[derive(Debug)]
pub struct SyndicateModule {
    client: Weak<WFInventoryState>,
}

impl SyndicateModule {
    pub fn get_syndicates(
        &self,
        query: WFItemPaginationDto,
    ) -> Result<PaginatedResult<WFInvItemBase>, Error> {
        let client = self.client.upgrade().unwrap();
        let root = client.get_root();
        let cache = states::cache_client()?;

        let affiliations = root.affiliations;
        let mut items = affiliations
            .iter()
            .map(|affiliation| WFInvItemBase::from_affiliation(affiliation, &cache))
            .collect::<Result<Vec<_>, _>>()?;

        match query.properties {
            FieldChange::Value(properties) => {
                let can_select = properties.get_property_value::<Option<bool>>("can_select", None);
                if can_select.is_some() {
                    let can_select = can_select.unwrap();
                    items.retain(|item| {
                        item.properties
                            .get_property_value::<bool>("can_select", false)
                            == can_select
                    });
                }
            }
            _ => {}
        }
        let paginate = paginate(&items, query.pagination.page, query.pagination.limit);
        Ok(paginate)
    }

    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
