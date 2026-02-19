use std::sync::{atomic::Ordering, Arc, LazyLock, Weak};

use entity::{dto::PaginatedResult, enums::FieldChange};
use utils::Error;
static COMPONENT: &str = "WFInventory:RivenModule";

use crate::{
    cache::RivenSummary,
    helper::paginate,
    utils::modules::states,
    wf_inventory::{enums::RivenType, *},
};

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

    pub fn get_unveiled(&self) -> Result<Vec<MiscItem>, Error> {
        let client = self.client.upgrade().unwrap();

        let root = client.get_root();

        let items: Vec<_> = root
            .raw_upgrades
            .iter()
            .chain(root.upgrades.iter())
            .filter(|item| {
                item.is_riven(RivenType::PreVeiled) || item.is_riven(RivenType::UnVeiled)
            })
            .cloned()
            .collect();
        Ok(items)
    }
    pub fn get_veiled(
        &self,
        query: VeiledRivensPaginationDto,
    ) -> Result<PaginatedResult<VeiledRiven>, Error> {
        let client = self.client.upgrade().unwrap();
        let root = client.get_root();
        let c = states::cache_client()?;

        let items: Vec<_> = root
            .raw_upgrades
            .iter()
            .chain(root.upgrades.iter())
            .filter(|item| item.is_riven(RivenType::Veiled))
            .cloned()
            .collect();

        let mut rivens: Vec<VeiledRiven> = vec![];
        for item in items.iter() {
            let fingerprint = item.get_upgrade_fingerprint();
            let riven = VeiledRiven::try_from_fingerprint(&fingerprint, &c).ok();
            if let Some(mut riven) = riven {
                // riven.grade_riven(&c)?;
                rivens.push(riven);
            }
        }

        match query.query {
            FieldChange::Value(query) => {
                rivens.retain(|riven| riven.matches_query(&query));
            }
            _ => {}
        }

        let paginate = paginate(&rivens, query.pagination.page, query.pagination.limit);
        Ok(paginate)
    }
    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
