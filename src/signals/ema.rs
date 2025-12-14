use super::{IndicatorValue, SignalGenerator, SignalType, TradingSignal};
use chrono::Utc;

pub struct EMASignal {
    pub short_period: usize,
    pub long_period: usize,
    pub symbol: String,
}

impl SignalGenerator for EMASignal {
    fn generate_signal(&self, prices: &[f64]) -> Result<TradingSignal, Box<dyn std::error::Error>> {
        if prices.len() < self.long_period {
            return Err("Insufficient data for EMA calculation".into());
        }

        // Calculate EMAs manually
        let ema_short_values = self.calculate_ema(prices, self.short_period);
        let ema_long_values = self.calculate_ema(prices, self.long_period);
        
        if ema_short_values.is_empty() || ema_long_values.is_empty() {
            return Err("Failed to calculate EMA values".into());
        }
        
        let last_short = *ema_short_values.last().unwrap();
        let last_long = *ema_long_values.last().unwrap();
        let last_price = *prices.last().unwrap();

        // Generate signal based on EMA crossover
        let (signal_type, confidence) = if last_short > last_long {
            // Golden cross: short EMA above long EMA (bullish)
            let price_diff = ((last_short - last_long) / last_long).abs();
            let confidence = (price_diff * 100.0).min(100.0);
            (SignalType::Buy, confidence)
        } else {
            // Death cross: short EMA below long EMA (bearish)
            let price_diff = ((last_long - last_short) / last_short).abs();
            let confidence = (price_diff * 100.0).min(100.0);
            (SignalType::Sell, confidence)
        };

        let indicators = vec![
            IndicatorValue {
                name: "EMA Short".to_string(),
                value: last_short,
                signal: if last_short > last_long { SignalType::Buy } else { SignalType::Sell },
            },
            IndicatorValue {
                name: "EMA Long".to_string(),
                value: last_long,
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
        self.calculate_ema(prices, self.short_period)
    }
}

impl EMASignal {
    fn calculate_ema(&self, prices: &[f64], period: usize) -> Vec<f64> {
        if prices.len() < period {
            return vec![];
        }
        
        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut emas = Vec::with_capacity(prices.len() - period + 1);
        
        // Calculate SMA for first value
        let first_sma: f64 = prices[..period].iter().sum::<f64>() / period as f64;
        emas.push(first_sma);
        
        // Calculate EMA for remaining values
        for i in period..prices.len() {
            let ema = (prices[i] - emas.last().unwrap()) * multiplier + emas.last().unwrap();
            emas.push(ema);
        }
        
        emas
    }
}