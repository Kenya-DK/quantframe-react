pub struct Riven<'a> {
  client: &'a WarframeMarketClient,
  auction: &'a Auction,
}

impl<'a> Riven<'a> {
    
  // Static methods for Riven struct here
  pub async fn get_all_items(&self) -> Result<(), AppError>{      
           
  }
  pub async fn get_all_attributes(&self) -> Result<(), AppError>{      
           
  }
  // End of static methods for Riven struct

  pub async fn get_orders(&self) -> Result<(), AppError>{      
           
      
  }
}