use entity::{dto::*, enums::*};

use serde::{Deserialize, Serialize};
use utils::SortDirection;
use wf_market::enums::AuctionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeiledRivensPaginationDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,

    #[serde(default)]
    pub auction_type: FieldChange<AuctionType>,

    #[serde(default)]
    pub sort_by: FieldChange<String>,

    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
}
