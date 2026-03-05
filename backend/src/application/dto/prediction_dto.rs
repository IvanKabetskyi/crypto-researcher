use serde::Serialize;

use crate::domain::prediction::entities::Prediction;

#[derive(Debug, Serialize)]
pub struct PredictionDto {
    id: String,
    symbol: String,
    direction: String,
    confidence: f64,
    reasoning: String,
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    created_at: String,
    outcome: Option<String>,
    timeframe: Option<String>,
}

impl PredictionDto {
    pub fn transform_entity(prediction: Prediction) -> Self {
        Self {
            id: prediction.get_id().to_hex(),
            symbol: prediction.get_symbol().to_string(),
            direction: prediction.get_direction().to_string(),
            confidence: prediction.get_confidence(),
            reasoning: prediction.get_reasoning().to_string(),
            entry_price: prediction.get_entry_price(),
            target_price: prediction.get_target_price(),
            stop_loss: prediction.get_stop_loss(),
            created_at: prediction.get_created_at().to_rfc3339(),
            outcome: prediction.get_outcome().map(String::from),
            timeframe: prediction.get_timeframe().map(String::from),
        }
    }
}
