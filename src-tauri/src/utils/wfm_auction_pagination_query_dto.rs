use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use entity::{dto::*, enums::*};
use serde_json::{json, Value};
use utils::{filters_by, get_location, group_by, Error};
use wf_market::{
    enums::OrderType,
    types::{item, Order},
};

use crate::{
    app::client::AppState,
    cache::client::CacheState,
    enums::*,
    helper::paginate,
    live_scraper::LiveScraperState,
    utils::{ErrorFromExt, OrderExt, OrderListExt},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfmAuctionPaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,
}
