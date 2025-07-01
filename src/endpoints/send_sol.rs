use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Response {
    pub program_id: String,
    pub accounts: [String; 2],
    pub instruction_data: Vec<u8>
}

#[derive(Deserialize)]
pub struct Request {
    pub from: String,
    pub to: String,
    pub lamports: u64
}