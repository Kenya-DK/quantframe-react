use entity::{dto::*, enums::*};

use serde::{Deserialize, Serialize};
use utils::SortDirection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WFItemPaginationDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,

    #[serde(default)]
    pub item_types: FieldChange<Vec<String>>,

    #[serde(default)]
    pub sort_by: FieldChange<String>,

    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
}
