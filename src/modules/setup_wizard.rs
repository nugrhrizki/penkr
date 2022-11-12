use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Setup {
    user: User,
    database: Database,
}

#[derive(Deserialize, Serialize)]
struct User {
    password: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct Database {
    name: String,
    host: String,
    port: u16,
    user: String,
    password: String,
}

#[post("/setup")]
async fn setup(setup: web::Json<Setup>) -> impl Responder {
    let setup = setup.into_inner();
    HttpResponse::Ok().json(setup)
}
