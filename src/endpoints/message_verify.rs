use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct Request {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}
