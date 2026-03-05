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
            "Analyze this crypto data and give predictions for timeframe: {}.\n\
            Tickers: {}\n\
            Candles: {}\n\
            News: {}\n\
            Return JSON: {{\"predictions\":[{{\"symbol\":\"BTCUSDT\",\"direction\":\"long\",\"confidence\":75,\"reasoning\":\"brief\",\"entry_price\":100000,\"target_price\":101000,\"stop_loss\":99000}}]}}",
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
