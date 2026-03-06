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
            "Analyze the following cryptocurrency market data for the {} timeframe and provide trading signals.\n\n\
            === CURRENT TICKER DATA (24h stats, prices, volumes) ===\n{}\n\n\
            === RECENT CANDLESTICK DATA (OHLCV) ===\n{}\n\n\
            === LATEST NEWS & SENTIMENT ===\n{}\n\n\
            For each symbol, determine the strongest directional signal based on the data above. \
            Focus on actionable setups with clear entry, target, and stop-loss levels. \
            Explain your reasoning with specific references to the data provided.",
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
