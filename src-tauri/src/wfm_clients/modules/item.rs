pub struct Item<'a> {
  client: &'a WarframeMarketClient,
}

impl<'a> Item<'a> {
    
  pub async fn get_all_items(&self) -> Result<(), AppError>{      
           
  }
  pub async fn get_item(&self) -> Result<(), AppError>{
      
  }
  pub async fn get_item_orders(&self) -> Result<(), AppError>{
      
  }
}