use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Response {
    pub signature: String,
    pub public_key: String,
    pub message: String
}

#[derive(Deserialize)]
pub struct Request {
    pub message: String,
    pub secret: String
}