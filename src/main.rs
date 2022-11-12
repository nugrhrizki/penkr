mod app;
mod core;
mod modules;
mod utils;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

use crate::core::db::DBX;
use crate::utils::setup::{setup_app_db, setup_config};

#[derive(Debug)]
pub struct AppState {
    dbx: Mutex<Option<DBX>>,
    app_db: sqlx::SqlitePool,
}

#[get("/{tail:.*}")]
async fn index() -> impl Responder {
    fs::NamedFile::open("public/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_db = setup_app_db()
        .await
        .expect("Failed to setup application database");

    let config = setup_config(&app_db)
        .await
        .expect("Failed to setup configuration");

    let app_state = web::Data::new(AppState {
        dbx: Mutex::new(config.dbx),
        app_db,
    });

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(web::scope("/pnkr").configure(app::init))
            .service(web::scope("/api").configure(modules::restful::init))
            .service(fs::Files::new("/public", "./public"))
            .service(index)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
