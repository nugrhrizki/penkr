use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    internal::de::QueryResult,
    models::schema::{Table, VColumn},
    utils::db::get_pg_pool,
    AppState,
};

#[derive(Deserialize)]
struct DbBody {
    max_connections: u32,
    db_url: String,
}

#[post("/connect")]
async fn connect(body: web::Json<DbBody>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    let db_url = body.db_url.clone();
    let max_connections = body.max_connections.clone();
    if let Some(mut pg_pool) = state_pg_pool {
        let pool = get_pg_pool(max_connections, db_url.as_str()).await;
        if let Some(new_pool) = pool.ok() {
            *pg_pool = Some(new_pool);
            return HttpResponse::Ok().body("Connected to Postgres");
        }
        return HttpResponse::InternalServerError().body("Failed to connect to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/disconnect")]
async fn disconnect(state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    if let Some(mut pg_pool) = state_pg_pool {
        *pg_pool = None;
        return HttpResponse::Ok().body("Disconnected from Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/introspect")]
async fn introspect(state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(pg_pool) = pg_pool.as_ref() {
            let tables = sqlx::query_as::<_, Table>(
                r#"select t.table_name as name from information_schema.tables t
                where t.table_schema = 'public'
                and t.table_catalog = 'db_las'
                and t.table_type = 'BASE TABLE'"#,
            )
            .fetch_all(pg_pool)
            .await;
            if let Ok(tables) = tables {
                let mut tables_with_columns: Vec<VColumn> = Vec::new();
                for table in tables {
                    let columns = sqlx::query_as::<_, VColumn>(
                        r#"select t.column_name as name, t.data_type, t.is_nullable, t.column_default as default_value, t.character_maximum_length as maximum_length from information_schema.columns t
                        where t.table_schema = 'public'
                        and t.table_catalog = 'db_las'
                        and t.table_name = $1"#,
                    )
                    .bind(table.name)
                    .fetch_all(pg_pool)
                    .await;
                    if let Ok(columns) = columns {
                        tables_with_columns.extend(columns);
                    }
                }
                return HttpResponse::Ok().json(tables_with_columns);
            }
            return HttpResponse::InternalServerError().body("Failed to introspect Postgres");
        }
        return HttpResponse::InternalServerError().body("Not connected to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/reconnect")]
async fn reconnect(body: web::Json<DbBody>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    let db_url = body.db_url.clone();
    let max_connections = body.max_connections.clone();
    if let Some(mut pg_pool) = state_pg_pool {
        *pg_pool = None;
        let pool = get_pg_pool(max_connections, db_url.as_str()).await;
        if let Some(new_pool) = pool.ok() {
            *pg_pool = Some(new_pool);
            return HttpResponse::Ok().body("Reconnected to Postgres");
        }
        return HttpResponse::InternalServerError().body("Failed to reconnect to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/status")]
async fn status(state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(pg_pool) = pg_pool.as_ref() {
            let status = sqlx::query("select 1").fetch_one(pg_pool).await;
            if let Ok(_) = status {
                return HttpResponse::Ok().body("Connected to Postgres");
            }
            return HttpResponse::InternalServerError().body("Failed to connect to Postgres");
        }
        return HttpResponse::InternalServerError().body("Not connected to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[derive(Deserialize)]
struct Query {
    query: String,
}

#[post("/select")]
async fn select(body: web::Json<Query>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    let query = body.query.clone();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(pg_pool) = pg_pool.as_ref() {
            let users = sqlx::query_as::<_, QueryResult>(&query)
                .fetch_all(pg_pool)
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError().body("Failed to connect to Postgres");
        }
        return HttpResponse::InternalServerError().body("Not connected to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}
