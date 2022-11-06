use sqlx::{Postgres, QueryBuilder};

use crate::{internal::de::QueryResult, utils::db::get_pg_pool};

#[derive(Debug)]
pub struct DBX {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

pub struct DBQuery {
    pub table: String,
    pub columns: Option<String>,
    pub r#where: Option<String>,
    pub order_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl DBX {
    pub async fn new(max_connections: u32, db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = get_pg_pool(max_connections, db_url).await?;
        Ok(Self { pool })
    }

    pub async fn disconnect(&self) -> Result<(), sqlx::Error> {
        Ok(self.pool.close().await)
    }

    pub async fn raw(&self, query: &str) -> Result<Vec<QueryResult>, sqlx::Error> {
        let result = sqlx::query_as::<_, QueryResult>(query)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }

    pub async fn select(&self, query: &DBQuery) -> Result<Vec<QueryResult>, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("select ");

        let query = self
            .query_filter(query, &mut query_builder)
            .build_query_as::<QueryResult>()
            .fetch_all(&self.pool)
            .await?;

        Ok(query)
    }

    pub async fn select_one(&self, query: &DBQuery) -> Result<QueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("select ");

        let query = self
            .query_filter(query, &mut query_builder)
            .build_query_as::<QueryResult>()
            .fetch_one(&self.pool)
            .await?;

        Ok(query)
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

    fn query_filter<'a>(
        &self,
        query: &'a DBQuery,
        query_builder: &'a mut QueryBuilder<'a, Postgres>,
    ) -> &'a mut QueryBuilder<'a, Postgres> {
        match &query.columns {
            Some(columns) => {
                query_builder.push(columns);
            }
            None => {
                query_builder.push("*");
            }
        }

        query_builder.push(" from ");
        query_builder.push(query.table.as_str());

        if let Some(r#where) = &query.r#where {
            query_builder.push(" where ");
            query_builder.push(r#where.as_str());
        }

        if let Some(order_by) = &query.order_by {
            query_builder.push(" order by ");
            query_builder.push_bind(order_by.as_str());
        }

        if let Some(order) = &query.order {
            query_builder.push(" ");
            query_builder.push_bind(order.as_str());
        }

        if let Some(limit) = &query.limit {
            query_builder.push(" limit ");
            query_builder.push_bind(limit);
        }

        if let Some(offset) = &query.offset {
            query_builder.push(" offset ");
            query_builder.push_bind(offset);
        }

        query_builder
    }
}
