use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub binance_base_url: String,
    pub trading_pairs: Vec<String>,
    
    // Signal parameters
    pub ema_short_period: usize,
    pub ema_long_period: usize,
    pub rsi_period: usize,
    pub rsi_overbought: f64,
    pub rsi_oversold: f64,
    pub macd_fast: usize,
    pub macd_slow: usize,
    pub macd_signal: usize,
    
    // Solana blockchain
    pub solana_rpc_url: Option<String>,
    pub solana_wallet_key: Option<String>,
    pub solana_program_id: Option<String>,
    pub update_interval_seconds: Option<u64>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let trading_pairs = env::var("TRADING_PAIRS")
            .unwrap_or_else(|_| "BTCUSDT,ETHUSDT,SOLUSDT".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|e| format!("Invalid PORT: {}", e))?,
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            binance_base_url: env::var("BINANCE_BASE_URL")
                .unwrap_or_else(|_| "https://api.binance.com".to_string()),
            trading_pairs,
            
            // Default signal parameters
            ema_short_period: 12,
            ema_long_period: 26,
            rsi_period: 14,
            rsi_overbought: 70.0,
            rsi_oversold: 30.0,
            macd_fast: 12,
            macd_slow: 26,
            macd_signal: 9,
            
            // Solana (optional)
            solana_rpc_url: env::var("SOLANA_RPC_URL").ok(),
            solana_wallet_key: env::var("SOLANA_WALLET_KEY").ok(),
            solana_program_id: env::var("SOLANA_PROGRAM_ID").ok(),
            update_interval_seconds: env::var("UPDATE_INTERVAL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok()),
        })
    }
}
