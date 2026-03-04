use crate::domain::prediction::entities::Prediction;

pub struct AnalysisService;

impl AnalysisService {
    pub fn filter_by_confidence(
        predictions: &[Prediction],
        min_confidence: f64,
        max_confidence: f64,
    ) -> Vec<&Prediction> {
        predictions
            .iter()
            .filter(|p| p.get_confidence() >= min_confidence && p.get_confidence() <= max_confidence)
            .collect()
    }

    pub fn build_analysis_prompt(
        tickers_json: &str,
        klines_json: &str,
        news_json: &str,
    ) -> String {
        format!(
            "Analyze this crypto data and give predictions.\n\
            Tickers: {}\n\
            Candles (1h): {}\n\
            News: {}\n\
            Return JSON: {{\"predictions\":[{{\"symbol\":\"BTCUSDT\",\"direction\":\"long\",\"confidence\":75,\"reasoning\":\"brief\",\"entry_price\":100000,\"target_price\":101000,\"stop_loss\":99000}}]}}",
            tickers_json, klines_json, news_json
        )
    }

    pub fn determine_outcome(
        prediction: &Prediction,
        current_price: f64,
    ) -> String {
        let direction = prediction.get_direction();
        let entry = prediction.get_entry_price();
        let target = prediction.get_target_price();
        let stop = prediction.get_stop_loss();

        if direction == "long" {
            if current_price >= target {
                return String::from("correct");
            }
            if current_price <= stop {
                return String::from("incorrect");
            }
        } else {
            if current_price <= target {
                return String::from("correct");
            }
            if current_price >= stop {
                return String::from("incorrect");
            }
        }

        String::from("pending")
    }
}
