mod internal;
mod models;
mod utils;
mod api;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use actix_files as fs;

use crate::utils::db::get_sqlite_pool;
use crate::internal::db::DBX;

#[derive(Debug)]
pub struct AppState {
    dbx: Mutex<Option<DBX>>,
    sqlite_pool: sqlx::SqlitePool,
}

#[get("/{tail:.*}")]
async fn index() -> impl Responder {
    fs::NamedFile::open("public/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !std::path::Path::new("db").exists() {
        std::fs::create_dir("db").expect("Failed to create db directory");
    }

    if !std::path::Path::new("db/pnkr.db").exists() {
        std::fs::File::create("db/pnkr.db").expect("Failed to create pnkr.db");
    }

    let sqlite_pool = get_sqlite_pool(5, "sqlite://db/pnkr.db").await.expect("Failed to connect to application database");

    let app_state = web::Data::new(AppState {
        dbx: Mutex::new(None),
        sqlite_pool,
    });

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(web::scope("/api").configure(api::init_routes))
            .service(fs::Files::new("/public", "./public"))
            .service(index)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
