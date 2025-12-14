use super::{IndicatorValue, SignalGenerator, SignalType, TradingSignal};
use chrono::Utc;

pub struct RSISignal {
    pub period: usize,
    pub overbought: f64,
    pub oversold: f64,
    pub symbol: String,
}

impl SignalGenerator for RSISignal {
    fn generate_signal(&self, prices: &[f64]) -> Result<TradingSignal, Box<dyn std::error::Error>> {
        if prices.len() < self.period + 1 {
            return Err("Insufficient data for RSI calculation".into());
        }

        // Calculate RSI manually
        let rsi_values = self.calculate_rsi(prices);
        
        if rsi_values.is_empty() {
            return Err("Failed to calculate RSI values".into());
        }
        
        let rsi_value = *rsi_values.last().unwrap();
        let last_price = *prices.last().unwrap();

        // Generate signal based on RSI levels
        let (signal_type, confidence) = if rsi_value <= self.oversold {
            // Oversold condition (bullish)
            let oversold_pct = ((self.oversold - rsi_value) / self.oversold).abs();
            let confidence = (oversold_pct * 100.0).min(100.0);
            (SignalType::Buy, confidence)
        } else if rsi_value >= self.overbought {
            // Overbought condition (bearish)
            let overbought_pct = ((rsi_value - self.overbought) / (100.0 - self.overbought)).abs();
            let confidence = (overbought_pct * 100.0).min(100.0);
            (SignalType::Sell, confidence)
        } else {
            // Neutral zone
            let mid_point = (self.oversold + self.overbought) / 2.0;
            let distance_from_mid = (rsi_value - mid_point).abs() / (self.overbought - self.oversold) * 2.0;
            let confidence = (distance_from_mid * 50.0).min(50.0);
            (SignalType::Hold, confidence)
        };

        let indicators = vec![
            IndicatorValue {
                name: "RSI".to_string(),
                value: rsi_value,
                signal: signal_type.clone(),
            },
            IndicatorValue {
                name: "RSI Oversold".to_string(),
                value: self.oversold,
                signal: SignalType::Hold,
            },
            IndicatorValue {
                name: "RSI Overbought".to_string(),
                value: self.overbought,
                signal: SignalType::Hold,
            },
        ];

        Ok(TradingSignal {
            symbol: self.symbol.clone(),
            signal_type,
            confidence,
            price: last_price,
            timestamp: Utc::now().timestamp(),
            indicators,
        })
    }

    fn calculate(&self, prices: &[f64]) -> Vec<f64> {
        self.calculate_rsi(prices)
    }
}

impl RSISignal {
    fn calculate_rsi(&self, prices: &[f64]) -> Vec<f64> {
        if prices.len() <= self.period {
            return vec![];
        }
        
        let mut gains = Vec::new();
        let mut losses = Vec::new();
        
        // Calculate price changes
        for i in 1..prices.len() {
            let change = prices[i] - prices[i-1];
            if change >= 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-change);
            }
        }
        
        let mut rsi_values = Vec::new();
        
        // Calculate initial averages
        let mut avg_gain: f64 = gains[..self.period].iter().sum::<f64>() / self.period as f64;
        let mut avg_loss: f64 = losses[..self.period].iter().sum::<f64>() / self.period as f64;
        
        if avg_loss == 0.0 {
            rsi_values.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsi_values.push(100.0 - (100.0 / (1.0 + rs)));
        }
        
        // Calculate remaining RSI values
        for i in self.period..gains.len() {
            avg_gain = (avg_gain * (self.period - 1) as f64 + gains[i]) / self.period as f64;
            avg_loss = (avg_loss * (self.period - 1) as f64 + losses[i]) / self.period as f64;
            
            if avg_loss == 0.0 {
                rsi_values.push(100.0);
            } else {
                let rs = avg_gain / avg_loss;
                rsi_values.push(100.0 - (100.0 / (1.0 + rs)));
            }
        }
        
        rsi_values
    }
}