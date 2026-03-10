use crate::domain::prediction::entities::Prediction;

pub struct AnalysisService;

impl AnalysisService {
    pub fn build_analysis_prompt(
        tickers_json: &str,
        klines_json: &str,
        news_json: &str,
        indicators_json: &str,
        derivatives_json: &str,
        timeframe: &str,
    ) -> String {
        format!(
            "Timeframe: {}\n\n\
            === CURRENT PRICES (24h stats) ===\n\
            Each entry: symbol, price (current last traded), change_24h (decimal, -0.02 = -2%), volume (24h USD), high (24h highest), low (24h lowest)\n\
            {}\n\n\
            === PRE-COMPUTED TECHNICAL INDICATORS ===\n\
            These are deterministic calculations from candle data. TRUST these numbers — they are correct.\n\
            Key fields per symbol:\n\
            - sma_fast / sma_slow: moving averages. sma_fast > sma_slow = bullish crossover.\n\
            - sma_trend: \"bullish\" / \"bearish\" / \"neutral\" based on SMA crossover.\n\
            - rsi: relative strength index (>70 overbought, <30 oversold, 40-60 neutral).\n\
            - momentum_5_candles / momentum_10_candles: price change % over last 5/10 candles.\n\
            - volume_ratio: recent vol / prior vol (>1.2 = volume confirming, <0.8 = volume weak).\n\
            - support / resistance: lowest low / highest high across the candle data.\n\
            - dist_to_support / dist_to_resistance: distance as % of current price.\n\
            - consecutive_streak: how many candles in a row same direction (e.g. \"5 green\").\n\
            - exhaustion_signal: \"none\" = safe, anything else = caution.\n\
            - last_candle_signal: \"normal\" / \"bullish_rejection\" / \"bearish_rejection\" / \"indecision\".\n\
            - last_3_candles_pattern: engulfing, soldiers, crows, recovery patterns.\n\
            {}\n\n\
            === DERIVATIVES & ORDER BOOK DATA ===\n\
            - orderbook_ratio: bid_vol / ask_vol. >1.2 = buyers dominate, <0.8 = sellers dominate.\n\
            - funding_rate: perpetual swap cost. Negative = shorts pay longs (short squeeze risk). Positive = longs pay shorts (long squeeze risk). >0.01 or <-0.01 = extreme.\n\
            - long_ratio / short_ratio: what % of traders are long vs short. >0.60 = crowded side.\n\
            - open_interest: total open contracts (higher = more leveraged market).\n\
            {}\n\n\
            === CANDLESTICK HISTORY (oldest → newest) ===\n\
            o=open, h=high, l=low, c=close, v=volume, t=timestamp(ms)\n\
            {}\n\n\
            === RECENT NEWS ===\n\
            {}\n\n\
            IMPORTANT: The technical indicators are pre-computed and correct.\n\
            Use them as primary signal source. Cross-reference with derivatives data.\n\
            Derivatives often LEAD price — funding rate extremes and long/short ratio imbalances\n\
            predict reversals before they happen on the chart.",
            timeframe, tickers_json, indicators_json, derivatives_json, klines_json, news_json
        )
    }

    pub fn determine_outcome(prediction: &Prediction, current_price: f64) -> String {
        let direction = prediction.get_direction();
        let target = prediction.get_target_price();
        let stop = prediction.get_stop_loss();

        if direction == "long" {
            if current_price >= target {
                return "correct".into();
            }
            if current_price <= stop {
                return "incorrect".into();
            }
        } else {
            if current_price <= target {
                return "correct".into();
            }
            if current_price >= stop {
                return "incorrect".into();
            }
        }

        "pending".into()
    }
}
