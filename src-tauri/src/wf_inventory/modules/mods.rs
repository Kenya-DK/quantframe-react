use std::collections::HashMap;
use std::sync::{Arc, Weak};

use entity::{dto::PaginatedResult, enums::FieldChange};
use utils::{warning, Error, LoggerOptions, SortDirection};
static COMPONENT: &str = "WFInventory:ModsModule";

use crate::{
    helper::paginate, utils::modules::states, wf_inventory::item_base::WFInvItemBase,
    wf_inventory::*,
};

/// Collects the non-riven mods from RawUpgrades + Upgrades and merges duplicate
/// copies (same unique_name + rank) into single entries with combined quantity.
fn merge_raw_mods(root: &WarframeRootObject) -> HashMap<String, WFInvItemRaw> {
    let mut uniques: HashMap<String, WFInvItemRaw> = HashMap::new();
    for item in root.raw_upgrades.iter().chain(root.upgrades.iter()) {
        if !item.unique_name.starts_with("/Lotus/Upgrades/Mods/") || item.is_riven() {
            continue;
        }
        let key = format!(
            "{}|{}",
            item.unique_name,
            item.get_upgrade_fingerprint().mod_rank
        );
        match uniques.get_mut(&key) {
            Some(merged) => merged.quantity += item.quantity,
            None => {
                uniques.insert(key, item.clone());
            }
        }
    }
    uniques
}

fn sort_items<T: Ord>(
    items: &mut [WFInvItemBase],
    dir: &SortDirection,
    mut key: impl FnMut(&WFInvItemBase) -> T,
) {
    match dir {
        SortDirection::Asc => items.sort_by(|a, b| key(a).cmp(&key(b))),
        SortDirection::Desc => items.sort_by(|a, b| key(b).cmp(&key(a))),
    }
}

#[derive(Debug)]
pub struct ModsModule {
    client: Weak<WFInventoryState>,
}

impl ModsModule {
    /**
     * Creates a new `ModsModule` with an empty item list.
     * The `client` parameter is an `Arc<WFInventoryState>` that allows the module
     * to access the live scraper state.
     */
    pub fn get_mods(
        &self,
        query: WFItemPaginationDto,
    ) -> Result<PaginatedResult<WFInvItemBase>, Error> {
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

        let mut mods: Vec<WFInvItemBase> = vec![];
        for item in uniques.into_values() {
            match WFInvItemBase::from_mod(&item, &cache) {
                Ok(mod_item) => mods.push(mod_item),
                Err(e) => {
                    warning(
                        format!("{}:ParseMod", COMPONENT),
                        format!("Failed to parse mod from raw item: {}", e),
                        &LoggerOptions::default(),
                    );
                }
            }
        }

        match query.query {
            FieldChange::Value(query) => {
                mods.retain(|mod_item| mod_item.matches_query(&query));
            }
            _ => {}
        }
        match query.item_types {
            FieldChange::Value(query) => {
                mods.retain(|mod_item| {
                    query
                        .iter()
                        .any(|item_type| mod_item.unique_name.contains(item_type))
                });
            }
            _ => {}
        }

        match &query.sort_by {
            FieldChange::Value(sort_by) => {
                let dir = match &query.sort_direction {
                    FieldChange::Value(dir) => dir,
                    _ => &SortDirection::Asc,
                };
                match sort_by.as_str() {
                    "name" => sort_items(&mut mods, dir, |item| item.name.to_lowercase()),
                    "quantity" => sort_items(&mut mods, dir, |item| item.quantity),
                    "mod_rank" => sort_items(&mut mods, dir, |item| {
                        item.properties.get_property_value("mod_rank", 0i64)
                    }),
                    _ => {}
                }
            }
            _ => {}
        }

        let paginate = paginate(&mods, query.pagination.page, query.pagination.limit);
        Ok(paginate)
    }

    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::wf_inventory::{ItemId, WFInvItemRaw, WarframeRootObject};

    fn raw_mod(unique_name: &str, quantity: i64) -> WFInvItemRaw {
        WFInvItemRaw {
            id: ItemId::default(),
            quantity,
            unique_name: unique_name.to_string(),
            upgrade_fingerprint: None,
            last_added: ItemId::default(),
            xp: 0,
        }
    }

    #[test]
    fn merge_raw_mods_merges_and_skips_rivens_and_non_mods() {
        let mut root = WarframeRootObject::default();
        // Mod listed twice with different quantities -> merged
        root.raw_upgrades = vec![
            raw_mod("/Lotus/Upgrades/Mods/Rifle/WeaponDamageAmountMod", 1),
            raw_mod("/Lotus/Upgrades/Mods/Rifle/WeaponDamageAmountMod", 3),
            // Ranked copy counts separately from the unranked one
            {
                let mut r = raw_mod("/Lotus/Upgrades/Mods/Rifle/WeaponDamageAmountMod", 2);
                r.upgrade_fingerprint = Some("{\"lvl\":10}".to_string());
                r
            },
            // Riven mods and non-mod items are excluded
            raw_mod("/Lotus/Upgrades/Mods/Randomized/LotusRifleRandomModRare", 1),
            raw_mod("/Lotus/Types/Items/MiscItems/Endo", 500),
        ];

        let merged = super::merge_raw_mods(&root);
        assert_eq!(merged.len(), 2);
        let key_rank0 = "/Lotus/Upgrades/Mods/Rifle/WeaponDamageAmountMod|0";
        let key_rank10 = "/Lotus/Upgrades/Mods/Rifle/WeaponDamageAmountMod|10";
        assert_eq!(merged[key_rank0].quantity, 4);
        assert_eq!(merged[key_rank10].quantity, 2);
        assert!(!merged.contains_key("/Lotus/Upgrades/Mods/Randomized/LotusRifleRandomModRare|0"));
        assert!(!merged.contains_key("/Lotus/Types/Items/MiscItems/Endo|0"));
    }
}
