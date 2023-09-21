use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{auth::AuthState, error::AppError, logger, database2::client::DBClient};
pub struct TransactionModule<'a> {
    pub client: &'a DBClient,
}

impl<'a> TransactionModule<'a> {
    
}
