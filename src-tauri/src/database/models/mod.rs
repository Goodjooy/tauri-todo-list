pub mod tag_item_bind;
pub mod tags;
pub mod todo_item;

#[cfg(test)]
pub mod test_sqlite {
    use once_cell::sync::OnceCell;
    use sqlx::SqlitePool;

    use crate::database::init_sqlite;

    pub static SQLITE: OnceCell<SqlitePool> = OnceCell::new();

    pub async fn init() {
        if SQLITE.get().is_none() {
            let pool = init_sqlite().await;
            SQLITE.set(pool).expect("Unreachable");
        }
    }
}
