use sea_orm::*;
pub async fn paginate_query<E>(
    stmt: Select<E>,
    db: &DbConn,
    page: i64,
    limit: i64,
) -> Result<::entity::dto::pagination::PaginatedResult<E::Model>, DbErr>
where
    E: EntityTrait,
    E::Model: Send + Sync,
{
    let mut page = page.max(1);
    let limit = limit.max(-1);
    let total;
    let total_pages: i64;

    let results = if limit == -1 {
        // No pagination - return all results
        total = stmt.clone().count(db).await? as i64;
        total_pages = 1;
        stmt.all(db).await?
    } else {
        // Paginated results
        let paginator = stmt.paginate(db, limit as u64);
        total = paginator.num_items().await? as i64;

        // Calculate total pages (handle edge case where total is 0)
        total_pages = if total == 0 {
            1
        } else {
            (total as f64 / limit as f64).ceil() as i64
        };

        // Clamp page to valid range
        if page > total_pages {
            page = total_pages;
        }

        paginator.fetch_page((page - 1) as u64).await?
    };

    Ok(::entity::dto::pagination::PaginatedResult::new(
        total,
        limit,
        page,
        total_pages,
        results,
    ))
}
