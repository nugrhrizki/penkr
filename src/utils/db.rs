use sqlx::{Postgres, Sqlite, Pool, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};

pub async fn get_pg_pool(max_conn: u32, url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(url)
        .await
}

pub async fn get_sqlite_pool(max_conn: u32, url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(url)
        .await
}
