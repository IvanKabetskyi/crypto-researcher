use crate::domain::prediction::entities::Prediction;

pub struct AnalysisService;

impl AnalysisService {
    pub fn build_analysis_prompt(
        tickers_json: &str,
        klines_json: &str,
        news_json: &str,
        indicators_json: &str,
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
            === CANDLESTICK HISTORY (oldest to newest, OHLCV) ===\n\
            Fields: o=open, h=high, l=low, c=close, v=volume, t=timestamp(ms)\n\
            {}\n\n\
            === RECENT NEWS ===\n\
            {}\n\n\
            Use the pre-computed indicators as your primary analysis input. \
            Cross-reference with raw candle data only for pattern confirmation. \
            Follow the system instructions strictly.",
            timeframe, tickers_json, indicators_json, klines_json, news_json
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
