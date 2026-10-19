use ::entity::syndicate_item::*;

use sea_orm::{sea_query::Expr, *};

use crate::{paginate_query, ErrorFromExt};
use utils::*;

pub struct SyndicateItemQuery;

static COMPONENT: &str = "SyndicateItemQuery";
impl SyndicateItemQuery {
    pub async fn get_all(
        db: &DbConn,
        query: SyndicateItemPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<Model>, Error> {
        let stmt = query.get_query();

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit)
                .await
                .map_err(|e| e.with_location(get_location!()))?;
        Ok(paginated_result)
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<syndicate_item::Model>, Error> {
        Entity::find()
            .filter(syndicate_item::Column::WfmUrl.eq(url_name))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByUrlName", COMPONENT),
                    "Failed to find Syndicate Items by URL name",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<syndicate_item::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:FindById", COMPONENT),
                "Failed to find Syndicate Item by ID",
                e,
                get_location!(),
            )
        })
    }
    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<i64>,
    ) -> Result<Vec<syndicate_item::Model>, Error> {
        Entity::find()
            .filter(Expr::col(syndicate_item::Column::Id).is_in(ids))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByIds", COMPONENT),
                    "Failed to find Syndicate Items by IDs",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_url_name_and_sub_type(
        db: &DbConn,
        url_name: &str,
        sub_type: Option<SubType>,
    ) -> Result<Option<syndicate_item::Model>, Error> {
        let items = SyndicateItemQuery::find_by_url_name(db, url_name).await?;
        for item in items {
            if SubType::matches(&item.sub_type, &sub_type) {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}
