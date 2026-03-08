use std::collections::HashMap;

pub struct Ticker {
    symbol: String,
    last_price: f64,
    price_24h_pcnt: f64,
    volume_24h: f64,
    high_price_24h: f64,
    low_price_24h: f64,
}

impl Ticker {
    pub fn new(
        symbol: &str,
        last_price: f64,
        price_24h_pcnt: f64,
        volume_24h: f64,
        high_price_24h: f64,
        low_price_24h: f64,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            last_price,
            price_24h_pcnt,
            volume_24h,
            high_price_24h,
            low_price_24h,
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_last_price(&self) -> f64 {
        self.last_price
    }

    pub fn get_price_24h_pcnt(&self) -> f64 {
        self.price_24h_pcnt
    }

    pub fn get_volume_24h(&self) -> f64 {
        self.volume_24h
    }

    pub fn get_high_price_24h(&self) -> f64 {
        self.high_price_24h
    }

    pub fn get_low_price_24h(&self) -> f64 {
        self.low_price_24h
    }
}

pub struct Kline {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: i64,
}

impl Kline {
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: f64, timestamp: i64) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        }
    }

    pub fn get_open(&self) -> f64 {
        self.open
    }

    pub fn get_high(&self) -> f64 {
        self.high
    }

    pub fn get_low(&self) -> f64 {
        self.low
    }

    pub fn get_close(&self) -> f64 {
        self.close
    }

    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

pub struct NewsArticle {
    title: String,
    source: String,
    sentiment: Option<String>,
    published_at: String,
}

impl NewsArticle {
    pub fn new(
        title: &str,
        source: &str,
        sentiment: Option<String>,
        published_at: &str,
    ) -> Self {
        Self {
            title: title.into(),
            source: source.into(),
            sentiment,
            published_at: published_at.into(),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_sentiment(&self) -> Option<&str> {
        self.sentiment.as_deref()
    }

    pub fn get_published_at(&self) -> &str {
        &self.published_at
    }
}

pub struct DerivativesData {
    symbol: String,
    orderbook_ratio: f64,
    bid_volume: f64,
    ask_volume: f64,
    funding_rate: f64,
    open_interest: f64,
    long_ratio: f64,
    short_ratio: f64,
}

impl DerivativesData {
    pub fn new(
        symbol: &str,
        orderbook_ratio: f64,
        bid_volume: f64,
        ask_volume: f64,
        funding_rate: f64,
        open_interest: f64,
        long_ratio: f64,
        short_ratio: f64,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            orderbook_ratio,
            bid_volume,
            ask_volume,
            funding_rate,
            open_interest,
            long_ratio,
            short_ratio,
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_funding_rate(&self) -> f64 {
        self.funding_rate
    }

    pub fn get_long_ratio(&self) -> f64 {
        self.long_ratio
    }

    pub fn get_short_ratio(&self) -> f64 {
        self.short_ratio
    }

    pub fn get_orderbook_ratio(&self) -> f64 {
        self.orderbook_ratio
    }
}

pub struct MarketSnapshot {
    tickers: Vec<Ticker>,
    klines: HashMap<String, Vec<Kline>>,
    news: Vec<NewsArticle>,
    derivatives: Vec<DerivativesData>,
}

impl MarketSnapshot {
    pub fn new(
        tickers: Vec<Ticker>,
        klines: HashMap<String, Vec<Kline>>,
        news: Vec<NewsArticle>,
        derivatives: Vec<DerivativesData>,
    ) -> Self {
        Self {
            tickers,
            klines,
            news,
            derivatives,
        }
    }

    pub fn first_symbol(&self) -> Option<String> {
        self.tickers.first().map(|t| t.get_symbol().to_string())
    }

    pub fn get_ticker(&self, symbol: &str) -> Option<&Ticker> {
        self.tickers.iter().find(|t| t.get_symbol() == symbol)
    }

    pub fn get_klines(&self, symbol: &str) -> Option<&Vec<Kline>> {
        self.klines.get(symbol)
    }

    pub fn get_derivatives(&self, symbol: &str) -> Option<&DerivativesData> {
        self.derivatives.iter().find(|d| d.get_symbol() == symbol)
    }

    pub fn tickers_to_json(&self) -> String {
        let entries: Vec<String> = self
            .tickers
            .iter()
            .map(|t| {
                format!(
                    "{{\"symbol\":\"{}\",\"price\":{},\"change_24h\":{},\"volume\":{},\"high\":{},\"low\":{}}}",
                    t.get_symbol(),
                    t.get_last_price(),
                    t.get_price_24h_pcnt(),
                    t.get_volume_24h(),
                    t.get_high_price_24h(),
                    t.get_low_price_24h()
                )
            })
            .collect();
        format!("[{}]", entries.join(","))
    }

    pub fn klines_to_json(&self) -> String {
        let entries: Vec<String> = self
            .klines
            .iter()
            .map(|(symbol, klines)| {
                let kline_entries: Vec<String> = klines
                    .iter()
                    .map(|k| {
                        format!(
                            "{{\"o\":{},\"h\":{},\"l\":{},\"c\":{},\"v\":{},\"t\":{}}}",
                            k.get_open(),
                            k.get_high(),
                            k.get_low(),
                            k.get_close(),
                            k.get_volume(),
                            k.get_timestamp()
                        )
                    })
                    .collect();
                format!("\"{}\":[{}]", symbol, kline_entries.join(","))
            })
            .collect();
        format!("{{{}}}", entries.join(","))
    }

    pub fn news_to_json(&self) -> String {
        let entries: Vec<String> = self
            .news
            .iter()
            .map(|n| {
                let sentiment = match n.get_sentiment() {
                    Some(s) => format!("\"{}\"", s),
                    None => "null".to_string(),
                };
                format!(
                    "{{\"title\":\"{}\",\"source\":\"{}\",\"sentiment\":{},\"published\":\"{}\"}}",
                    n.get_title().replace('"', "\\\""),
                    n.get_source().replace('"', "\\\""),
                    sentiment,
                    n.get_published_at()
                )
            })
            .collect();
        format!("[{}]", entries.join(","))
    }

    pub fn derivatives_to_json(&self) -> String {
        let entries: Vec<String> = self
            .derivatives
            .iter()
            .map(|d| {
                let ob_signal = if d.orderbook_ratio > 1.2 {
                    "bullish_pressure"
                } else if d.orderbook_ratio < 0.8 {
                    "bearish_pressure"
                } else {
                    "neutral"
                };

                let funding_signal = if d.funding_rate < -0.005 {
                    "short_squeeze_risk"
                } else if d.funding_rate > 0.005 {
                    "long_squeeze_risk"
                } else {
                    "neutral"
                };

                format!(
                    "{{\"symbol\":\"{}\",\
                    \"orderbook_ratio\":{:.3},\"orderbook_signal\":\"{}\",\
                    \"bid_volume\":{:.2},\"ask_volume\":{:.2},\
                    \"funding_rate\":{:.6},\"funding_signal\":\"{}\",\
                    \"open_interest\":{:.2},\
                    \"long_ratio\":{:.3},\"short_ratio\":{:.3}}}",
                    d.symbol,
                    d.orderbook_ratio, ob_signal,
                    d.bid_volume, d.ask_volume,
                    d.funding_rate, funding_signal,
                    d.open_interest,
                    d.long_ratio, d.short_ratio,
                )
            })
            .collect();
        format!("[{}]", entries.join(","))
    }

    pub fn compute_indicators(&self, timeframe: &str) -> String {
        // Timeframe-aware thresholds
        // Shorter timeframes: streaks are common, need more candles to signal exhaustion
        // Longer timeframes: each candle covers hours/days, fewer needed
        // Timeframe-adaptive parameters
        // (streak_warn, streak_exhaust, momentum_threshold, sma_dist_threshold, sma_fast, sma_slow, rsi_period)
        let (streak_warn, streak_exhaust, momentum_threshold, sma_dist_threshold, sma_fast, sma_slow, rsi_period) = match timeframe {
            "5min"  => (6u32, 8u32, 0.5f64, 0.15f64, 5usize, 13usize, 9usize),
            "30min" => (5, 7, 1.0, 0.3, 7, 15, 10),
            "1h"    => (4, 6, 2.0, 1.0, 10, 20, 14),
            "6h"    => (3, 4, 2.5, 1.5, 10, 20, 14),
            "12h"   => (2, 3, 3.0, 2.0, 10, 20, 14),
            "24h"   => (2, 3, 4.0, 2.5, 10, 20, 14),
            _       => (4, 6, 2.0, 1.0, 10, 20, 14),
        };

        let mut results = Vec::new();

        for (symbol, klines) in &self.klines {
            if klines.len() < 5 {
                continue;
            }

            let closes: Vec<f64> = klines.iter().map(|k| k.get_close()).collect();
            let highs: Vec<f64> = klines.iter().map(|k| k.get_high()).collect();
            let lows: Vec<f64> = klines.iter().map(|k| k.get_low()).collect();
            let volumes: Vec<f64> = klines.iter().map(|k| k.get_volume()).collect();
            let n = closes.len();
            let current_price = closes[n - 1];

            // Fast SMA (timeframe-adaptive period)
            let sma10 = if n >= sma_fast {
                closes[n - sma_fast..].iter().sum::<f64>() / sma_fast as f64
            } else {
                closes.iter().sum::<f64>() / n as f64
            };

            // Slow SMA (timeframe-adaptive period)
            let sma20 = if n >= sma_slow {
                closes[n - sma_slow..].iter().sum::<f64>() / sma_slow as f64
            } else {
                closes.iter().sum::<f64>() / n as f64
            };

            // RSI (timeframe-adaptive period)
            let rsi = compute_rsi(&closes, rsi_period);

            // Price change over last 5 candles (%)
            let momentum_5 = if n >= 6 {
                ((current_price - closes[n - 6]) / closes[n - 6]) * 100.0
            } else {
                ((current_price - closes[0]) / closes[0]) * 100.0
            };

            // Price change over last 10 candles (%)
            let momentum_10 = if n >= 11 {
                ((current_price - closes[n - 11]) / closes[n - 11]) * 100.0
            } else {
                ((current_price - closes[0]) / closes[0]) * 100.0
            };

            // Volume trend: avg last 5 candles / avg prior candles
            let recent_count = n.min(5);
            let vol_recent: f64 =
                volumes[n - recent_count..].iter().sum::<f64>() / recent_count as f64;

            let prior_end = if n > 5 { n - 5 } else { 0 };
            let vol_prior = if prior_end > 0 {
                volumes[..prior_end].iter().sum::<f64>() / prior_end as f64
            } else {
                vol_recent
            };

            let volume_ratio = if vol_prior > 0.0 {
                vol_recent / vol_prior
            } else {
                1.0
            };

            // Support (lowest low) and Resistance (highest high)
            let resistance = highs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let support = lows.iter().cloned().fold(f64::INFINITY, f64::min);

            // Green vs red candle count
            let count_n = n.min(20);
            let green = klines[n - count_n..]
                .iter()
                .filter(|k| k.get_close() > k.get_open())
                .count();
            let red = count_n - green;

            // SMA crossover trend
            let sma_trend = if sma10 > sma20 {
                "bullish"
            } else if sma10 < sma20 {
                "bearish"
            } else {
                "neutral"
            };

            // Price position relative to SMAs
            let price_position = if current_price > sma10 && current_price > sma20 {
                "above_both_SMAs"
            } else if current_price < sma10 && current_price < sma20 {
                "below_both_SMAs"
            } else {
                "between_SMAs"
            };

            // Distance to support/resistance as %
            let dist_to_resistance = if resistance > 0.0 {
                ((resistance - current_price) / current_price) * 100.0
            } else {
                0.0
            };
            let dist_to_support = if support > 0.0 {
                ((current_price - support) / current_price) * 100.0
            } else {
                0.0
            };

            // Consecutive candles in same direction (streak from most recent)
            let mut streak = 0i32;
            for k in klines.iter().rev() {
                let is_green = k.get_close() > k.get_open();
                if streak == 0 {
                    streak = if is_green { 1 } else { -1 };
                } else if (streak > 0 && is_green) || (streak < 0 && !is_green) {
                    streak += if is_green { 1 } else { -1 };
                } else {
                    break;
                }
            }
            let streak_dir = if streak > 0 { "green" } else { "red" };
            let streak_count = streak.unsigned_abs();

            // Distance from SMA20 (overextension indicator)
            let dist_from_sma20 = if sma20 > 0.0 {
                ((current_price - sma20) / sma20) * 100.0
            } else {
                0.0
            };

            // Last candle wick analysis (reversal signal)
            let last_candle = &klines[n - 1];
            let body = (last_candle.get_close() - last_candle.get_open()).abs();
            let full_range = last_candle.get_high() - last_candle.get_low();
            let wick_ratio = if full_range > 0.0 {
                1.0 - (body / full_range)
            } else {
                0.0
            };
            // Upper wick vs lower wick
            let upper_wick = last_candle.get_high()
                - last_candle.get_close().max(last_candle.get_open());
            let lower_wick = last_candle.get_close().min(last_candle.get_open())
                - last_candle.get_low();
            let last_candle_signal = if wick_ratio > 0.6 && upper_wick > lower_wick * 2.0 {
                "bearish_rejection" // long upper wick = sellers pushed price down
            } else if wick_ratio > 0.6 && lower_wick > upper_wick * 2.0 {
                "bullish_rejection" // long lower wick = buyers pushed price up
            } else if wick_ratio > 0.7 {
                "indecision" // doji-like candle
            } else {
                "normal"
            };

            // Exhaustion signal: combines streak + RSI + overextension (timeframe-aware)
            let exhaustion = if streak_count >= streak_exhaust && streak > 0 && rsi > 65.0 && dist_from_sma20.abs() > sma_dist_threshold {
                "BULLISH_EXHAUSTION_likely_reversal_down"
            } else if streak_count >= streak_exhaust && streak < 0 && rsi < 35.0 && dist_from_sma20.abs() > sma_dist_threshold {
                "BEARISH_EXHAUSTION_likely_reversal_up"
            } else if streak_count >= streak_warn && streak > 0 && rsi > 55.0 {
                "bullish_extended_pullback_possible"
            } else if streak_count >= streak_warn && streak < 0 && rsi < 45.0 {
                "bearish_extended_bounce_possible"
            } else if momentum_5.abs() > momentum_threshold && streak_count >= streak_warn {
                if momentum_5 > 0.0 { "momentum_overextended_up" } else { "momentum_overextended_down" }
            } else {
                "none"
            };

            // Last 3 candles pattern (important for short timeframes)
            let last3_pattern = if n >= 3 {
                let c1 = &klines[n - 3]; // oldest of the 3
                let c2 = &klines[n - 2];
                let c3 = &klines[n - 1]; // most recent

                let c1_bull = c1.get_close() > c1.get_open();
                let c2_bull = c2.get_close() > c2.get_open();
                let c3_bull = c3.get_close() > c3.get_open();

                let c3_body = (c3.get_close() - c3.get_open()).abs();
                let c2_body = (c2.get_close() - c2.get_open()).abs();

                if !c2_bull && c3_bull && c3_body > c2_body * 1.5 {
                    "bullish_engulfing" // strong buy signal
                } else if c2_bull && !c3_bull && c3_body > c2_body * 1.5 {
                    "bearish_engulfing" // strong sell signal
                } else if c1_bull && c2_bull && c3_bull {
                    "three_green_soldiers"
                } else if !c1_bull && !c2_bull && !c3_bull {
                    "three_red_crows"
                } else if c1_bull && !c2_bull && c3_bull && c3.get_close() > c1.get_close() {
                    "bullish_recovery"
                } else if !c1_bull && c2_bull && !c3_bull && c3.get_close() < c1.get_close() {
                    "bearish_rejection_pattern"
                } else {
                    "mixed"
                }
            } else {
                "insufficient_data"
            };

            // Volume spike on last candle (important for short timeframes)
            let vol_spike = if n >= 2 {
                let last_vol = volumes[n - 1];
                let avg_vol = if n >= 6 {
                    volumes[n - 6..n - 1].iter().sum::<f64>() / 5.0
                } else {
                    volumes[..n - 1].iter().sum::<f64>() / (n - 1) as f64
                };
                if avg_vol > 0.0 { last_vol / avg_vol } else { 1.0 }
            } else {
                1.0
            };
            let vol_spike_label = if vol_spike > 2.0 {
                "HIGH_SPIKE"
            } else if vol_spike > 1.5 {
                "moderate_spike"
            } else if vol_spike < 0.5 {
                "very_low"
            } else {
                "normal"
            };

            results.push(format!(
                "{{\"symbol\":\"{symbol}\",\
                \"sma_fast({sma_fast})\":{sma10:.4},\"sma_slow({sma_slow})\":{sma20:.4},\"sma_trend\":\"{sma_trend}\",\
                \"rsi({rsi_period})\":{rsi:.1},\
                \"momentum_5_candles\":\"{momentum_5:+.2}%\",\"momentum_10_candles\":\"{momentum_10:+.2}%\",\
                \"volume_ratio\":{volume_ratio:.2},\"last_candle_volume_spike\":\"{vol_spike_label}({vol_spike:.1}x)\",\
                \"support\":{support:.4},\"resistance\":{resistance:.4},\
                \"dist_to_support\":\"{dist_to_support:.2}%\",\"dist_to_resistance\":\"{dist_to_resistance:.2}%\",\
                \"green_candles\":{green},\"red_candles\":{red},\
                \"price_position\":\"{price_position}\",\
                \"consecutive_streak\":\"{streak_count} {streak_dir}\",\
                \"dist_from_sma_slow\":\"{dist_from_sma20:+.2}%\",\
                \"last_candle_signal\":\"{last_candle_signal}\",\
                \"last_3_candles_pattern\":\"{last3_pattern}\",\
                \"exhaustion_signal\":\"{exhaustion}\"}}"
            ));
        }

        format!("[{}]", results.join(","))
    }
}

fn compute_rsi(closes: &[f64], period: usize) -> f64 {
    if closes.len() <= period {
        return 50.0;
    }

    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;

    for i in 1..=period {
        let change = closes[i] - closes[i - 1];
        if change > 0.0 {
            avg_gain += change;
        } else {
            avg_loss += change.abs();
        }
    }
    avg_gain /= period as f64;
    avg_loss /= period as f64;

    // Wilder's smoothing for remaining data points
    for i in (period + 1)..closes.len() {
        let change = closes[i] - closes[i - 1];
        if change > 0.0 {
            avg_gain = (avg_gain * (period as f64 - 1.0) + change) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0)) / period as f64;
        } else {
            avg_gain = (avg_gain * (period as f64 - 1.0)) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) + change.abs()) / period as f64;
        }
    }

    if avg_loss == 0.0 {
        return 100.0;
    }

    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}
