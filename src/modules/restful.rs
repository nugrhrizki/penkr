use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use indexmap::IndexMap;
use serde::Deserialize;

use crate::{core::db::DBXAction, AppState};

#[derive(Deserialize)]
struct QueryFilter {
    field: Option<Vec<String>>,
    limit: Option<i32>,
    sort: Option<String>,
    filter: Option<String>,
}

#[get("/{collection}")]
async fn list(
    collection: web::Path<String>,
    query: web::Query<QueryFilter>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_dbx = state.dbx.lock().unwrap();

    if let Some(dbx) = &*state_dbx {
        let result = dbx
            .find(collection.to_string())
            .query(
                query.field.clone(),
                query.limit,
                query.sort.clone(),
                query.filter.clone(),
            )
            .get_all()
            .await;
        return match result {
            Ok(rows) => HttpResponse::Ok().json(rows),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        };
    }

    HttpResponse::InternalServerError().body("Not connected to database")
}

#[get("/{collection}/{id}")]
async fn view(
    collection: web::Path<(String, String)>,
    query: web::Query<QueryFilter>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let record = dbx
                .find(collection.0.clone())
                .query(
                    query.field.clone(),
                    query.limit,
                    query.sort.clone(),
                    Some(format!("id = {}", collection.1).to_string()),
                )
                .get_one()
                .await;
            return match record {
                Ok(row) => HttpResponse::Ok().json(row),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            };
        }
    }

    HttpResponse::InternalServerError().body("Not connected to database")
}

#[post("/{collection}")]
async fn create(
    collection: web::Path<String>,
    body: web::Json<Vec<IndexMap<String, serde_json::Value>>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let record = dbx
                .insert(collection.to_string())
                .values(body.clone())
                .execute()
                .await;
            return match record {
                Ok(row) => HttpResponse::Ok().json(row),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            };
        }
    }

    HttpResponse::InternalServerError().body("Not connected to database")
}

#[patch("/{collection}/{id}")]
async fn update(
    collection: web::Path<(String, String)>,
    body: web::Json<IndexMap<String, serde_json::Value>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let record = dbx
                .update(collection.0.clone())
                .values(body.clone())
                .filter(format!("id = '{}'", collection.1))
                .execute()
                .await;
            return match record {
                Ok(row) => HttpResponse::Ok().json(row),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            };
        }
    }

    HttpResponse::InternalServerError().body("Not connected to database")
}

#[delete("/{collection}/{id}")]
async fn delete(
    collection: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let record = dbx
                .delete(collection.0.clone())
                .filter(format!("id = '{}'", collection.1))
                .execute()
                .await;
            return match record {
                Ok(row) => HttpResponse::Ok().json(row),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            };
        }
    }

    HttpResponse::InternalServerError().body("Not connected to database")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(view);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
