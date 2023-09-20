pub struct Order<'a> {
  client: &'a WarframeMarketClient,
}

impl<'a> Order<'a> {
    
  pub async fn get_my_orders(&self) -> Result<(), AppError>{      
           
  }
  pub async fn create(&self) -> Result<(), AppError>{
      
  }
  pub async fn update(&self) -> Result<(), AppError>{
      
  }
  pub async fn delete(&self) -> Result<(), AppError>{
      
  }  

  pub async fn search(&self) -> Result<(), AppError>{
            
  }
  
}