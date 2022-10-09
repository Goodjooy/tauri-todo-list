use sea_query::{ColumnDef, ColumnType, Expr, Iden, Query, SqliteQueryBuilder, Table};
use sqlx::{query, query_as_with, query_with, FromRow, SqlitePool};
use tap::Pipe;

use sea_query_binder::{SqlxBinder, SqlxValues};
pub struct TagEntity;

#[derive(Debug, Iden)]
enum Tag {
    Table,
    Id,
    Value,
}

#[derive(Debug, FromRow)]
pub struct TagModel {
    pub id: i32,
    pub value: String,
}

impl TagEntity {
    pub async fn create_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let stat = Table::create()
            .table(Tag::Table)
            .if_not_exists()
            .col(
                ColumnDef::new_with_type(Tag::Id, ColumnType::Integer(None))
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new_with_type(Tag::Value, ColumnType::Text)
                    .not_null()
                    .unique_key(),
            )
            .build(SqliteQueryBuilder);

        query(&stat).execute(pool).await?;
        Ok(())
    }

    fn query_by_tag(tag: &impl AsRef<str>) -> (String, SqlxValues) {
        Query::select()
            .columns([Tag::Id])
            .from(Tag::Table)
            .and_where(Expr::col(Tag::Value).eq(tag.as_ref()))
            .to_owned()
            .build_sqlx(SqliteQueryBuilder)
    }

    pub async fn fetch_all(
        pool: &SqlitePool,
        limit: Option<u64>,
    ) -> Result<Vec<TagModel>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([Tag::Id, Tag::Value])
            .from(Tag::Table)
            .pipe(|q| {
                if let Some(limit) = limit {
                    q.limit(limit)
                } else {
                    q
                }
            })
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&sql, values).fetch_all(pool).await
    }

    pub async fn find_all_by_id(
        pool: &SqlitePool,
        ids: impl IntoIterator<Item = i32>,
    ) -> Result<Vec<TagModel>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([Tag::Id, Tag::Value])
            .from(Tag::Table)
            .cond_where(Expr::col(Tag::Id).is_in(ids))
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&sql, values).fetch_all(pool).await
    }
    pub async fn find_by_id(pool: &SqlitePool, id: i32) -> Result<String, sqlx::Error> {
        let (sql, values) = Query::select()
            .column(Tag::Value)
            .from(Tag::Table)
            .and_where(Expr::col(Tag::Id).eq(id))
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&sql, values)
            .fetch_one(pool)
            .await
            .map(|(s,)| s)
    }
    pub async fn remove(pool: &SqlitePool, tag: &impl AsRef<str>) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(Tag::Table)
            .and_where(Expr::col(Tag::Value).eq(tag.as_ref()))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await.map(|_| ())
    }

    pub async fn get_id(pool: &SqlitePool, tag: &impl AsRef<str>) -> Result<i32, sqlx::Error> {
        let (sql, values) = Self::query_by_tag(tag);
        query_as_with(&sql, values)
            .fetch_one(pool)
            .await
            .map(|(resp,)| resp)
    }

    pub async fn edit(
        pool: &SqlitePool,
        id: i32,
        tag: &impl AsRef<str>,
    ) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::update()
            .table(Tag::Table)
            .value(Tag::Value, tag.as_ref().into())
            .and_where(Expr::col(Tag::Id).eq(id))
            .build_sqlx(SqliteQueryBuilder);
        query_with(&sql, values).execute(pool).await.map(|_| ())
    }

    pub async fn new(pool: &SqlitePool, tag: impl AsRef<str>) -> Result<i32, sqlx::Error> {
        // search is the tag exist
        let (query, values) = Self::query_by_tag(&tag);

        // the tag not exist insert it
        if let Some((id,)) =
            query_as_with(&query, values)
                .fetch_one(pool)
                .await
                .pipe(|v| match v {
                    Ok(v) => Ok(Some(v)),
                    Err(sqlx::Error::RowNotFound) => Ok(None),
                    Err(err) => Err(err),
                })?
        {
            Ok(id)
        } else {
            let (query, values) = Query::insert()
                .into_table(Tag::Table)
                .columns([Tag::Value])
                .values_panic([(tag.as_ref()).into()])
                .build_sqlx(SqliteQueryBuilder);

            let v = query_with(&query, values).execute(pool).await?;
            Ok(v.last_insert_rowid() as i32)
        }
    }
}

#[cfg(test)]
mod test_tag {

    use crate::database::models::test_sqlite::{init, SQLITE};

    use super::{TagEntity, TagModel};


    #[tokio::test]
    async fn test_insert() {
        init().await;
        let data = TagModel {
            id: 1,
            value: "abccc".to_string(),
        };

        let v = TagEntity::new(SQLITE.get().unwrap(), &data.value)
            .await
            .expect("Failure save data");

        println!("{v}")
    }

    #[tokio::test]
    async fn test_edit() {
        init().await;
        let pool = SQLITE.get().unwrap();

        TagEntity::edit(pool, 1, &"ccb").await.unwrap();
        let r = TagEntity::fetch_all(pool, None).await.unwrap();
        println!("{r:?}")
    }
}
