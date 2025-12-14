use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ConditionRequest {
    pub symbol: String,
    pub target_price: f64,
    pub condition: String,
}

#[derive(Debug, Serialize)]
pub struct ConditionResponse {
    pub status: String,
    pub message: String,
    pub condition_id: String,
}

pub async fn set_price_condition(condition: web::Json<ConditionRequest>) -> impl Responder {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    HttpResponse::Ok().json(ConditionResponse {
        status: "ok".to_string(),
        message: format!("Condition set for {} {} {}", 
            condition.symbol, condition.condition, condition.target_price),
        condition_id: format!("cond-{}", timestamp),
    })
}

pub async fn check_condition_example() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Example condition check endpoint",
        "example": "When BTC > 100000, execute trade"
    }))
}
