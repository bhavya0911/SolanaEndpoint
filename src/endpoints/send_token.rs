use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Response {
    pub program_id: String,
    pub accounts: Vec<spl_token::solana_program::instruction::AccountMeta>,
    pub instruction_data: Vec<u8>
}

#[derive(Deserialize)]
pub struct Request {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64
}