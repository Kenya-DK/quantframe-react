use std::sync::{Arc, Weak};

use entity::{dto::PaginatedResult, enums::FieldChange};
use serde_json::json;
use utils::{log_json_formatted, warning, Error, LoggerOptions, SortDirection};
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
        match query.properties {
            FieldChange::Value(properties) => {
                let riven_type_filter = properties.get_property_value("riven_type", String::new());
                match riven_type_filter.as_str() {
                    "veiled" | "" => {
                        rivens.retain(|riven| riven.riven_type == RivenState::Veiled);
                    }
                    "unveiled" => {
                        rivens.retain(|riven| {
                            riven.riven_type == RivenState::Unveiled
                                || riven.riven_type == RivenState::PreVeiled
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match &query.sort_by {
            FieldChange::Value(sort_by) => {
                let dir = match &query.sort_direction {
                    FieldChange::Value(dir) => dir,
                    _ => &SortDirection::Asc,
                };
                // Only allow sorting by known columns for safety
                match sort_by.as_str() {
                    "disposition" => rivens.sort_by(|a, b| match dir {
                        SortDirection::Asc => a
                            .base
                            .properties
                            .get_property_value("disposition", 0.0f64)
                            .partial_cmp(
                                &b.base.properties.get_property_value("disposition", 0.0f64),
                            )
                            .unwrap_or(std::cmp::Ordering::Equal),
                        SortDirection::Desc => b
                            .base
                            .properties
                            .get_property_value("disposition", 0.0f64)
                            .partial_cmp(
                                &a.base.properties.get_property_value("disposition", 0.0f64),
                            )
                            .unwrap_or(std::cmp::Ordering::Equal),
                    }),
                    "endo" => rivens.sort_by(|a, b| match dir {
                        SortDirection::Asc => a
                            .base
                            .properties
                            .get_property_value("endo_cost", 0)
                            .cmp(&b.base.properties.get_property_value("endo_cost", 0)),
                        SortDirection::Desc => b
                            .base
                            .properties
                            .get_property_value("endo_cost", 0)
                            .cmp(&a.base.properties.get_property_value("endo_cost", 0)),
                    }),
                    "riven_grade" => rivens.sort_by(|a, b| match dir {
                        SortDirection::Asc => a
                            .base
                            .properties
                            .get_property_value("riven_grade", String::new())
                            .cmp(
                                &b.base
                                    .properties
                                    .get_property_value("riven_grade", String::new()),
                            ),
                        SortDirection::Desc => b
                            .base
                            .properties
                            .get_property_value("riven_grade", String::new())
                            .cmp(
                                &a.base
                                    .properties
                                    .get_property_value("riven_grade", String::new()),
                            ),
                    }),
                    _ => {}
                }
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
