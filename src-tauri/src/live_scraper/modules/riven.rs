use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{auth::AuthState, error::AppError, live_scraper::client::LiveScraperClient, logger};
pub struct RivenModule<'a> {
    pub client: &'a LiveScraperClient,
}

impl<'a> RivenModule<'a> {
    pub async fn check_stock(&self) -> Result<(), AppError> {
        let db = self.client.db.lock()??.clone()
        let wfm = self.client.wfm.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        logger::info_con("RivenModule", "Run riven module");
        let stockrivens = db.stock_riven().get_rivens().await?;
        let my_auctions = wfm.auction().get_my_auctions().await?;
        let my_rivens = my_auctions
            .iter()
            .filter(|a| a.item_type == "riven")
            .collect::<Vec<_>>();
        for riven in stockrivens {
            // Check if riven is in my auctions and update
            let auction = my_rivens
                .iter()
                .find(|a| a.id == riven.order_id)
                .unwrap();
            
            let live_auctions = wfm.auction().search("riven", riven.weapon_url, None, None, None, None, None, None, None, Some("direct"), Some("price_asc")).await?;
            let live_auctions = live_auctions.iter().filter(|a| a.owner.ingame_name != auth.ingame_name && a.closed == false && a.visible).unwrap();
            if live_auctions.len() == 0 {
                // No live auctions
                continue;
            }
            // Get the lowest price
            let lowest_price = live_auctions.get(0).unwrap().platinum;
            let profit = auction.platinum - lowest_price;
            // Check if profit is greater than 25
            if profit > 25 {
               
            } else {

            }

        }
        Ok(())
    }
}
