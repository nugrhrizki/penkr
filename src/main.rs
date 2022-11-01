mod internal;
mod models;
mod utils;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use crate::utils::db::get_sqlite_pool;

#[derive(Debug)]
pub struct AppState {
    pg_pool: Mutex<Option<sqlx::PgPool>>,
    sqlite_pool: Option<sqlx::SqlitePool>,
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {:#?}", state))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sqlite_pool = get_sqlite_pool(5, "sqlite://data.db").await;

    let app_state = web::Data::new(AppState {
        pg_pool: Mutex::new(None),
        sqlite_pool: sqlite_pool.ok(),
    });

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(web::scope("/v").configure(internal::init_routes))
            .service(index)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
