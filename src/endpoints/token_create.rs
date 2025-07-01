use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Request {
    pub mintAuthority: String,
    pub mint: String,
    pub decimals: u8
}

#[derive(Serialize)]
pub struct Response {
    pub program_id: String,
    pub accounts: Vec<spl_token::solana_program::instruction::AccountMeta>,
    pub instruction_data: Vec<u8>
}