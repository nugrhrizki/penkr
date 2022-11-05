use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx;

use crate::AppState;

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(body: web::Json<LoginData>, state: web::Data<AppState>) -> impl Responder {
    let sqlite_pool = &state.sqlite_pool;
    let user =
        sqlx::query_as::<_, LoginData>("SELECT * FROM users WHERE username = $1 AND password = $2")
            .bind(&body.username)
            .bind(&body.password)
            .fetch_one(sqlite_pool)
            .await;
    if let Ok(user) = user {
        return HttpResponse::Ok().body(format!("Logged in as {:#?}", user));
    }
    HttpResponse::Ok().body("Failed to login")
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logged out")
}

#[post("/register")]
pub async fn register(body: web::Json<LoginData>, state: web::Data<AppState>) -> impl Responder {
    let sqlite_pool = &state.sqlite_pool;
    let user = sqlx::query_as::<_, LoginData>("SELECT * FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_one(sqlite_pool)
        .await;
    if let Ok(_) = user {
        return HttpResponse::Ok().body("User already exists");
    }
    let user =
        sqlx::query_as::<_, LoginData>("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(&body.username)
            .bind(&body.password)
            .fetch_one(sqlite_pool)
            .await;
    if let Ok(user) = user {
        return HttpResponse::Ok().body(format!("Registered as {:#?}", user));
    }
    return HttpResponse::Ok().body("Failed to register");
}
