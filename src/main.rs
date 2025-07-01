mod endpoints;

use actix_web::{post, web, App, HttpServer, Responder, Result};
use endpoints::{
    generate_keypair, message_sign, message_verify,
    response::{ErrorMessage, Response},
    send_sol, send_token, token_create, token_mint,
};
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
};
use spl_token::solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[post("/keypair")]
async fn keypair_generate() -> Result<impl Responder, ErrorMessage> {
    let keypair = Keypair::new();
    let response = generate_keypair::Response {
        pubkey: keypair.pubkey().to_string(),
        secret: keypair.to_base58_string(),
    };
    Ok(web::Json(Response::new(response)))
}

#[post("/token/create")]
async fn create_token(
    info: web::Json<token_create::Request>,
) -> Result<impl Responder, ErrorMessage> {
    let mint_authority = match Pubkey::from_str(&info.mintAuthority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new(
                "mintAuthority PubKey is not valid".to_string(),
            ));
        }
    };
    let mint = match Pubkey::from_str(&info.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new(
                "mintAuthority PubKey is not valid".to_string(),
            ));
        }
    };
    let transaction = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        info.decimals,
    );
    let response = token_create::Response {
        program_id: transaction.clone().unwrap().program_id.to_string(),
        accounts: transaction.clone().unwrap().accounts,
        instruction_data: transaction.unwrap().data,
    };
    Ok(web::Json(Response::new(response)))
}

#[post("/token/mint")]
async fn mint_token(info: web::Json<token_mint::Request>) -> Result<impl Responder, ErrorMessage> {
    let mint = match Pubkey::from_str(&info.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new("mint PubKey is not valid".to_string()));
        }
    };
    let destination = match Pubkey::from_str(&info.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new(
                "destination PubKey is not valid".to_string(),
            ));
        }
    };
    let authority = match Pubkey::from_str(&info.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new(
                "authority PubKey is not valid".to_string(),
            ));
        }
    };
    let transaction = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[&authority],
        info.amount,
    );
    let response = token_mint::Response {
        program_id: transaction.clone().unwrap().program_id.to_string(),
        accounts: transaction.clone().unwrap().accounts,
        instruction_data: transaction.unwrap().data,
    };
    Ok(web::Json(Response::new(response)))
}

#[post("/message/sign")]
async fn sign_message(
    info: web::Json<message_sign::Request>,
) -> Result<impl Responder, ErrorMessage> {
    let decoded_bytes = match bs58::decode(&info.secret).into_vec() {
        Ok(value) => value,
        Err(_) => return Err(ErrorMessage::new("Invalid secret provided".to_string())),
    };
    if decoded_bytes.len() != 64 {
        return Err(ErrorMessage::new("Invalid secret provided".to_string()));
    }
    let keypair = match Keypair::from_bytes(&decoded_bytes) {
        Ok(value) => value,
        Err(_) => return Err(ErrorMessage::new("Invalid secret provided".to_string())),
    };
    let signature = keypair.sign_message(&info.message.as_bytes());
    let response = message_sign::Response {
        signature: signature.to_string(),
        public_key: keypair.pubkey().to_string(),
        message: info.message.clone(),
    };
    Ok(web::Json(Response::new(response)))
}

#[post("/message/verify")]
async fn verify_message(
    info: web::Json<message_verify::Request>,
) -> Result<impl Responder, ErrorMessage> {
    let pubkey = match Pubkey::from_str(&info.pubkey) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new(
                "authority PubKey is not valid".to_string(),
            ));
        }
    };
    let sign = match Signature::from_str(&info.signature) {
        Ok(sig) => sig,
        Err(err) => return Err(ErrorMessage::new(err.to_string())),
    };
    let verified = sign.verify(pubkey.as_ref(), &info.message.as_bytes());
    let response = message_verify::Response {
        valid: verified,
        message: info.message.clone(),
        pubkey: pubkey.to_string(),
    };
    Ok(web::Json(Response::new(response)))
}

#[post("/send/sol")]
async fn sol_send(info: web::Json<send_sol::Request>) -> Result<impl Responder, ErrorMessage> {
    let from = match solana_sdk::pubkey::Pubkey::from_str(&info.from) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new("from PubKey is not valid".to_string()));
        }
    };
    let to = match solana_sdk::pubkey::Pubkey::from_str(&info.to) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new("to PubKey is not valid".to_string()));
        }
    };
    let transaction = system_instruction::transfer(&from, &to, info.lamports);
    let response = send_sol::Response {
        program_id: transaction.program_id.to_string(),
        accounts: [from.to_string(), to.to_string()],
        instruction_data: transaction.data,
    };
    Ok(web::Json(Response::new(response)))
}

#[post("/send/token")]
async fn token_send(info: web::Json<send_token::Request>) -> Result<impl Responder, ErrorMessage> {
    let destination = match Pubkey::from_str(&info.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::new(
                "destination pubkey is not valid".to_string(),
            ))
        }
    };
    let owner = match Pubkey::from_str(&info.owner) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(ErrorMessage::new("owner pubkey is not valid".to_string())),
    };
    let transaction = spl_token::instruction::transfer(
        &spl_token::id(),
        &owner,
        &destination,
        &owner,
        &[&owner],
        info.amount,
    );
    let response = send_token::Response {
        program_id: transaction.clone().unwrap().program_id.to_string(),
        accounts: transaction.clone().unwrap().accounts,
        instruction_data: transaction.unwrap().data,
    };
    Ok(web::Json(Response::new(response)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(keypair_generate)
            .service(create_token)
            .service(mint_token)
            .service(sign_message)
            .service(verify_message)
            .service(sol_send)
            .service(token_send)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
