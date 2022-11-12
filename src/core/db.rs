use indexmap::IndexMap;
use sqlx::{FromRow, Pool, Postgres, QueryBuilder};

use crate::{core::de::QueryResult, utils::db::get_pg_pool};

#[derive(Debug)]
pub struct DBX {
    pub pool: Pool<Postgres>,
}

impl DBX {
    pub async fn new(max_connections: u32, db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = get_pg_pool(max_connections, db_url).await?;
        Ok(Self { pool })
    }
}

pub trait DBXAction {
    fn find(&self, collection: String) -> DBXQueryBuilder;
    fn insert(&self, collection: String) -> DBXInsertBuilder;
    fn update(&self, collection: String) -> DBXUpdateBuilder;
    fn delete(&self, collection: String) -> DBXDeleteBuilder;
    fn introspect_collection(&self) -> DBXIntrospectBuilder;
}

impl DBXAction for DBX {
    fn find(&self, collection: String) -> DBXQueryBuilder {
        DBXQueryBuilder::new(self.pool.clone(), collection)
    }

    fn insert(&self, collection: String) -> DBXInsertBuilder {
        DBXInsertBuilder::new(self.pool.clone(), collection)
    }

    fn update(&self, collection: String) -> DBXUpdateBuilder {
        DBXUpdateBuilder::new(self.pool.clone(), collection)
    }

    fn delete(&self, collection: String) -> DBXDeleteBuilder {
        DBXDeleteBuilder::new(self.pool.clone(), collection)
    }

    fn introspect_collection(&self) -> DBXIntrospectBuilder {
        DBXIntrospectBuilder::new(self.pool.clone())
    }
}

pub struct DBXQueryBuilder {
    pool: Pool<Postgres>,
    collection: String,
    field: Option<Vec<String>>,
    limit: Option<i32>,
    sort: Option<String>,
    filter: Option<String>,
}

impl DBXQueryBuilder {
    fn new(pool: Pool<Postgres>, collection: String) -> Self {
        Self {
            pool,
            collection,
            field: None,
            limit: None,
            sort: None,
            filter: None,
        }
    }

    pub fn query(
        &mut self,
        field: Option<Vec<String>>,
        limit: Option<i32>,
        sort: Option<String>,
        filter: Option<String>,
    ) -> &mut Self {
        self.field = field;
        self.limit = limit;
        self.sort = sort;
        self.filter = filter;
        self
    }

    fn build_query<'a>(
        &'a self,
        query_builder: &'a mut QueryBuilder<'a, Postgres>,
    ) -> &'a mut QueryBuilder<'a, Postgres> {
        query_builder.push("select ");
        match &self.field {
            Some(field) => {
                for (i, f) in field.iter().enumerate() {
                    query_builder.push_bind(f.as_str());
                    if i != field.len() - 1 {
                        query_builder.push(", ");
                    }
                }
            }
            None => {
                query_builder.push("*");
            }
        }

        query_builder.push(" from ");
        query_builder.push_bind(self.collection.as_str());

        if let Some(filter) = &self.filter {
            query_builder.push(" where ");
            query_builder.push(filter.as_str());
        }

        if let Some(sort) = &self.sort {
            query_builder.push(" order by ");
            query_builder.push_bind(sort.as_str());
        }

        if let Some(limit) = self.limit {
            query_builder.push(" limit ");
            query_builder.push_bind(limit);
        }

        query_builder
    }

    pub async fn get_one(&self) -> Result<QueryResult, sqlx::Error> {
        Ok(self
            .build_query(&mut QueryBuilder::new(""))
            .build_query_as::<QueryResult>()
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn get_all(&self) -> Result<Vec<QueryResult>, sqlx::Error> {
        Ok(self
            .build_query(&mut QueryBuilder::new(""))
            .build_query_as::<QueryResult>()
            .fetch_all(&self.pool)
            .await?)
    }
}

pub struct DBXInsertBuilder {
    pool: Pool<Postgres>,
    collection: String,
    values: Vec<IndexMap<String, serde_json::Value>>,
    returning: Option<Vec<String>>,
}

impl DBXInsertBuilder {
    fn new(pool: Pool<Postgres>, collection: String) -> Self {
        Self {
            pool,
            collection,
            values: Vec::new(),
            returning: None,
        }
    }

    pub fn values(&mut self, values: Vec<IndexMap<String, serde_json::Value>>) -> &mut Self {
        self.values = values;
        self
    }

    fn build_query<'a>(
        &'a self,
        query_builder: &'a mut QueryBuilder<'a, Postgres>,
    ) -> &'a mut QueryBuilder<'a, Postgres> {
        query_builder.push("insert into ");
        query_builder.push_bind(self.collection.as_str());
        query_builder.push(" (");

        for (i, v) in self.values[0].iter().enumerate() {
            query_builder.push_bind(v.0.as_str());
            if i != self.values[0].len() - 1 {
                query_builder.push(", ");
            }
        }

        query_builder.push(") values ");

        for (i, v) in self.values.iter().enumerate() {
            query_builder.push("(");
            for (j, vv) in v.iter().enumerate() {
                query_builder.push_bind(vv.1.as_str());
                if j != v.len() - 1 {
                    query_builder.push(", ");
                }
            }
            query_builder.push(")");
            if i != self.values.len() - 1 {
                query_builder.push(", ");
            }
        }

        if let Some(returning) = &self.returning {
            query_builder.push(" returning ");
            for (i, r) in returning.iter().enumerate() {
                query_builder.push_bind(r.as_str());
                if i != returning.len() - 1 {
                    query_builder.push(", ");
                }
            }
        }

        query_builder
    }

    pub async fn execute(&self) -> Result<QueryResult, sqlx::Error> {
        Ok(self
            .build_query(&mut QueryBuilder::new(""))
            .build_query_as::<QueryResult>()
            .fetch_one(&self.pool)
            .await?)
    }
}

pub struct DBXUpdateBuilder {
    pool: Pool<Postgres>,
    collection: String,
    values: IndexMap<String, serde_json::Value>,
    filter: Option<String>,
}

impl DBXUpdateBuilder {
    fn new(pool: Pool<Postgres>, collection: String) -> Self {
        Self {
            pool,
            collection,
            values: IndexMap::new(),
            filter: None,
        }
    }

    pub fn values(&mut self, values: IndexMap<String, serde_json::Value>) -> &mut Self {
        self.values = values;
        self
    }

    pub fn filter(&mut self, filter: String) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    fn build_query<'a>(
        &'a self,
        query_builder: &'a mut QueryBuilder<'a, Postgres>,
    ) -> &'a mut QueryBuilder<'a, Postgres> {
        query_builder.push("update ");
        query_builder.push_bind(self.collection.as_str());
        query_builder.push(" set ");

        for (i, v) in self.values.iter().enumerate() {
            query_builder.push_bind(v.0.as_str());
            query_builder.push(" = ");
            query_builder.push_bind(v.1.as_str());
            if i != self.values.len() - 1 {
                query_builder.push(", ");
            }
        }

        if let Some(filter) = &self.filter {
            query_builder.push(" where ");
            query_builder.push(filter.as_str());
        }

        query_builder
    }

    pub async fn execute(&self) -> Result<QueryResult, sqlx::Error> {
        Ok(self
            .build_query(&mut QueryBuilder::new(""))
            .build_query_as::<QueryResult>()
            .fetch_one(&self.pool)
            .await?)
    }
}

pub struct DBXDeleteBuilder {
    pool: Pool<Postgres>,
    collection: String,
    filter: Option<String>,
}

impl DBXDeleteBuilder {
    fn new(pool: Pool<Postgres>, collection: String) -> Self {
        Self {
            pool,
            collection,
            filter: None,
        }
    }

    pub fn filter(&mut self, filter: String) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    fn build_query<'a>(
        &'a self,
        query_builder: &'a mut QueryBuilder<'a, Postgres>,
    ) -> &'a mut QueryBuilder<'a, Postgres> {
        query_builder.push("delete from ");
        query_builder.push_bind(self.collection.as_str());

        if let Some(filter) = &self.filter {
            query_builder.push(" where ");
            query_builder.push(filter.as_str());
        }

        query_builder
    }

    pub async fn execute(&self) -> Result<QueryResult, sqlx::Error> {
        Ok(self
            .build_query(&mut QueryBuilder::new(""))
            .build_query_as::<QueryResult>()
            .fetch_one(&self.pool)
            .await?)
    }
}

pub struct DBXIntrospectBuilder {
    pool: Pool<Postgres>,
    db_name: String,
}

#[derive(FromRow)]
pub struct Collection {
    pub table_name: String,
}

impl DBXIntrospectBuilder {
    fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            db_name: String::new(),
        }
    }

    pub fn db_name(&mut self, db_name: String) -> &mut Self {
        self.db_name = db_name;
        self
    }

    pub async fn execute(&self) -> Result<Vec<Collection>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Collection>(
            r#"
            select
                table_name,
            from information_schema.columns
            where table_schema = 'public'
            and table_type = 'BASE TABLE'
            and table_catalog = $1
            "#,
        )
        .bind(self.db_name.as_str())
        .fetch_all(&self.pool)
        .await?)
    }
}
