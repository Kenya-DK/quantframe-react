pub mod alecaframe;
pub use alecaframe::*;

pub mod file;
pub use file::*;

pub mod helpers;

pub mod profile;
pub use profile::*;

pub mod traits;
pub use traits::*;

use crate::wf_inventory::WarframeRootObject;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use utils::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WFInventorySource {
    None,
    Profile(WFInvProfileSource),
    Alecaframe(WFInvAlecaframeSource),
    File(WFInvFileSource),
}

impl InventorySource for WFInventorySource {
    fn update(&self, root: &Arc<Mutex<WarframeRootObject>>) -> Result<(), Error> {
        match self {
            WFInventorySource::None => Ok(()),
            WFInventorySource::Profile(source) => source.update(root),
            WFInventorySource::Alecaframe(source) => source.update(root),
            WFInventorySource::File(source) => source.update(root),
        }
    }

    fn start(&self, root: &Arc<Mutex<WarframeRootObject>>) {
        match self {
            WFInventorySource::None => {}
            WFInventorySource::Profile(source) => source.start(root),
            WFInventorySource::Alecaframe(source) => source.start(root),
            WFInventorySource::File(source) => source.start(root),
        }
    }

    fn stop(&self) {
        match self {
            WFInventorySource::None => {}
            WFInventorySource::Profile(source) => source.stop(),
            WFInventorySource::Alecaframe(source) => source.stop(),
            WFInventorySource::File(source) => source.stop(),
        }
    }

    fn validate(&self) -> Result<(), Error> {
        match self {
            WFInventorySource::None => Ok(()),
            WFInventorySource::Profile(source) => source.validate(),
            WFInventorySource::Alecaframe(source) => source.validate(),
            WFInventorySource::File(source) => source.validate(),
        }
    }
}

impl Display for WFInventorySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WFInventorySource::None => write!(f, "None"),
            WFInventorySource::Profile(_) => write!(f, "Profile"),
            WFInventorySource::Alecaframe(_) => write!(f, "Alecaframe"),
            WFInventorySource::File(_) => write!(f, "File"),
        }
    }
}
