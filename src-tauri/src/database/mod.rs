pub mod priority;
use sea_query::Iden;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use self::models::tags::TagEntity;

pub mod models;

pub async fn init_sqlite() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect(r#"sqlite://./app.sqlite?mode=rwc"#)
        .await
        .expect("start sqlite failure");

    // init tables
    TagEntity::create_table(&pool)
        .await
        .expect("create table failure");

    return pool;
}

#[derive(Debug, Iden)]
pub struct Count;
