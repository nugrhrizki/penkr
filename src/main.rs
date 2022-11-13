mod app;
mod core;
mod modules;
mod utils;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};

use crate::core::db::DBX;
use crate::utils::setup::{setup_app_db, setup_config};

#[derive(Debug)]
pub struct AppState {
    dbx: Mutex<Option<DBX>>,
    app_db: sqlx::SqlitePool,
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

    let signing_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                signing_key.clone(),
            ))
            .app_data(app_state.clone())
            .service(web::scope("/pnkr").configure(app::init))
            .service(
                web::scope("/api")
                    .wrap(Cors::permissive())
                    .configure(modules::restful::init),
            )
            .configure(modules::admin_panel::init)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
