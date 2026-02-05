use std::{
    collections::HashSet,
    sync::{atomic::Ordering, Arc, Weak},
};

use entity::{dto::PriceHistory, enums::stock_status::StockStatus};
use serde_json::json;
use service::{StockItemMutation, WishListMutation};
use utils::*;
use wf_market::{
    enums::{OrderType, StatusType},
    errors::ApiError,
    types::{Order, OrderList, OrderWithUser},
};

use crate::{
    app::{client::AppState, Settings},
    cache::types::{CacheTradableItem, ItemPriceInfo},
    utils::{ErrorFromExt, OrderListExt},
};
use crate::{
    enums::TradeMode, live_scraper::*, send_event, types::*, utils::modules::states,
    utils::SubTypeExt, DATABASE,
};

#[derive(Debug)]
pub struct ItemModule {
    client: Weak<AlecaframeState>,
}

impl ItemModule {
    /**
     * Creates a new `ItemModule` with an empty item list.
     * The `client` parameter is an `Arc<AlecaframeState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<AlecaframeState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
