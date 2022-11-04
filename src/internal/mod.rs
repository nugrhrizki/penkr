use actix_files as fs;
use actix_web::web;

mod api;
mod auth;
mod db;
mod de;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin").service(fs::Files::new("/", "public").index_file("index.html")),
    );
    cfg.service(
        web::scope("/api")
            .service(api::collections)
            .service(api::collection),
    );
    cfg.service(
        web::scope("/db")
            .service(db::introspect)
            .service(db::connect)
            .service(db::disconnect)
            .service(db::select)
            .service(db::reconnect),
    );
    cfg.service(
        web::scope("/auth")
            .service(auth::login)
            .service(auth::logout)
            .service(auth::register),
    );
}
