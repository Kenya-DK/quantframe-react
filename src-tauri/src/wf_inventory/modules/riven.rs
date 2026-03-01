use std::sync::{Arc, Weak};

use entity::{dto::PaginatedResult, enums::FieldChange};
use serde_json::json;
use utils::{log_json_formatted, warning, Error, LoggerOptions};
static COMPONENT: &str = "WFInventory:RivenModule";

use crate::{helper::paginate, utils::modules::states, wf_inventory::*};

#[derive(Debug)]
pub struct RivenModule {
    client: Weak<WFInventoryState>,
}

impl RivenModule {
    /**
     * Creates a new `RivenModule` with an empty item list.
     * The `client` parameter is an `Arc<WFInventoryState>` that allows the module
     * to access the live scraper state.
     */
    pub fn get_rivens(
        &self,
        query: WFItemPaginationDto,
    ) -> Result<PaginatedResult<WFInvItemRiven>, Error> {
        let client = self.client.upgrade().unwrap();
        let root = client.get_root();
        let cache = states::cache_client()?;

        let items: Vec<_> = root
            .raw_upgrades
            .iter()
            .chain(root.upgrades.iter())
            .filter(|item| item.is_riven())
            .cloned()
            .collect();

        let mut rivens: Vec<WFInvItemRiven> = vec![];
        for item in items.iter() {
            match WFInvItemRiven::try_from_raw(item, &cache) {
                Ok(riven) => rivens.push(riven),
                Err(e) => {
                    warning(
                        format!("{}:ParseRiven", COMPONENT),
                        format!("Failed to parse riven from raw item: {}", e),
                        &LoggerOptions::default(),
                    );
                }
            }
        }

        match query.query {
            FieldChange::Value(query) => {
                rivens.retain(|riven| riven.base.matches_query(&query));
            }
            _ => {}
        }

        match query.item_types {
            FieldChange::Value(query) => {
                rivens.retain(|riven| {
                    query
                        .iter()
                        .any(|item_type| riven.base.unique_name.contains(item_type))
                });
            }
            _ => {}
        }
        log_json_formatted(json!(rivens), "RivenList.json", true)?;
        let paginate = paginate(&rivens, query.pagination.page, query.pagination.limit);
        Ok(paginate)
    }
    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
