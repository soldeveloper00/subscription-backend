use super::{IndicatorValue, SignalGenerator, SignalType, TradingSignal};
use chrono::Utc;

pub struct MACDSignal {
    pub fast_period: usize,
    pub slow_period: usize,
    pub signal_period: usize,
    pub symbol: String,
}

impl SignalGenerator for MACDSignal {
    fn generate_signal(&self, prices: &[f64]) -> Result<TradingSignal, Box<dyn std::error::Error>> {
        if prices.len() < self.slow_period + self.signal_period {
            return Err("Insufficient data for MACD calculation".into());
        }

        // Calculate MACD manually
        let (macd_line, signal_line, histogram) = self.calculate_macd(prices);
        
        if macd_line.is_empty() || signal_line.is_empty() {
            return Err("Failed to calculate MACD values".into());
        }
        
        let last_macd = *macd_line.last().unwrap();
        let last_signal = *signal_line.last().unwrap();
        let last_histogram = *histogram.last().unwrap();
        let last_price = *prices.last().unwrap();

        // Generate signal based on MACD crossover
        let (signal_type, confidence) = if last_macd > last_signal && last_histogram > 0.0 {
            // Bullish crossover
            let histogram_pct = (last_histogram / last_price * 100.0).abs();
            let confidence = (histogram_pct * 10.0).min(100.0);
            (SignalType::Buy, confidence)
        } else if last_macd < last_signal && last_histogram < 0.0 {
            // Bearish crossover
            let histogram_pct = (last_histogram / last_price * 100.0).abs();
            let confidence = (histogram_pct * 10.0).min(100.0);
            (SignalType::Sell, confidence)
        } else {
            // No clear signal
            let confidence = 0.0;
            (SignalType::Hold, confidence)
        };

        let indicators = vec![
            IndicatorValue {
                name: "MACD Line".to_string(),
                value: last_macd,
                signal: if last_macd > last_signal { SignalType::Buy } else { SignalType::Sell },
            },
            IndicatorValue {
                name: "Signal Line".to_string(),
                value: last_signal,
                signal: SignalType::Hold,
            },
            IndicatorValue {
                name: "Histogram".to_string(),
                value: last_histogram,
                signal: if last_histogram > 0.0 { SignalType::Buy } else { SignalType::Sell },
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
        let (macd_line, _, _) = self.calculate_macd(prices);
        macd_line
    }
}

impl MACDSignal {
    fn calculate_macd(&self, prices: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        if prices.len() < self.slow_period {
            return (vec![], vec![], vec![]);
        }
        
        // Calculate EMA for fast and slow periods
        let ema_fast = self.calculate_ema(prices, self.fast_period);
        let ema_slow = self.calculate_ema(prices, self.slow_period);
        
        // Calculate MACD line (fast EMA - slow EMA)
        let mut macd_line = Vec::new();
        let min_len = ema_fast.len().min(ema_slow.len());
        
        for i in 0..min_len {
            // Adjust indices since ema_fast and ema_slow start at different positions
            let fast_idx = ema_fast.len() - min_len + i;
            let slow_idx = ema_slow.len() - min_len + i;
            macd_line.push(ema_fast[fast_idx] - ema_slow[slow_idx]);
        }
        
        // Calculate signal line (EMA of MACD line)
        let signal_line = if macd_line.len() >= self.signal_period {
            self.calculate_ema(&macd_line, self.signal_period)
        } else {
            vec![]
        };
        
        // Calculate histogram (MACD line - signal line)
        let mut histogram = Vec::new();
        let min_len_hist = macd_line.len().min(signal_line.len());
        
        for i in 0..min_len_hist {
            let macd_idx = macd_line.len() - min_len_hist + i;
            let signal_idx = signal_line.len() - min_len_hist + i;
            histogram.push(macd_line[macd_idx] - signal_line[signal_idx]);
        }
        
        (macd_line, signal_line, histogram)
    }
    
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