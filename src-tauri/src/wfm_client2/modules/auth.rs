use crate::{error::AppError, wfm_client2::client::ClientState};
pub struct AuthModule<'a> {
    pub client: &'a ClientState,
}

impl<'a> AuthModule<'a> {}
