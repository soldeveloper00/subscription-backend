pub mod http_client;

use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn create_binance_signature(query_string: &str, secret_key: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(query_string.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

pub fn format_price_data(data: &HashMap<String, Value>) -> HashMap<String, f64> {
    let mut result = HashMap::new();
    
    for (symbol, value) in data {
        if let Some(price_str) = value.as_str() {
            if let Ok(price) = price_str.parse::<f64>() {
                result.insert(symbol.clone(), price);
            }
        }
    }
    
    result
}

pub fn round_decimal(value: f64, decimals: usize) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}