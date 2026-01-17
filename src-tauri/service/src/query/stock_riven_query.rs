use ::entity::stock_riven::*;

use crate::{paginate_query, ErrorFromExt};
use ::entity::dto::SubType;
use sea_orm::*;
use sea_query::Expr;
use utils::*;

pub struct StockRivenQuery;

static COMPONENT: &str = "StockRivenQuery";
impl StockRivenQuery {
    pub async fn get_all(
        db: &DbConn,
        query: StockRivenPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<stock_riven::Model>, Error> {
        let stmt = query.get_query();

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit)
                .await
                .map_err(|e| e.with_location(get_location!()))?;
        Ok(paginated_result)
    }

    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i64>, Error> {
        let data = StockRivenQuery::get_all(db, StockRivenPaginationQueryDto::new(1, -1)).await?;
        let res = data.results.iter().map(|x| x.id).collect();
        Ok(res)
    }

    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<stock_riven::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:GetById", COMPONENT),
                "Failed to get Stock Riven by ID",
                e,
                get_location!(),
            )
        })
    }
    pub async fn get_by_uuid(
        db: &DbConn,
        uuid: impl Into<String>,
    ) -> Result<Option<stock_riven::Model>, Error> {
        Entity::find()
            .filter(Expr::col(stock_riven::Column::Uuid).eq(uuid.into()))
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:GetByUUID", COMPONENT),
                    "Failed to get Stock Riven by UUID",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<stock_riven::Model>, Error> {
        Entity::find()
            .filter(Expr::col(stock_riven::Column::Id).is_in(ids))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByIds", COMPONENT),
                    "Failed to find Stock Rivens by IDs",
                    e,
                    get_location!(),
                )
            })
    }
    pub async fn find_by_uuids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<stock_riven::Model>, Error> {
        Entity::find()
            .filter(Expr::col(stock_riven::Column::Uuid).is_in(ids))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByUUIDs", COMPONENT),
                    "Failed to find Stock Rivens by UUIDs",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn get_by_riven_name(
        db: &DbConn,
        weapon_url: impl Into<String>,
        mod_name: impl Into<String>,
        sub_type: SubType,
    ) -> Result<Option<stock_riven::Model>, Error> {
        // Ignore case for mod_name
        Entity::find()
            .filter(stock_riven::Column::WfmWeaponUrl.eq(weapon_url.into()))
            .filter(stock_riven::Column::ModName.like(format!("{}", mod_name.into())))
            .filter(stock_riven::Column::SubType.eq(sub_type))
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:GetByRivenName", COMPONENT),
                    "Failed to get Stock Riven by Riven Name",
                    e,
                    get_location!(),
                )
            })
    }
}
