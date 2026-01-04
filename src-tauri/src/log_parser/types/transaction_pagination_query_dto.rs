use entity::dto::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPaginationQueryDto {
    pub pagination: PaginationQueryDto,
}
