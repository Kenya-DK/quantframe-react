use entity::{dto::*, enums::*};
use utils::{OperationSet, SortDirection};
use wf_market::enums::OrderType;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfmOrderPaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,

    #[serde(default)]
    pub order_type: FieldChange<OrderType>,

    #[serde(default)]
    pub sort_by: FieldChange<String>,

    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,

    #[serde(default)]
    pub operations: FieldChange<Vec<String>>,
}
