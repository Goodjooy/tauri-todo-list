use sea_query::{ColumnDef, Expr, Iden, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::{query, query_as_with, query_with, FromRow, SqlitePool};
use tap::{Conv, Pipe};

use crate::database::priority::Priority;

pub struct TodoItemEntity;

#[derive(Debug, Iden)]
pub enum TodoItem {
    Table,
    Id,
    Message,
    Priority,
    Done,
}
impl TodoItem {
    fn get_columns() -> [Self; 4] {
        [Self::Id, Self::Message, Self::Priority, Self::Done]
    }
    fn columns_without_id() -> [Self; 3] {
        [Self::Message, Self::Priority, Self::Done]
    }

    fn get_table() -> Self {
        Self::Table
    }

    fn into_col_expr(self) -> Expr {
        Expr::col(self)
    }
}

#[derive(Debug, FromRow, PartialEq)]
pub struct TodoItemModel {
    pub id: i32,
    pub message: String,
    pub priority: Priority,
    pub done: bool,
}

impl TodoItemEntity {
    pub async fn create_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let stet = Table::create()
            .table(TodoItem::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TodoItem::Id)
                    .integer()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(TodoItem::Message).text().not_null())
            .col(
                ColumnDef::new(TodoItem::Priority)
                    .small_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TodoItem::Done)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .build(SqliteQueryBuilder);

        query(&stet).execute(pool).await?;
        Ok(())
    }

    pub async fn fetch_all(
        pool: &SqlitePool,
        limit: impl Into<Option<u64>>,
    ) -> Result<Vec<TodoItemModel>, sqlx::Error> {
        let (stet, values) = Query::select()
            .columns(TodoItem::get_columns())
            .from(TodoItem::get_table())
            .pipe(|query| {
                if let Some(limit) = limit.into() {
                    query.limit(limit)
                } else {
                    query
                }
            })
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&stet, values).fetch_all(pool).await
    }

    pub async fn find_all_by_id(
        pool: &SqlitePool,
        ids: impl IntoIterator<Item = i32>,
    ) -> Result<Vec<TodoItemModel>, sqlx::Error> {
        let (stet, values) = Query::select()
            .columns(TodoItem::get_columns())
            .from(TodoItem::get_table())
            .and_where(TodoItem::Id.into_col_expr().is_in(ids))
            .build_sqlx(SqliteQueryBuilder);

        query_as_with(&stet, values).fetch_all(pool).await
    }

    pub async fn save(
        pool: &SqlitePool,
        message: String,
        priority: Priority,
        done: impl Into<Option<bool>>,
    ) -> Result<i32, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(TodoItem::get_table())
            .columns(TodoItem::columns_without_id())
            .values_panic([
                message.into(),
                priority.into(),
                done.conv::<Option<bool>>().unwrap_or(false).into(),
            ])
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values)
            .execute(pool)
            .await
            .map(|result| result.last_insert_rowid() as i32)
    }

    pub async fn update_message(
        pool: &SqlitePool,
        id: i32,
        message: String,
    ) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::update()
            .table(TodoItem::get_table())
            .value(TodoItem::Message, message.into())
            .and_where(TodoItem::Id.into_col_expr().eq(id))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;
        Ok(())
    }

    pub async fn update_priority(
        pool: &SqlitePool,
        id: i32,
        priority: Priority,
    ) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::update()
            .table(TodoItem::get_table())
            .value(TodoItem::Priority, priority.into())
            .and_where(TodoItem::Id.into_col_expr().eq(id))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;
        Ok(())
    }

    pub async fn revert_done(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::update()
            .table(TodoItem::get_table())
            .value_expr(TodoItem::Done, TodoItem::Done.into_col_expr().not())
            .and_where(TodoItem::Id.into_col_expr().eq(id))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;
        Ok(())
    }

    pub async fn remove(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TodoItem::get_table())
            .and_where(TodoItem::Id.into_col_expr().eq(id))
            .build_sqlx(SqliteQueryBuilder);

        query_with(&sql, values).execute(pool).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test_todo_item {

    use sqlx::SqlitePool;

    use crate::database::{
        models::test_sqlite::{init, SQLITE},
        priority::Priority,
    };

    use super::{TodoItemEntity, TodoItemModel};

    async fn get_model(pool: &SqlitePool, id: i32) -> TodoItemModel {
        let model = TodoItemEntity::find_all_by_id(pool, [id])
            .await
            .unwrap()
            .remove(0);

        dbg!(model)
    }

    #[tokio::test]
    async fn test_create() {
        init().await;
        let pool = SQLITE.get().unwrap();
        let idx = TodoItemEntity::save(pool, "Foo".into(), Priority::VeryHigh, None)
            .await
            .unwrap();

        let model = get_model(pool, idx).await;

        assert_eq!(
            model,
            TodoItemModel {
                id: idx,
                message: "Foo".into(),
                priority: Priority::VeryHigh,
                done: false
            }
        );
    }

    #[tokio::test]
    async fn test_rev() {
        init().await;
        let pool = SQLITE.get().unwrap();

        TodoItemEntity::revert_done(pool, 1).await.unwrap();

        let model = get_model(pool, 1).await;

        assert_eq!(
            model,
            TodoItemModel {
                id: 1,
                message: "Foo".into(),
                priority: Priority::VeryHigh,
                done: true
            }
        )
    }
}
