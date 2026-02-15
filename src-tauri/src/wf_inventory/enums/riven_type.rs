use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum RivenType {
    Veiled,
    PreVeiled,
    UnVeiled,
}
