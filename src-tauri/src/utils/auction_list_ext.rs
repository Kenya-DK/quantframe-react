use utils::Error;
use wf_market::types::{Auction, AuctionList, AuctionWithOwner};

use crate::{cache::client::CacheState, utils::AuctionExt};

/// Extension trait for auction list
pub trait AuctionListExt {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
}

impl AuctionListExt for AuctionList<Auction> {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        for auction in self.auctions.iter_mut() {
            auction.apply_item_info(cache)?;
        }

        Ok(())
    }
}

/// Extension trait for auction list
pub trait AuctionWithOwnerListExt {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
}

impl AuctionWithOwnerListExt for AuctionList<AuctionWithOwner> {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        for auction in self.auctions.iter_mut() {
            auction.auction.apply_item_info(cache)?;
        }

        Ok(())
    }
}
