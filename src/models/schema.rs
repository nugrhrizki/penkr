use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTable {
    pub name: String,
    #[serde(default = "default_primary_key")]
    pub primary_key: String,
    pub fields: Vec<Field>,
}

fn default_primary_key() -> String {
    "id".to_string()
}

#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    pub r#type: String,
    pub unique: bool,
    pub required: bool,
}
