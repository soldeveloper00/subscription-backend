use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn blockchain_status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "blockchain_integration_ready",
        "message": "Backend is ready for blockchain integration",
        "your_wallet": "3dfYDooHVyZP5oe5x3sq72wSVa6wqdqRdvVJaYfjQvkC",
        "program_id": "J4QVyiFGT9csd3rUoPsgwrxdhUwRnu4paWB5uVgTH6S7",
        "environment": "devnet",
        "note": "Wallet private key is in .env for transaction signing",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}

pub async fn test_integration() -> impl Responder {
    // Test if we can create a simple integration
    let has_wallet_key = std::env::var("SOLANA_PRIVATE_KEY").is_ok();
    let has_rpc = std::env::var("SOLANA_RPC_URL").is_ok();
    let has_program = std::env::var("SOLANA_PROGRAM_ID").is_ok();
    
    HttpResponse::Ok().json(json!({
        "integration_test": true,
        "wallet_configured": has_wallet_key,
        "rpc_configured": has_rpc,
        "program_configured": has_program,
        "ready_for_real_integration": has_wallet_key && has_rpc && has_program,
        "next_step": "Implement transaction logic based on price conditions",
        "example_logic": "When BTC > $100,000 → call smart contract subscribe()",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}
