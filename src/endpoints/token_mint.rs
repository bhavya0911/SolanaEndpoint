use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Request {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64
}

#[derive(Serialize)]
pub struct Response {
    pub program_id: String,
    pub accounts: Vec<spl_token::solana_program::instruction::AccountMeta>,
    pub instruction_data: Vec<u8>
}