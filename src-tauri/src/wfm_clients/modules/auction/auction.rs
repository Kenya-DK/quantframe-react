pub struct Auction<'a> {
  client: &'a WarframeMarketClient,
}

impl<'a> Auction<'a> {
    
  pub fn riven(&self) -> Riven {
    Riven {
        client: self.client,
        auction: self,
    }
  }
  
}