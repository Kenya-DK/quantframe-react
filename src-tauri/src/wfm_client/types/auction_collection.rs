use super::{auction::Auction, auction_owner::AuctionOwner};
use entity::stock::riven::attribute::RivenAttribute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuctionCollection<T> {
    #[serde(rename = "auctions")]
    pub auctions: Vec<Auction<T>>,
}
impl<T> AuctionCollection<T> {
    pub fn new(auctions: Vec<Auction<T>>) -> Self {
        AuctionCollection { auctions }
    }
    pub fn sort_by_platinum(&mut self) {
        self.auctions
            .sort_by(|a, b| a.starting_price.cmp(&b.starting_price));
    }
    pub fn total(&self) -> i64 {
        self.auctions.len() as i64
    }

    pub fn get_auction_by_id(&self, id: &str) -> Option<Auction<T>>
    where
        T: Clone,
    {
        self.auctions
            .iter()
            .find(|auction| auction.id == id)
            .cloned()
    }

    pub fn filter_by_type(&self, auction_type: &str) -> AuctionCollection<T>
    where
        T: Clone,
    {
        let auctions = self
            .auctions
            .iter()
            .filter(|auction| auction.item.item_type == auction_type)
            .cloned()
            .collect::<Vec<_>>();

        AuctionCollection::new(auctions)
    }

    pub fn lowest_auction(&self) -> Option<Auction<T>>
    where
        T: Clone,
    {
        self.auctions
            .iter()
            .min_by(|a, b| a.starting_price.cmp(&b.starting_price))
            .cloned()
    }
    pub fn lowest_price(&self) -> i64
    where
        T: Clone,
    {
        let auction = self.lowest_auction();
        if auction.is_none() {
            return 0;
        }
        auction.unwrap().starting_price
    }

    pub fn highest_auction(&self) -> Option<Auction<T>>
    where
        T: Clone,
    {
        self.auctions
            .iter()
            .max_by(|a, b| a.starting_price.cmp(&b.starting_price))
            .cloned()
    }
    pub fn highest_price(&self) -> i64
    where
        T: Clone,
    {
        let auction = self.highest_auction();
        if auction.is_none() {
            return 0;
        }
        auction.unwrap().starting_price
    }

    pub fn prices(&self) -> Vec<i64>
    where
        T: Clone,
    {
        self.auctions
            .iter()
            .map(|auction| auction.starting_price)
            .collect()
    }
}
impl AuctionCollection<AuctionOwner> {
    pub fn filter_by_username(
        &self,
        username: &str,
        exclude: bool,
    ) -> AuctionCollection<AuctionOwner> {
        let auctions = self
            .auctions
            .iter()
            .filter(|auction| {
                // Convert auction.owner to AuctionOwner
                let owner: AuctionOwner = auction.owner.clone().into();
                if exclude {
                    // Exclude auctions by the given username
                    owner.ingame_name != username
                } else {
                    // Include only auctions by the given username
                    owner.ingame_name == username
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        AuctionCollection::new(auctions)
    }
    pub fn remove_offline_auctions(&self) -> AuctionCollection<AuctionOwner> {
        let auctions = self
            .auctions
            .iter()
            .filter(|auction| {
                // Convert auction.owner to AuctionOwner
                let owner: AuctionOwner = auction.owner.clone().into();
                if owner.status == "ingame" {
                    return true;
                } else {
                    return false;
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        AuctionCollection::new(auctions)
    }
    pub fn calculate__riven_similarity(
        &self,
        attributes: Vec<RivenAttribute>,
    ) -> AuctionCollection<AuctionOwner> {
        let mut auctions = self.auctions.clone();
        for auction in auctions.iter_mut() {
            auction.set_similarity_riven(attributes.clone());
        }
        AuctionCollection::new(auctions)
    }
}
