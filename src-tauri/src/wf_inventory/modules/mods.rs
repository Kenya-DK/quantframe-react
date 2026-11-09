use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use entity::{dto::PaginatedResult, enums::FieldChange};
use utils::Error;

static COMPONENT: &str = "WFInventory:ModsModule";

use crate::{
    helper::paginate, utils::modules::states, wf_inventory::item_base::WFInvItemBase,
    wf_inventory::*,
};

#[derive(Debug)]
pub struct ModsModule {
    client: Weak<WFInventoryState>,
}

impl ModsModule {
    /// Returns filtered and paginated mod inventory.
    pub fn get_mods(
        &self,
        query: WFItemPaginationDto,
    ) -> Result<PaginatedResult<WFInvItemBase>, Error> {
        let client = self.client.upgrade().unwrap();
        let root = client.get_root();
        let cache = states::cache_client()?;

        let mut mods: HashMap<String, WFInvItemBase> = HashMap::new();

        // Convert mods and merge duplicates by UUID.
        for item in root.raw_upgrades.iter().chain(&root.upgrades) {
            if !item.is_mod() {
                continue;
            }

            let Ok(mod_item) = WFInvItemBase::from_mod(item, &cache) else {
                continue;
            };

            let key = mod_item.uuid();

            match mods.get_mut(&key) {
                Some(existing) => existing.quantity += mod_item.quantity,
                None => {
                    mods.insert(key, mod_item);
                }
            }
        }

        // Search filter.
        if let FieldChange::Value(search) = query.query {
            mods.retain(|_, item| item.matches_query(&search));
        }

        // Pagination.
        Ok(paginate(
            &mods.into_values().collect::<Vec<_>>(),
            query.pagination.page,
            query.pagination.limit,
        ))
    }

    /// Creates a new ModsModule.
    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
