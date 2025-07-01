use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub pubkey: String,
    pub secret: String,
}
