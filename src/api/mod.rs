mod auth;
mod collection;
mod db;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/db")
            .service(db::introspect)
            .service(db::connect)
            .service(db::disconnect)
            .service(db::select),
    );
    cfg.service(
        web::scope("/collection")
            .service(collection::get_all)
            .service(collection::get)
            .service(collection::get_by_field)
            .service(collection::create)
            .service(collection::create_batch)
            .service(collection::update)
            .service(collection::update_batch)
            .service(collection::delete)
            .service(collection::delete_batch),
    );
    cfg.service(
        web::scope("/auth")
            .service(auth::login)
            .service(auth::logout)
            .service(auth::register),
    );
}
