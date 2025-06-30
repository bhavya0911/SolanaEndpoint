use std::str::FromStr;
use actix_web::{get, http::Error, post, web, App, HttpResponse, HttpServer, Responder, Result};
use solana_sdk::signature::{self, Signature};
use solana_sdk::signer::{keypair, SeedDerivable};
use solana_sdk::{ signature::Keypair, signer::Signer};
use serde::{Serialize, Deserialize};
use spl_token::solana_program::pubkey::Pubkey;

#[derive(Serialize)] 
struct ErrorMessage {
    success: bool,
    error: String,
}

#[derive(Deserialize)]
struct SendSolRequest {
    from: String,
    to: String,
    lamports: usize
}

#[derive(Deserialize)]
struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64
}

#[derive(Serialize)]
struct SendTokenResponse {
    success: bool,
    data: Transaction
}

#[derive(Deserialize)]
struct VerifyingMessageRequest {
    message: String,
    signature: String,
    pubkey: String
}

#[derive(Serialize)]
struct VerifyingMessageResponse {
    success: bool,
    data: VerifyingMessageData
}

#[derive(Serialize)]
struct VerifyingMessageData {
    valid: bool,
    message: String,
    pubkey: String,
}

#[derive(Deserialize)]
struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: usize
}

#[derive(Deserialize)]
struct SigningRequest {
    message: String,
    secret: String
}

#[derive(Serialize)]
struct SigningResponse {
    success: bool,
    data: SigningData
}

#[derive(Serialize)]
struct SigningData {
    signature: String,
    public_key: String,
    message: String,
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    mintAuthority: String,
    mint: String,
    decimals: usize
}

#[derive(Serialize)]
struct CreateTokenResponse {
    success: bool,
    data: Transaction
}

#[derive(Serialize)]
struct Transaction {
    program_id: String,
    accounts: Vec<spl_token::solana_program::instruction::AccountMeta>,
    instruction_data: Vec<u8>
}

#[derive(Serialize)]
struct KeyPairData {
    pubkey: String,
    secret: String
}

#[derive(Serialize)]
struct KeyPairResponse {
    success: bool,
    data: KeyPairData
}

#[post("/keypair")]
async fn generate() -> Result<impl Responder, Error> {
    let keypair = Keypair::new();
    let response = KeyPairResponse {
        success: true,
        data: KeyPairData {
            pubkey: keypair.pubkey().to_string(),
            secret: keypair.to_base58_string()
        }
    };
    Ok(web::Json(response))
}

#[post("/token/create")]
async fn create_token(info: web::Json<CreateTokenRequest>) -> Result<impl Responder, Error> {
    let mint_authority = Pubkey::from_str(&info.mintAuthority).unwrap();
    let mint = Pubkey::from_str(&info.mint).unwrap();
    let decimals = info.decimals.try_into().unwrap();
    let transaction = spl_token::instruction::initialize_mint(&spl_token::id(), &mint, &mint_authority, Some(&mint_authority), decimals);
    let response = CreateTokenResponse {
        success: true,
        data: Transaction {
            program_id: transaction.clone().unwrap().program_id.to_string(),
            accounts: transaction.clone().unwrap().accounts,
            instruction_data: transaction.unwrap().data
        }
    };
    Ok(web::Json(response))
}

#[post("/token/mint")]
async fn token_mint(info: web::Json<MintTokenRequest>) -> Result<impl Responder, Error> {
    let mint = Pubkey::from_str(&info.mint).unwrap();
    let destination = Pubkey::from_str(&info.destination).unwrap();
    let authority = Pubkey::from_str(&info.authority).unwrap();
    let amount = info.amount.try_into().unwrap();
    let transaction = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[&authority],
        amount
    );
    let response = CreateTokenResponse {
        success: true,
        data: Transaction {
            program_id: transaction.clone().unwrap().program_id.to_string(),
            accounts: transaction.clone().unwrap().accounts,
            instruction_data: transaction.unwrap().data
        }
    };
    Ok(web::Json(response))
}

#[post("/message/sign")]
async fn sign(info: web::Json<SigningRequest>) -> Result<impl Responder, Error> {
    let message = &info.message;
    let secret = &info.secret;
    let keypair = Keypair::from_base58_string(&secret);
    let signature = keypair.sign_message(message.as_bytes()).to_string();
    let response = SigningResponse {
        success: true,
        data: SigningData { 
            signature: signature, 
            public_key: keypair.pubkey().to_string(),
            message: info.message.to_string()
        }
    };
    Ok(web::Json(response))
}

#[post("/message/verify")]
async fn verify(info: web::Json<VerifyingMessageRequest>) -> Result<impl Responder, Error> {
    let message = &info.message;
    let signature = &info.signature;
    let pubkey = Pubkey::from_str(&info.pubkey).unwrap();
    let new_sign = Signature::from_str(signature).unwrap();
    let verify = new_sign.verify(pubkey.as_ref(), message.as_bytes());
    let response = VerifyingMessageResponse {
        success: true,
        data: VerifyingMessageData { valid: verify, message: message.to_string(), pubkey: pubkey.to_string() }
    };
    Ok(web::Json(response))
}

#[post("/send/sol")]
async fn send_sol(info: web::Json<SendSolRequest>) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok())
}

#[post("/send/token")]
async fn send_token(info: web::Json<SendTokenRequest>) -> Result<impl Responder, Error> {
    // let transaction = spl_token::instruction::transfer(&spl_token::id(), &info.owner, &info.destination, &info.owner, &[&info.owner], info.amount);
    // let response = SendTokenResponse {
    //     success: true,
    //     data: Transaction {
    //         program_id: transaction.clone().unwrap().program_id.to_string(),
    //         accounts: transaction.clone().unwrap().accounts,
    //         instruction_data: transaction.unwrap().data
    //     }
    // };
    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(generate)
        .service(create_token)
        .service(token_mint)
        .service(sign)
        .service(verify)
        .service(send_sol)
        .service(send_token)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}