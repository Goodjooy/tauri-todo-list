pub mod tags;
pub mod todo_item;

#[cfg(test)]
pub mod test_sqlite {
    use once_cell::sync::OnceCell;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    use super::{tags::TagEntity, todo_item::TodoItemEntity};

    pub static SQLITE: OnceCell<SqlitePool> = OnceCell::new();

    pub async fn init() {
        if SQLITE.get().is_none() {
            let pool = SqlitePoolOptions::new()
                .connect(r#"sqlite://./test.sqlite?mode=rwc"#)
                .await
                .expect("start sqlite failure");

            TagEntity::create_table(&pool)
                .await
                .expect("create table failure");
            TodoItemEntity::create_table(&pool)
                .await
                .expect("create table failure");
            SQLITE.set(pool).expect("Unreachable");
        }
    }
}
