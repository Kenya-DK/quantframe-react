use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    cache::client::CacheClient,
};


enum Events {
    Conversation,
    World
}
impl Events {
    fn as_str_list(&self) -> &'static Vec<str> {
        match self {
            Events::Conversation => Vec<String>[r"Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F(?<name>.+) to index.+"],
            Events::World => Vec<String>["World"],
        }
    }
}

pub struct OnNewConversationEvent<'a> {
    pub client: &'a EELogParser,
}

impl<'a> OnNewConversationEvent<'a> {
    fn check(input: &str) -> Result<(bool, Option<String>), regex::Error> {
        let pattern = r"Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F(?<name>.+) to index.+";
        let re = Regex::new(pattern)?;

        if let Some(captures) = re.captures(input) {
            let group1 = captures.get(1).map(|m| m.as_str().to_string());
            let result: Option<String> =
                group1.map(|s| s.chars().filter(|c| c.is_ascii()).collect());
            return Ok((true, result));
        }

        Ok((false, None))
    }
}
