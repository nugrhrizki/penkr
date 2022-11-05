use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::internal::db::DBX;
use crate::models::schema::{Table, VColumn};
use crate::AppState;

#[derive(Deserialize)]
struct Connect {
    max_connections: u32,
    db_url: String,
}

#[post("/connect")]
async fn connect(body: web::Json<Connect>, state: web::Data<AppState>) -> impl Responder {
    let dbx = state.dbx.lock().ok();
    if let Some(mut dbx) = dbx {
        if dbx.is_some() {
            return HttpResponse::Ok().body("Already connected to database");
        }
        let db_url = body.db_url.clone();
        let max_connections = body.max_connections.clone();
        let new_dbx = DBX::new(max_connections, db_url.as_str()).await;
        if let Ok(new_dbx) = new_dbx {
            *dbx = Some(new_dbx);
            return HttpResponse::Ok().body("Connected to database");
        }
        return HttpResponse::InternalServerError().body("Failed to connect to database");
    };
    HttpResponse::InternalServerError().body("Failed to lock dbx")
}

#[delete("/disconnect")]
async fn disconnect(state: web::Data<AppState>) -> impl Responder {
    let dbx = state.dbx.lock().ok();
    if let Some(mut mtx_dbx) = dbx {
        if let Some(dbx) = mtx_dbx.as_mut() {
            match dbx.disconnect().await {
                Ok(_) => {
                    *mtx_dbx = None;
                    return HttpResponse::Ok().body("Disconnected from database");
                }
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .body("Failed to disconnect from database");
                }
            }
        }
        return HttpResponse::Ok().body("You're not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock dbx")
}

#[put("/introspect")]
async fn introspect(state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let tables = sqlx::query_as::<_, Table>(
                r#"select t.table_name as name from information_schema.tables t
                where t.table_schema = 'public'
                and t.table_catalog = 'db_las'
                and t.table_type = 'BASE TABLE'"#,
            )
            .fetch_all(&dbx.pool)
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
                    .fetch_all(&dbx.pool)
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

#[get("/status")]
async fn status(state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let status = dbx.select_one("select 1").await;
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
    let state_pg_pool = state.dbx.lock().ok();
    let query = body.query.clone();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx.select(&query).await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError().body("Failed to connect to Postgres");
        }
        return HttpResponse::InternalServerError().body("Not connected to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}
