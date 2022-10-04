mod models;
mod utils;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use models::schema::CreateTable;
use serde::Deserialize;
use serde_json::json;
use utils::query_builder::table_fields;

use crate::utils::db::get_pool;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, index")
}

#[get("/createDB")]
async fn create_db(data: web::Data<AppState>) -> impl Responder {
    let _pool = &data.db_pool;
    HttpResponse::Ok().body("create DB")
}

#[post("/dropTable")]
async fn drop_table(request: web::Json<TableRequest>, data: web::Data<AppState>) -> impl Responder {
    sqlx::query(format!("DROP TABLE IF EXISTS {}", request.table_name).as_str())
        .execute(&data.db_pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Table dropped"
    }))
}

#[derive(Deserialize)]
struct TableRequest {
    table_name: String,
}

#[post("/createTable")]
async fn create_table(
    request: web::Json<CreateTable>,
    data: web::Data<AppState>,
) -> impl Responder {
    let fields = table_fields(&request.fields);
    if let Some(pool) = &data.db_pool {
        let sql_res = sqlx::query(
            format!(
                "CREATE TABLE IF NOT EXISTS {} ({} SERIAL PRIMARY KEY{});",
                request.name, request.primary_key, fields
            )
            .as_str(),
        )
        .execute(pool)
        .await;

        match sql_res {
            Ok(_) => HttpResponse::Ok().json(json!({
                "status": "ok",
                "message": "Table created"
            })),
            Err(e) => HttpResponse::Ok().json(json!({
                "status": "error",
                "message": e.to_string()
            })),
        }
    } else {
        HttpResponse::Ok().json(json!({
            "status": "error",
            "message": "No connection to DB"
        }))
    }
}

#[get("/insert")]
async fn insert() -> impl Responder {
    HttpResponse::Ok().body("insert")
}

#[get("/select")]
async fn select() -> impl Responder {
    HttpResponse::Ok().body("select")
}

struct AppState {
    db_pool: Option<sqlx::PgPool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let get_pool = get_pool().await;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let mut pool: Option<sqlx::PgPool> = None;

    match get_pool {
        Ok(pg) => {
            pool = Some(pg);
        }
        Err(e) => {
            // log error here
            println!("Error: {}", e);
        }
    }

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .service(index)
            .service(create_db)
            .service(drop_table)
            .service(create_table)
            .service(insert)
            .service(select)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
