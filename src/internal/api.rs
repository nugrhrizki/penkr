use actix_web::{get, web, HttpResponse, Responder};

use crate::{internal::de::QueryResult, AppState};

#[get("/collections/{collection}")]
async fn collections(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(pg_pool) = pg_pool.as_ref() {
            let users = sqlx::query_as::<_, QueryResult>(
                format!("select * from {} limit 100", path).as_str(),
            )
            .fetch_all(pg_pool)
            .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::InternalServerError()
                .body(format!("Failed to collections: {}", path));
        }
        return HttpResponse::InternalServerError().body("Not connected to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}

#[get("/collections/{collection}/{id}")]
async fn collection(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let state_pg_pool = state.pg_pool.lock().ok();
    if let Some(pg_pool) = state_pg_pool {
        if let Some(pg_pool) = pg_pool.as_ref() {
            let users = sqlx::query_as::<_, QueryResult>(
                format!("select * from {} where id = {}", path.0, path.1).as_str(),
            )
            .fetch_one(pg_pool)
            .await;
            if let Ok(user) = users {
                return HttpResponse::Ok().json(user);
            }
            return HttpResponse::NotFound().body(format!(
                "Collection {} with id {} is not found",
                path.0, path.1
            ));
        }
        return HttpResponse::InternalServerError().body("Not connected to Postgres");
    }
    HttpResponse::InternalServerError().body("Failed to lock pool")
}
