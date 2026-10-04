use crate::wf_inventory::WarframeRootObject;
use std::sync::{Arc, Mutex};
use utils::*;

use serde::{Deserialize, Serialize};

use super::{helpers::*, traits::InventorySource};

const PROFILE_URL: &str = "https://api.warframe.com/cdn/getProfileViewingData.php";
const COMPONENT: &str = "WFInvProfile";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WFInvProfileSource {
    pub id: String,
}

impl WFInvProfileSource {
    pub fn get_url(&self, id: impl Into<String>) -> String {
        format!("{PROFILE_URL}?playerId={}", id.into())
    }

    async fn fetch(&self) -> Result<WarframeRootObject, Error> {
        let resp = reqwest::get(&self.get_url(&self.id)).await.map_err(|e| {
            Error::new(
                format!("{COMPONENT}:Fetch:Request"),
                format!("Failed to fetch profile: {e:?}"),
                get_location!(),
            )
        })?;
        let text = resp.text().await.map_err(|e| {
            Error::new(
                format!("{COMPONENT}:Fetch:Read"),
                format!("Failed to read response: {e:?}"),
                get_location!(),
            )
        })?;
        info(
            format!("{COMPONENT}:Fetch"),
            "Response received",
            &LoggerOptions::default(),
        );
        parse_lastdata(&text)
    }
}

impl InventorySource for WFInvProfileSource {
    fn update(&self, root: &Arc<Mutex<WarframeRootObject>>) -> Result<(), Error> {
        let parsed = block_on_async(self.fetch())?;

        let mut root = root.lock().map_err(|_| {
            Error::new(
                "WFInvProfileSource:Lock",
                "Root mutex poisoned",
                get_location!(),
            )
        })?;
        info(
            format!("{COMPONENT}:Update:Complete"),
            "Profile data updated",
            &LoggerOptions::default(),
        );
        *root = parsed;
        Ok(())
    }

    fn start(&self, root: &Arc<Mutex<WarframeRootObject>>) {
        if let Err(e) = self.update(root) {
            e.log("WFInventoryState.log").with_location(get_location!());
        }
    }

    fn validate(&self) -> Result<(), Error> {
        if self.id.trim().is_empty() {
            return Err(Error::new(
                format!("{COMPONENT}:Validate"),
                "Profile ID is empty",
                get_location!(),
            ));
        }
        Ok(())
    }
}
