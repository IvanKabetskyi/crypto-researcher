use crate::domain::prediction::entities::Prediction;

pub struct AnalysisService;

impl AnalysisService {
    pub fn build_analysis_prompt(
        tickers_json: &str,
        klines_json: &str,
        news_json: &str,
        timeframe: &str,
    ) -> String {
        format!(
            "Timeframe: {}\n\n\
            === CURRENT PRICES (24h stats) ===\n\
            Fields: symbol, price (current), change_24h (percent as decimal, e.g. -0.02 = -2%), volume (24h USD), high (24h), low (24h)\n\
            {}\n\n\
            === CANDLESTICK HISTORY (oldest to newest, OHLCV) ===\n\
            Fields: o=open, h=high, l=low, c=close, v=volume, t=timestamp(ms)\n\
            {}\n\n\
            === RECENT NEWS ===\n\
            {}\n\n\
            Analyze each symbol's trend, momentum, support/resistance from the candle data. \
            Follow the system instructions strictly. Be conservative — accuracy matters more than conviction.",
            timeframe, tickers_json, klines_json, news_json
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
