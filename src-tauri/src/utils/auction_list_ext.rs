use utils::Error;
use wf_market::types::{Auction, AuctionList};

use crate::{cache::client::CacheState, enums::FindBy, utils::AuctionExt};

/// Extension trait for auction list
pub trait AuctionListExt {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
}

impl AuctionListExt for AuctionList<Auction> {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        for auction in self.auctions.iter_mut() {
            match cache.riven().get_riven_by(FindBy::new(
                crate::enums::FindByType::Url,
                &auction.item.weapon_url_name,
            )) {
                Ok(item) => {
                    if let Some(item) = item {
                        auction.update_details(
                            auction
                                .get_details()
                                .set_item_name(&item.name)
                                .set_image_url(&item.wfm_icon),
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get item info for auction {}: {}", auction.id, e);
                }
            }
        }

        Ok(())
    }
}
