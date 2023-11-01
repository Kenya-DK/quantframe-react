use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    cache::client::CacheClient,
};

pub struct OnTradingEvent<'a> {
    pub client: &'a EELogParser,
}

impl<'a> OnTradingEvent<'a> {
}
