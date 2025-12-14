pub mod ema;
pub mod macd;
pub mod rsi;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
    StrongBuy,
    StrongSell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    pub symbol: String,
    pub signal_type: SignalType,
    pub confidence: f64,
    pub price: f64,
    pub timestamp: i64,
    pub indicators: Vec<IndicatorValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorValue {
    pub name: String,
    pub value: f64,
    pub signal: SignalType,
}

pub trait SignalGenerator {
    fn generate_signal(&self, prices: &[f64]) -> Result<TradingSignal, Box<dyn std::error::Error>>;
    fn calculate(&self, prices: &[f64]) -> Vec<f64>;
}