use sea_query::{ColumnDef, Condition, Expr, Iden, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::{query, query_as_with, query_with, FromRow, SqlitePool};
use tap::Pipe;

pub struct BindEntity;

#[derive(Debug, Iden)]
pub enum TagItemBind {
    Table,
    TagId,
    ItemId,
}
#[derive(Debug, FromRow)]
pub struct BindModel {
    pub tag_id: i32,
    pub item_id: i32,
}

impl BindModel {
    pub fn new(tag_id: i32, item_id: i32) -> Self {
        Self { tag_id, item_id }
    }
}

impl BindEntity {
    pub async fn create_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let sql = Table::create()
            .table(TagItemBind::Table)
            .if_not_exists()
            .col(ColumnDef::new(TagItemBind::TagId).integer().primary_key())
            .col(ColumnDef::new(TagItemBind::ItemId).integer().primary_key())
            .build(SqliteQueryBuilder);

        query(&sql).execute(pool).await?;
        Ok(())
    }
    pub async fn save_all<I: IntoIterator<Item = BindModel>>(
        pool: &SqlitePool,
        peers: I,
    ) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(TagItemBind::Table)
            .columns([TagItemBind::TagId, TagItemBind::ItemId])
            .pipe(|query| {
                peers.into_iter().for_each(|BindModel { tag_id, item_id }| {
                    query.values_panic([tag_id.into(), item_id.into()]);
                });
                query
            })
            .build_sqlx(SqliteQueryBuilder);
        query_with(&sql, values).execute(pool).await?;
        Ok(())
    }
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<BindModel>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([TagItemBind::TagId, TagItemBind::ItemId])
            .from(TagItemBind::Table)
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&sql, values).fetch_all(pool).await
    }

    pub async fn fetch_all_tag_id(
        pool: &SqlitePool,
        item_id: i32,
    ) -> Result<Vec<i32>, sqlx::Error> {
        let (sql, values) = Query::select()
            .column(TagItemBind::TagId)
            .from(TagItemBind::Table)
            .and_where(Expr::col(TagItemBind::ItemId).eq(item_id))
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&sql, values)
            .fetch_all(pool)
            .await
            .map(|v| v.into_iter().map(|(id,)| id).collect())
    }

    pub async fn fetch_all_item_id(
        pool: &SqlitePool,
        tag_id: i32,
    ) -> Result<Vec<i32>, sqlx::Error> {
        let (sql, values) = Query::select()
            .column(TagItemBind::ItemId)
            .from(TagItemBind::Table)
            .and_where(Expr::col(TagItemBind::TagId).eq(tag_id))
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&sql, values)
            .fetch_all(pool)
            .await
            .map(|v| v.into_iter().map(|(id,)| id).collect())
    }

    pub async fn remove(pool: &SqlitePool, tag_id: i32, item_id: i32) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TagItemBind::Table)
            .cond_where(
                Condition::all()
                    .add(Expr::col(TagItemBind::TagId).eq(tag_id))
                    .add(Expr::col(TagItemBind::ItemId).eq(item_id)),
            )
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;

        Ok(())
    }

    pub async fn remove_bind_tag_id(pool: &SqlitePool, tag_id: i32) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TagItemBind::Table)
            .and_where(Expr::col(TagItemBind::TagId).eq(tag_id))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;
        Ok(())
    }
    pub async fn remove_bind_item_id(pool: &SqlitePool, item_id: i32) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TagItemBind::Table)
            .and_where(Expr::col(TagItemBind::ItemId).eq(item_id))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;
        Ok(())
    }
}
