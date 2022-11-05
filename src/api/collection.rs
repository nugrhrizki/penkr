use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{internal::db::DBQuery, AppState};

#[derive(Deserialize)]
struct QueryFilter {
    columns: Option<Vec<String>>,
    where_clause: Option<String>,
    order_by: Option<String>,
    order: Option<String>,
    limit: Option<String>,
    offset: Option<String>,
}

#[get("/{collection}")]
async fn get_all(
    path: web::Path<String>,
    filter: web::Query<QueryFilter>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let query = DBQuery {
                table: path.clone(),
                columns: filter.columns.clone(),
                where_clause: filter.where_clause.clone(),
                order_by: filter.order_by.clone(),
                order: filter.order.clone(),
                limit: filter.limit.clone(),
                offset: filter.offset.clone(),
            };
            let users = dbx.select(&query).await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/{collection}/{id}")]
async fn get(
    path: web::Path<(String, String)>,
    filter: web::Query<QueryFilter>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let query = DBQuery {
                table: path.0.clone(),
                columns: filter.columns.clone(),
                where_clause: Some(format!("id = '{}'", path.1)),
                order_by: filter.order_by.clone(),
                order: filter.order.clone(),
                limit: filter.limit.clone(),
                offset: filter.offset.clone(),
            };
            let users = dbx.select_one(&query).await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::NotFound().body(format!(
                "Collection {} with id {} is not found",
                path.0, path.1
            ));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/{collection}/{where}/{value}")]
async fn get_by_field(
    path: web::Path<(String, String, String)>,
    filter: web::Query<QueryFilter>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let query = DBQuery {
                table: path.0.clone(),
                columns: filter.columns.clone(),
                where_clause: Some(format!("{} = '{}'", path.1, path.2)),
                order_by: filter.order_by.clone(),
                order: filter.order.clone(),
                limit: filter.limit.clone(),
                offset: filter.offset.clone(),
            };
            let users = dbx.select(&query).await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::NotFound().body(format!(
                "Collection {} with {} {} is not found",
                path.0, path.1, path.2
            ));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[post("/{collection}")]
async fn create(
    path: web::Path<String>,
    state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .insert_one(format!("insert into {} values {}", path, body).as_str())
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[post("/batch/{collection}")]
async fn create_batch(
    path: web::Path<String>,
    state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .insert(format!("insert into {} values {}", path, body).as_str())
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[put("/{collection}/{id}")]
async fn update(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .update(format!("update {} set {} where id = {}", path.0, body, path.1).as_str())
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path.0));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[put("/batch/{collection}/{id}")]
async fn update_batch(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .update(format!("update {} set {} where id = {}", path.0, body, path.1).as_str())
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path.0));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[delete("/{collection}/{id}")]
async fn delete(path: web::Path<(String, String)>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .delete(format!("delete from {} where id = {}", path.0, path.1).as_str())
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path.0));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[delete("/batch/{collection}/{id}")]
async fn delete_batch(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .delete(format!("delete from {} where id = {}", path.0, path.1).as_str())
                .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path.0));
        }
        return HttpResponse::InternalServerError().body("Not connected to database");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}
