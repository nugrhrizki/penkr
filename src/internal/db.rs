use actix_web::{HttpResponse, Responder};

use crate::{internal::de::QueryResult, utils::db::get_pg_pool};

#[derive(Debug)]
pub struct DBX {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl DBX {
    pub async fn new(max_connections: u32, db_url: &str) -> Result<Self, impl Responder> {
        let pool = get_pg_pool(max_connections, db_url).await;
        if let Ok(pool) = pool {
            Ok(Self { pool })
        } else {
            Err(HttpResponse::InternalServerError().body("Failed to connect to Postgres"))
        }
    }

    pub async fn select(&self, query: &str) -> Result<Vec<QueryResult>, sqlx::Error> {
        let users = sqlx::query_as::<_, QueryResult>(&query)
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn select_one(&self, query: &str) -> Result<QueryResult, sqlx::Error> {
        let user = sqlx::query_as::<_, QueryResult>(&query)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn insert(&self, query: &str) -> Result<Vec<QueryResult>, sqlx::Error> {
        let user = sqlx::query_as::<_, QueryResult>(&query)
            .fetch_all(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn insert_one(&self, query: &str) -> Result<QueryResult, sqlx::Error> {
        let user = sqlx::query_as::<_, QueryResult>(&query)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn update(&self, query: &str) -> Result<QueryResult, sqlx::Error> {
        let user = sqlx::query_as::<_, QueryResult>(&query)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn delete(&self, query: &str) -> Result<QueryResult, sqlx::Error> {
        let user = sqlx::query_as::<_, QueryResult>(&query)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
}
