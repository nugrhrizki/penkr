use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, QueryBuilder};

use crate::{
    core::db::{DBXAction, DBX},
    AppState,
};

#[derive(Deserialize)]
struct DatabaseConnection {
    name: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    max_connections: u32,
}

#[post("/connect")]
async fn connect(
    body: web::Json<DatabaseConnection>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mut dbx = state.dbx.lock().unwrap();
    let app_db = &state.app_db;

    if let Some(_) = &*dbx {
        return HttpResponse::InternalServerError().body("Already connected to database");
    }

    let max_connections = body.max_connections.clone();

    let result = DBX::new(
        max_connections,
        &format!(
            "postgres://{}:{}@{}:{}/{}",
            body.username, body.password, body.host, body.port, body.name
        ),
    )
    .await;

    match result {
        Ok(new_dbx) => {
            *dbx = Some(new_dbx);
            let _ = query(
                r#"
                    INSERT INTO databases
                    (name, host, port, username, password, type)
                    VALUES
                    (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(body.name.clone())
            .bind(body.host.clone())
            .bind(body.port)
            .bind(body.username.clone())
            .bind(body.password.clone())
            .bind("postgres")
            .execute(app_db)
            .await;
            HttpResponse::Ok().body("Connected to database")
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to database"),
    }
}

#[derive(FromRow)]
struct DatabaseName {
    name: String,
}

#[get("/introspect_collection")]
async fn introspect_collection(state: web::Data<AppState>) -> impl Responder {
    let state_dbx = state.dbx.lock().unwrap();
    let app_db = &state.app_db;

    // select database name from database
    let database_name = sqlx::query_as::<_, DatabaseName>("SELECT name FROM databases LIMIT 1")
        .fetch_one(app_db)
        .await;

    if let Ok(database) = database_name {
        if let Some(dbx) = &*state_dbx {
            let result = dbx
                .introspect_collection()
                .db_name(database.name)
                .execute()
                .await;
            return match result {
                Ok(rows) => {
                    let mut query_builder =
                        QueryBuilder::new("insert into collections (name) values ");

                    for (index, row) in rows.iter().enumerate() {
                        query_builder.push("(");
                        query_builder.push(&row.table_name);
                        query_builder.push(")");
                        if index != rows.len() - 1 {
                            query_builder.push(",");
                        }
                    }

                    let res = query_builder.build().execute(app_db).await;

                    match res {
                        Ok(_) => HttpResponse::Ok().body("Introspected collection"),
                        Err(_) => HttpResponse::InternalServerError()
                            .body("Failed to introspect collection"),
                    }
                }
                Err(_) => {
                    HttpResponse::InternalServerError().body("Failed to introspect collection")
                }
            };
        }
    }

    HttpResponse::InternalServerError().body("Not connected to database")
}

#[derive(Serialize, FromRow)]
struct Collection {
    name: String,
}

#[get("/collections")]
async fn collections(state: web::Data<AppState>) -> impl Responder {
    let app_db = &state.app_db;

    let result = sqlx::query_as::<_, Collection>("SELECT * FROM \"collection\"")
        .fetch_all(app_db)
        .await;

    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get collections"),
    }
}
