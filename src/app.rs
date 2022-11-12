mod auth;
mod db;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth::login)
            .service(auth::logout)
            .service(auth::register),
    );
    cfg.service(
        web::scope("/db")
            .service(db::connect)
            .service(db::collections)
            .service(db::introspect_collection),
    );
}
