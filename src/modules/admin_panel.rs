use actix_web::{get, Responder, HttpResponse, web};
use actix_files as fs;

use crate::AppState;


#[get("/{tail:.*}")]
async fn index() -> impl Responder {
    fs::NamedFile::open("public/index.html")
}

#[get("/info")]
async fn info(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:#?}", state))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(info);
    cfg.service(fs::Files::new("/public", "./public"));
    cfg.service(index);
}
