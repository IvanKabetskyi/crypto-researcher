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
            Fields: symbol, price (current), change_24h (percent as decimal, e.g. -0.02 = -2%), volume (24h USD), high (24h), low (24h)\n\
            {}\n\n\
            === PRE-COMPUTED TECHNICAL INDICATORS ===\n\
            These are computed from candle data. Use these as your PRIMARY signals.\n\
            {}\n\n\
            === DERIVATIVES & ORDER BOOK DATA ===\n\
            orderbook_ratio = bid_volume / ask_volume (>1.2 = bullish pressure, <0.8 = bearish pressure)\n\
            funding_rate = perpetual swap funding (negative = shorts pay longs = short squeeze risk, positive = longs pay shorts = long squeeze risk)\n\
            open_interest = total open derivative contracts\n\
            long_ratio / short_ratio = market positioning\n\
            {}\n\n\
            === CANDLESTICK HISTORY (oldest to newest, OHLCV) ===\n\
            Fields: o=open, h=high, l=low, c=close, v=volume, t=timestamp(ms)\n\
            {}\n\n\
            === RECENT NEWS ===\n\
            {}\n\n\
            Use technical indicators AND derivatives/order book data together. \
            Derivatives data reveals what large traders are doing — it often leads price. \
            Follow the system instructions strictly.",
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
