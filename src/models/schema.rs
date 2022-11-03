use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Table {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct VColumn {
    pub name: String,
    pub data_type: String,
    pub is_nullable: String,
    // pub is_primary_key: bool,
    // pub is_unique: bool,
    // pub is_auto_increment: bool,
    pub maximum_length: Option<i32>,
    pub default_value: Option<String>,
}
