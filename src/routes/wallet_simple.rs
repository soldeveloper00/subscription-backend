use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn wallet_info() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Wallet info endpoint"
    }))
}

pub async fn test_solana_connection() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Test connection endpoint"
    }))
}
