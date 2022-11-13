use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx;

use crate::{AppState, utils::responder::Respond};

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(body: web::Json<Credentials>, session: Session, state: web::Data<AppState>) -> impl Responder {
    let sqlite_pool = &state.app_db;
    let user = sqlx::query_as::<_, Credentials>(
        "SELECT * FROM users WHERE email = $1 AND password = $2",
    )
    .bind(&body.email)
    .bind(&body.password)
    .fetch_one(sqlite_pool)
    .await;
    if let Ok(user) = user {
        session.insert("user_email", user.email).unwrap();
        return HttpResponse::Ok().json(Respond {
            status: 200,
            message: "Logged in successfully".to_string(),
            data: None,
        });
    }
    HttpResponse::BadRequest().json(Respond {
        status: 400,
        message: "Invalid credentials".to_string(),
        data: None,
    })
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().body("Logged out")
}
