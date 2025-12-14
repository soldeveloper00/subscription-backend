use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub symbols: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SubscribeResponse {
    pub status: String,
    pub message: String,
    pub subscription_id: String,
}

pub async fn subscribe(subscription: web::Json<SubscribeRequest>) -> impl Responder {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    HttpResponse::Ok().json(SubscribeResponse {
        status: "subscribed".to_string(),
        message: format!("Subscribed {} to {} symbols", 
            subscription.email, subscription.symbols.len()),
        subscription_id: format!("sub-{}", timestamp),
    })
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "subscriptions": 0,
        "active": true,
        "timestamp": chrono::Utc::now().timestamp()
    }))
}
