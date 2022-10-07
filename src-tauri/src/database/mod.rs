use once_cell::sync::OnceCell;
use sea_query::Iden;
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};

use self::models::tags::TagEntity;

pub mod models;

static SQLITE: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_sqlite() {
    if SQLITE.get().is_none() {
        let pool = SqlitePoolOptions::new()
            .connect(r#"sqlite://./app.sqlite?mode=rwc"#)
            .await
            .expect("start sqlite failure");

        TagEntity::create_table(&pool)
            .await
            .expect("create table failure");
        // init tables
        SQLITE.set(pool).expect("Unreachable");
    }
}

pub fn get_sqlite_pool() -> &'static SqlitePool {
    SQLITE.get().expect("Sqlite Connection not Init")
}

#[derive(Debug, Iden)]
pub struct Count;

#[derive(Debug, FromRow)]
pub struct CountModel {
    count: i64,
}

impl CountModel {
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
