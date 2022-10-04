use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

pub async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/visorm")
        .await
}
