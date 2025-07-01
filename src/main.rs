mod endpoints;

use std::str::FromStr;
use actix_web::{ post, web, App, HttpServer, Responder, Result};
use solana_sdk::{signature::{Keypair, Signature}, signer::Signer, system_instruction};
use spl_token::solana_program::pubkey::Pubkey;
use endpoints::{response::{Response, ErrorMessage}, generate_keypair, token_create, token_mint, message_sign, message_verify, send_sol, send_token};

#[post("/keypair")]
async fn keypair_generate() -> Result<impl Responder, ErrorMessage> {
    let keypair = Keypair::new();
    let response = generate_keypair::Response {
        pubkey: keypair.pubkey().to_string(),
        secret: keypair.to_base58_string()
    };
    Ok(web::Json(Response::success(response)))
}

#[post("/token/create")]
async fn create_token(info: web::Json<token_create::Request>) -> Result<impl Responder, ErrorMessage> {
    let mint_authority = match Pubkey::from_str(&info.mintAuthority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("mintAuthority PubKey is not valid".to_string()));
        }
    };
    let mint = match Pubkey::from_str(&info.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("mintAuthority PubKey is not valid".to_string()));
        }
    };
    let decimals = info.decimals;
    let transaction = spl_token::instruction::initialize_mint(&spl_token::id(), &mint, &mint_authority, Some(&mint_authority), decimals);
    let response = token_create::Response {
        program_id: transaction.clone().unwrap().program_id.to_string(),
        accounts: transaction.clone().unwrap().accounts,
        instruction_data: transaction.unwrap().data
    };
    Ok(web::Json(Response::success(response)))
}

#[post("/token/mint")]
async fn mint_token(info: web::Json<token_mint::Request>) -> Result<impl Responder, ErrorMessage> {
    let mint = match Pubkey::from_str(&info.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("mint PubKey is not valid".to_string()));
        }
    };
    let destination = match Pubkey::from_str(&info.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("destination PubKey is not valid".to_string()));
        }
    };
    let authority = match Pubkey::from_str(&info.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("authority PubKey is not valid".to_string()));
        }
    };
    let amount = info.amount;
    let transaction = spl_token::instruction::mint_to(&spl_token::id(), &mint, &destination, &authority, &[&authority], amount);
    let response = token_mint::Response {
        program_id: transaction.clone().unwrap().program_id.to_string(),
        accounts: transaction.clone().unwrap().accounts,
        instruction_data: transaction.unwrap().data
    };
    Ok(web::Json(Response::success(response)))
}

#[post("/message/sign")]
async fn sign_message(info: web::Json<message_sign::Request>) -> Result<impl Responder, ErrorMessage> {
    let keypair = Keypair::from_base58_string(&info.secret);
    let signature = keypair.sign_message(&info.message.as_bytes());
    let response = message_sign::Response {
        signature: signature.to_string(),
        public_key: keypair.pubkey().to_string(),
        message: info.message.clone()
    };
    Ok(web::Json(Response::success(response)))
}

#[post("/message/verify")]
async fn verify_message(info: web::Json<message_verify::Request>) -> Result<impl Responder, ErrorMessage> {
    let pubkey = match Pubkey::from_str(&info.pubkey) {
            Ok(pubkey) => pubkey,
            Err(_) => {
            return Err(ErrorMessage::error("authority PubKey is not valid".to_string()));
            }  
    };
    let sign = match Signature::from_str(&info.signature) {
        Ok(sig) => sig,
        Err(err) => {
            return Err(ErrorMessage::error(err.to_string()))
        }
    };
    let verified = sign.verify(pubkey.as_ref(), &info.message.as_bytes());
    let response = message_verify::Response {
        valid: verified,
        message: info.message.clone(),
        pubkey: pubkey.to_string()
    };
    Ok(web::Json(Response::success(response)))
}

#[post("/send/sol")]
async fn sol_send(info: web::Json<send_sol::Request>) -> Result<impl Responder, ErrorMessage> {
    let from = match solana_sdk::pubkey::Pubkey::from_str(&info.from) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("from PubKey is not valid".to_string()));
        }
    };
    let to = match solana_sdk::pubkey::Pubkey::from_str(&info.to) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("to PubKey is not valid".to_string()));
        }
    };
    let lamports = info.lamports;
    let transaction = system_instruction::transfer(&from, &to, lamports);
    let response = send_sol::Response {
        program_id: transaction.program_id.to_string(),
        accounts: [from.to_string(), to.to_string()],
        instruction_data: transaction.data
    };
    Ok(web::Json(Response::success(response)))
}

#[post("/send/token")]
async fn token_send(info: web::Json<send_token::Request>) -> Result<impl Responder, ErrorMessage> {
    let destination = match Pubkey::from_str(&info.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("destination pubkey is not valid".to_string()))
        }
    };
    let owner = match Pubkey::from_str(&info.owner) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err(ErrorMessage::error("owner pubkey is not valid".to_string()))
        }
    };
    let amount = info.amount;
    let transaction = spl_token::instruction::transfer(&spl_token::id(), &owner, &destination, &owner, &[&owner], amount);
    let response = send_token::Response {
        program_id: transaction.clone().unwrap().program_id.to_string(),
        accounts: transaction.clone().unwrap().accounts,
        instruction_data: transaction.unwrap().data
    };
    Ok(web::Json(Response::success(response)))
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