use serde::Deserialize;
use sqlx::{query, Pool, Sqlite};

use crate::{core::db::DBX, utils::db::get_sqlite_pool};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CorsConfig {
    pub origin: String,
    pub methods: Vec<String>,
    pub headers: Vec<String>,
    pub expose_headers: Vec<String>,
    pub credentials: bool,
    pub max_age: u32,
}

#[derive(Deserialize)]
pub struct PNKRConfig {
    pub server: Option<ServerConfig>,
    pub database: Option<DatabaseConfig>,
    pub cors: Option<Vec<CorsConfig>>,
}

pub struct Config {
    pub server: ServerConfig,
    pub cors: Vec<CorsConfig>,
    pub dbx: Option<DBX>,
}

pub async fn setup_app_db() -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    let mut is_configured = true;

    if !std::path::Path::new("database").exists() {
        std::fs::create_dir("database").expect("Failed to create db directory");
    }

    if !std::path::Path::new("database/pnkr.db").exists() {
        std::fs::File::create("database/pnkr.db").expect("Failed to create pnkr.db");
        is_configured = false;
    }

    let pool = get_sqlite_pool(5, "sqlite://database/pnkr.db")
        .await
        .expect("Failed to connect to application database");

    if !is_configured {
        query(
            r#"
            CREATE TABLE IF NOT EXISTS "user" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "password"	TEXT NOT NULL,
                "email"	TEXT NOT NULL UNIQUE,
            );
            "#,
        )
        .execute(&pool)
        .await
        .expect("Failed to create user table");

        query(
            r#"
            INSERT INTO "user" ("password", "email") VALUES (?, ?);
            "#,
        )
        .bind("secret")
        .bind("su@root")
        .execute(&pool)
        .await
        .expect("Failed to create default user");

        query(
            r#"
            CREATE TABLE IF NOT EXISTS "database" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "name"	TEXT NOT NULL,
                "host"	TEXT NOT NULL,
                "port"	INTEGER NOT NULL,
                "username"	TEXT NOT NULL,
                "password"	TEXT NOT NULL,
                "type"	TEXT NOT NULL,
            );
            "#,
        )
        .execute(&pool)
        .await
        .expect("Failed to create database table");

        query(
            r#"
            CREATE TABLE IF NOT EXISTS "collection" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "name"	TEXT NOT NULL,
            );
            "#,
        )
        .execute(&pool)
        .await
        .expect("Failed to create collection table");
    }

    Ok(pool)
}

pub async fn setup_config(app_db: &Pool<Sqlite>) -> Result<Config, Box<dyn std::error::Error>> {
    if !std::path::Path::new("config").exists() {
        std::fs::create_dir("config").expect("Failed to create config directory");
    }

    if !std::path::Path::new("config/pnkr.toml").exists() {
        std::fs::File::create("config/pnkr.toml").expect("Failed to create pnkr.toml");
    }

    let config = std::fs::read_to_string("config/pnkr.toml").expect("Failed to read config file");

    let config: PNKRConfig = toml::from_str(&config).expect("Failed to parse config file");

    let dbx = match config.database {
        Some(db) => {
            let dbx = DBX::new(
                5,
                &format!(
                    "postgres://{}:{}@{}:{}/{}",
                    db.username, db.password, db.host, db.port, db.name
                ),
            )
            .await;
            match dbx {
                Ok(dbx) => {
                    let _ = query(
                        r#"
                            INSERT INTO "database"
                            ("name" ,"host" ,"port" ,"username" ,"password" ,"type")
                            VALUES
                            (?, ?, ?, ?, ?, ?)
                        "#,
                    )
                    .bind(db.name)
                    .bind(db.host)
                    .bind(db.port)
                    .bind(db.username)
                    .bind(db.password)
                    .bind("postgres")
                    .execute(app_db)
                    .await;

                    Some(dbx)
                }
                Err(_) => None,
            }
        }
        None => None,
    };

    Ok(Config {
        server: config.server.unwrap_or(ServerConfig {
            port: 8080,
            host: "0.0.0.0".to_string(),
        }),
        cors: config.cors.unwrap_or(vec![CorsConfig {
            origin: "*".to_string(),
            methods: vec!["GET".to_string(), "POST".to_string()],
            headers: vec!["*".to_string()],
            expose_headers: vec![],
            credentials: true,
            max_age: 3600,
        }]),
        dbx,
    })
}
