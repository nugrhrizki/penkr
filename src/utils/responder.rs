use serde::Serialize;

#[derive(Serialize)]
pub struct Respond {
    pub status: u16,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
