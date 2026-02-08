use std::sync::{atomic::Ordering, Arc, Weak};

use entity::{dto::PriceHistory, enums::*, stock_riven::*};
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};
use utils::{average_filtered_lowest_prices, get_location, info, warning, Error, LoggerOptions};
use wf_market::{
    enums::{AuctionType, Polarity, StatusType},
    types::{
        AuctionFilter, AuctionList, AuctionWithOwner, CreateAuctionItem, CreateAuctionParams,
        ItemAttribute, UpdateAuctionParams,
    },
};
static COMPONENT: &str = "LiveScraper:RivenModule";
use crate::alecaframe::AlecaframeState;

#[derive(Debug)]
pub struct RivenModule {
    client: Weak<AlecaframeState>,
}

impl RivenModule {
    /**
     * Creates a new `RivenModule` with an empty item list.
     * The `client` parameter is an `Arc<AlecaframeState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<AlecaframeState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
