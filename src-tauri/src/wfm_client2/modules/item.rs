use crate::{error::AppError, wfm_client2::client::ClientState};

pub struct ItemModule<'a> {
  pub client: &'a ClientState,
}

impl<'a> ItemModule<'a> {
    pub async fn get_all_items(&self) -> Result<(), AppError> {
      Ok(())
    }
    pub async fn get_item(&self) -> Result<(), AppError> {
      Ok(())
    }
    pub async fn get_item_orders(&self) -> Result<(), AppError> {
      Ok(())
    }
}
