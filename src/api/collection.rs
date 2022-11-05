use actix_web::{get,put,delete, post, web, HttpResponse, Responder};

use crate::AppState;

#[get("/{collection}")]
async fn get_all(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .select(format!("select * from {} limit 100", path).as_str())
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

#[get("/{collection}/{id}")]
async fn get(path: web::Path<(String, String)>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .select_one(format!("select * from {} where id = {}", path.0, path.1).as_str())
                .await;
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

#[get("/{collection}/{id}/{field}")]
async fn get_by_field(
    path: web::Path<(String, String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.dbx.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(dbx) = pg_pool.as_ref() {
            let users = dbx
                .select(format!(
                    "select * from {} where {} = {}",
                    path.0, path.2, path.1
                )
                .as_str())
                .await;
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
async fn delete(
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
