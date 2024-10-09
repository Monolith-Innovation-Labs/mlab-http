use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub id: u32,
    pub url: String,
    pub method: String,
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub id: u32,
    pub url: String,
    pub method: String,
    pub data: Option<Value>,
    pub status: u16,
}